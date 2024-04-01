import { useState, useRef, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import RustDoc from "./components/RustDoc";
import { getWsUri } from "./utils";
import { gen_hello_string, OpSeq, Test } from "rust-wasm";

function App() {
  const [count, setCount] = useState(0);
  const rustDoc = useRef<RustDoc>();
  const timerRef = useRef<number>(0);
  const activeRef = useRef<boolean>(false);

  useEffect(() => {
    if (activeRef.current) return;
    console.log(gen_hello_string("rust_doc"));
    console.log(OpSeq);
    const a = new Test(123, "xxx");
    console.log(a, a.get_age(), a.get_name());
    activeRef.current = true;
  }, []);

  useEffect(() => {
    return;
    timerRef.current = setInterval(() => {
      fetch("/api/json")
        .then((data) => data.json())
        .then((data) => {
          console.log(data);
        })
        .catch();
    }, 1000);

    return () => {
      console.log("destory");
      clearInterval(timerRef.current);
      timerRef.current = 0;
    };
  }, []);

  useEffect(() => {
    if (rustDoc.current) return;
    rustDoc.current = new RustDoc({
      uri: getWsUri(),
      onConnected: () => {
        console.log("connected!!!");
      },
      onDisconnected: () => {
        console.log("disconnected!!!");
      },
    });
  }, []);

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;
