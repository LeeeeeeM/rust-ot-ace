import { OpSeq } from "rust-wasm";
import { Ace } from "ace-builds";

/** Options passed in to the RustDoc constructor. */
export type RustDocOptions = {
  readonly uri: string;
  readonly onConnected?: () => unknown;
  readonly onDisconnected?: () => unknown;
  readonly reconnectInterval?: number;
  readonly editor: Ace.Editor;
  readonly onPolling: (operation: UserOperation) => void;
  // readonly setValue: (value: string) => void;
};

type AceAction = "insert" | "remove";

interface AceEvent {
  action: AceAction;
  start: {
    column: number;
  };
  lines: string[];
}

/** Browser client for RustDoc. */
class RustDoc {
  private ws?: WebSocket;
  private connecting?: boolean;
  private readonly intervalId: number;

  private editor: Ace.Editor;
  private lastValue: string = "";
  private ignoreChanges: boolean = false;

  private lastActions: AceAction[] = [];

  private me: number = -1;
  private revision: number = 0;

  constructor(readonly options: RustDocOptions) {
    this.editor = options.editor;
    this.editor.on("change", (event: AceEvent) => {
      this.onChange(event);
    });
    this.tryConnect();
    this.intervalId = window.setInterval(
      () => this.tryConnect(),
      options.reconnectInterval ?? 1000
    );
  }

  /** Destroy this RustDoc instance and close any sockets. */
  dispose() {
    window.clearInterval(this.intervalId);
    this.ws?.close();
    // this.editor.destroy();
  }

  private tryConnect() {
    if (this.connecting || this.ws) return;
    this.connecting = true;
    const ws = new WebSocket(this.options.uri);
    ws.onopen = () => {
      this.connecting = false;
      this.ws = ws;
      this.options.onConnected?.();
    };
    ws.onclose = () => {
      if (this.ws) {
        this.ws = undefined;
        this.options.onDisconnected?.();

        this.dispose();
      } else {
        this.connecting = false;
      }
    };
    ws.onmessage = ({ data }) => {
      if (typeof data === "string") {
        this.handleMessage(JSON.parse(data));
      }
    };
    ws.onerror = () => {
      this.dispose();
    };
  }

  private handleMessage(msg: ServerMessage) {
    // console.log(msg, "message");
    if (msg.Identity !== undefined) {
      this.me = msg.Identity;
      return;
    }
    if (msg.History) {
      const { start, operations } = msg.History;
      if (start > this.revision) {
        console.warn("History message has start greater than last operation.");
        this.ws?.close();
        return;
      }
      for (let i = this.revision - start; i < operations.length; i++) {
        const { id, operation } = operations[i];
        this.revision++;
        this.options.onPolling(operations[i]);
        if (id === this.me) {
          console.log("this is me, ignore...");
        } else {
          const op: OpSeq = OpSeq.from_str(JSON.stringify(operation));
          this.applyOperation(op);
        }
      }
      return;
    }
  }

  private applyOperation(operation: OpSeq) {
    if (operation.is_noop()) return;

    this.ignoreChanges = true;

    // console.log("before change...");
    const lastContent = this.editor.getValue();
    // const ops: (string | number)[] = JSON.parse(operation.to_string());

    const result = operation.apply(lastContent) || "";

    this.lastValue = result;

    // reset value
    this.editor.setValue(result);
    this.ignoreChanges = false;
    // console.log("after change...");
  }

  public sendOperation(operation: OpSeq, action: AceAction) {
    this.lastActions.push(action);
    const [remove, insert] = this.lastActions;
    setTimeout(() => {
      this.lastActions = [];
    }, 20);
    // remove & insert at the same time，increase the revision
    if (remove === "remove" && insert === "insert") {
      // done
      // console.log('insert & remove')
      this.revision++;
    }

    const op = operation.to_string();
    // console.log(op, "operation");
    this.ws?.send(`{"Edit":{"revision":${this.revision},"operation":${op}}}`);
  }

  private onChange(event: AceEvent) {
    // console.log("changing ...");
    if (!this.ignoreChanges) {
      const content = this.lastValue;
      const contentLength = content.length;
      let operation = new OpSeq();
      operation.retain(contentLength);
      // 仅处理单行
      const { action, start, lines } = event;

      const initialLength = content.slice(0, start.column).length;

      let restLength = contentLength - initialLength;

      const changeOp = new OpSeq();

      changeOp.retain(initialLength);

      if (action === "insert") {
        changeOp.insert(lines[0]);
      }

      if (action === "remove") {
        const deletedLength = lines[0].length;
        changeOp.delete(deletedLength);
        restLength = restLength - deletedLength;
      }

      changeOp.retain(restLength);

      operation = operation.compose(changeOp)!;

      this.sendOperation(operation, action);
      this.lastValue = this.editor.getValue();
    }
  }
}

export interface UserOperation {
  id: number;
  operation: unknown;
}

interface ServerMessage {
  Identity?: number;
  History?: {
    start: number;
    operations: UserOperation[];
  };
}

export default RustDoc;
