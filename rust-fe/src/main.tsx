// import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import init from "rust-wasm";
import './index.css'

init().then(() => {
  ReactDOM.createRoot(document.getElementById('root')!).render(
    // <React.StrictMode>
      <App />
    // </React.StrictMode>,
  )
});


