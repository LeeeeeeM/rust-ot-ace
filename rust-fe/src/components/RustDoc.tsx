/** Options passed in to the RustDoc constructor. */
export type RustDocOptions = {
  readonly uri: string;
  readonly onConnected?: () => unknown;
  readonly onDisconnected?: () => unknown;
  readonly reconnectInterval?: number;
};

/** Browser client for RustDoc. */
class RustDoc {
  private ws?: WebSocket;
  private connecting?: boolean;
  private readonly intervalId: number;

  constructor(readonly options: RustDocOptions) {
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
      } else {
        this.connecting = false;
      }
    };
    ws.onmessage = ({ data }) => {
      if (typeof data === "string") {
        this.handleMessage(JSON.parse(data));
      }
    };
  }

  private handleMessage(msg: ServerMessage) {
    console.log(msg);
  }


}

interface ServerMessage {
  Identity?: number;
}

export default RustDoc;