import { useState, useRef, useEffect } from "react";
import ReactAce from "react-ace";
import { Ace } from "ace-builds";
import "ace-builds/src-noconflict/mode-sqlserver";
import "ace-builds/src-noconflict/theme-sqlserver";
// import reactLogo from "./assets/react.svg";
// import viteLogo from "/vite.svg";
import "./App.css";
import RustDoc, { UserOperation } from "./components/RustDoc";
import { getWsUri } from "./utils";
import { gen_hello_string, OpSeq } from "ot-wasm";

window.OpSeq = OpSeq;

function App() {
  const [isActive, setIsActive] = useState(false);
  const rustDoc = useRef<RustDoc | null>(null);
  const timerRef = useRef<number>(0);
  const activeRef = useRef<boolean>(false);
  const [editor, setEditor] = useState<Ace.Editor | null>(null);
  const [value, setValue] = useState<string>("");

  // test wasm
  useEffect(() => {
    if (activeRef.current) return;
    console.log(gen_hello_string("rust_doc"));
    activeRef.current = true;
  }, []);

  useEffect(() => {
    timerRef.current = setInterval(() => {
      fetch("/api/json")
        .then((data) => data.json())
        .then((data) => {
          console.log(data);
        })
        .catch();
    }, 1000 * 1000);

    return () => {
      console.log("--- destoryed ---");
      clearInterval(timerRef.current);
      timerRef.current = 0;
    };
  }, []);

  useEffect(() => {
    if (editor) {
      rustDoc.current = new RustDoc({
        uri: getWsUri(),
        onConnected: () => {
          console.log("connected!!!");
        },
        onDisconnected: () => {
          console.log("disconnected!!!");
        },
        onPolling: (operation: UserOperation) => {
          console.log(operation);
        },
        editor,
      });
    }
  }, [editor]);

  return (
    <>
      <h1>Ace OT Editor</h1>
      <div className="card">
        <button onClick={() => setIsActive((active) => !active)}>
          Editor {`${!isActive ? 'Locked' : 'Opened'}`}
        </button>
      </div>
      <div className="editor-box">
        <ReactAce
          readOnly={!isActive}
          onLoad={(editor) => {
            setEditor(editor);
          }}
          name="query-ace-editor"
          theme="sqlserver"
          mode="sqlserver"
          value={value}
          onChange={(value) => {
            setValue(value);
          }}
        />
      </div>
    </>
  );
}

export default App;
