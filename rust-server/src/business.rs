use std::sync::atomic::{AtomicU64, Ordering};

use futures::prelude::*;

use anyhow::{Context, Result, bail};
use log::{info, warn};
use operational_transform::OperationSeq;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Notify};
use warp::filters::ws::{Message, WebSocket};

pub struct RustDoc {
    state: RwLock<State>,
    count: AtomicU64,
    // inner notify
    notify: Notify,
    // broadcaster: broadcast::Sender<ServerMessage>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    text: String,
    count: u64,
    operations: Vec<UserOperation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserOperation {
    id: u64,
    operation: OperationSeq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerMessage {
    Identity(u64), // unique server id for client
    History {
        start: usize,
        operations: Vec<UserOperation>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ClientMessage {
    Edit {
        operation: OperationSeq,
        revision: usize,
    },
}

impl From<ServerMessage> for Message {
    fn from(msg: ServerMessage) -> Self {
        let serialized = serde_json::to_string(&msg).expect("failed serialize");
        Message::text(serialized)
    }
}

impl RustDoc {
    pub fn new() -> Self {
        // let (tx, _) = broadcast::channel(16);
        // let state = State {
        //     text: String::from("rust_doc"),
        //     count: 0,
        //     operations: vec![],
        // };
        let state = Default::default();
        RustDoc {
            state: RwLock::new(state),
            count: AtomicU64::new(0),
            notify: Notify::new(),
            // broadcaster: tx,
        }
    }

    pub fn text(&self) -> String {
        let state = self.state.read();
        state.text.clone()
    }

    pub fn json(&self) -> State {
        let mut state = self.state.write();
        (*state).count += 1;
        // let state = self.state.read();
        State {
            count: state.count,
            text: state.text.clone(),
            operations: state.operations.clone(),
        }
    }

    pub async fn on_connection(&self, socket: WebSocket) {
        let id = self.count.fetch_add(1, Ordering::Relaxed);
        info!("connection id={}", id);

        if let Err(e) = self.handle_connection(id, socket).await {
            warn!("connection error: {}", e);
        }

        info!("disconnection id={}", id);
    }

    async fn send_initial(&self, id: u64, socket: &mut WebSocket) -> Result<usize> {
        socket.send(ServerMessage::Identity(id).into()).await?;
        let mut messages = Vec::new();

        let revision = {
            let state = self.state.read();

            if !state.operations.is_empty() {
                messages.push(ServerMessage::History {
                    start: 0,
                    operations: state.operations.clone(),
                });
            }
            state.operations.len()
        };

        for msg in messages {
            socket.send(msg.into()).await?;
        }
        // println!("------- revision: {} -------", revision);
        Ok(revision)
    }

    // current revision
    fn revision(&self) -> usize {
        let state = self.state.read();
        state.operations.len()
    }

    // send history
    async fn send_history(&self, start: usize, socket: &mut WebSocket) -> Result<usize> {
        let operations = {
            let state = self.state.read();
            let len = state.operations.len();
            if start < len {
                state.operations[start..].to_owned()
            } else {
                Vec::new()
            }
        };
        let ops_count = operations.len();

        if ops_count > 0 {
            let msg = ServerMessage::History { start, operations };
            socket.send(msg.into()).await?;
        }

        Ok(start + ops_count)
    }

    async fn handle_connection(&self, id: u64, mut socket: WebSocket) -> Result<()> {
        // socket init message
        // let mut rx = self.broadcaster.subscribe();

        let mut revision: usize = self.send_initial(id, &mut socket).await?;

        loop {
            let notified = self.notify.notified();
            // update revision & send
            if self.revision() > revision {
                revision = self.send_history(revision, &mut socket).await?;
            }

            tokio::select! {
                _ = notified => {}
                // update = rx.recv() => {
                //     let info = update?.into();
                //     socket.send(info).await?;
                // }
                result = socket.next() => {
                    match result {
                        None => break,
                        Some(message) => {
                            self.handle_message(id, message?).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, id: u64, message: Message) -> Result<()> {
        // println!("{:?}", message.to_str());
        let msg: ClientMessage = match message.to_str() {
            Ok(text) => serde_json::from_str(text).context("failed to deserialize message")?,
            Err(()) => return Ok(()), // Ignore non-text messages
        };

        match msg {
            ClientMessage::Edit {
                operation,
                revision,
            } => {
                self.apply_edit(id, revision, operation)
                    .context("invalid edit operation")?;
                self.notify.notify_waiters();
            }
        }
        Ok(())
    }

    fn apply_edit(&self, id: u64, revision: usize, mut operation: OperationSeq) -> Result<()> {
        let state = self.state.upgradable_read();
        let len = state.operations.len();
        if revision > len {
            bail!("got revision {}, but current is {}", revision, len);
        }
        // println!("Text: {}, Operation: {:?}, revision: {}", state.text, operation, revision);
        for history_op in &state.operations[revision..] {
            operation = operation.transform(&history_op.operation)?.0;
        }
        let new_text = operation.apply(&state.text)?;
        let mut state = RwLockUpgradableReadGuard::upgrade(state);
        state.operations.push(UserOperation { id, operation });
        state.text = new_text;
        Ok(())
    }
}
