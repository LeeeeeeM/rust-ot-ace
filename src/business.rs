use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use futures::prelude::*;

use anyhow::{Context, Result};
use log::{info, warn};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::{sync::Notify, time};
use warp::filters::ws::{Message, WebSocket};

#[derive(Default)]
pub struct RustDoc {
    state: RwLock<State>,
    count: AtomicU64,
    notify: Notify,
}

#[derive(Default)]
struct State {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerMessage {
    Identity(u64), // unique server id for client
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ClientMessage {
    Info {
        revision: usize,
    }
}

impl From<ServerMessage> for Message {
    fn from(msg: ServerMessage) -> Self {
        let serialized = serde_json::to_string(&msg).expect("failed serialize");
        Message::text(serialized)
    }
}

impl RustDoc {
    pub fn new() -> Self {
        let state = State {
            text: String::from("rust_doc"),
        };
        RustDoc {
            state: RwLock::new(state),
            count: AtomicU64::new(0),
            notify: Notify::new(),
        }
    }

    pub fn text(&self) -> String {
        let state = self.state.read();
        state.text.clone()
    }

    pub async fn on_connection(&self, socket: WebSocket) {
        let id = self.count.fetch_add(1, Ordering::Relaxed);
        info!("connection id={}", id);

        if let Err(e) = self.handle_connection(id, socket).await {
            warn!("connection error: {}", e);
        }

        info!("disconnection id={}", id);
    }

    async fn handle_connection(&self, id: u64, mut socket: WebSocket) -> Result<()> {
        // socket init message
        socket.send(ServerMessage::Identity(id).into()).await?;

        loop {
            let sleep = time::sleep(Duration::from_millis(500));
            tokio::pin!(sleep);
            tokio::select! {
                _ = sleep => {}
                _ = self.notify.notified() => {}
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
        let msg: ClientMessage = match message.to_str() {
            Ok(text) => serde_json::from_str(text).context("failed to des")?,
            Err(_) => {
                return Ok(());
            }
        };

        match msg {
            ClientMessage::Info { revision } => {
                println!("{}, {}", revision, id);
                self.notify.notify_waiters();
            }
        }
        Ok(())
    }
}
