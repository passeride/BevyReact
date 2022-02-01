import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import * as test from "BevyTest";

function App() {
  const [wasm, setWasm] = useState(null);

  /* const loadWasm = async () => {
   *   setWasm(await import("BevyTest_bg.wasm"));
   * };
   * useEffect(() => {
   *   loadWasm();
   * }, []); */
  useEffect(() => {
    test.default();
  });

  return (
    <div className="App">
      {/* {wasm ? wasm.init() : null} */}
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <canvas id="game" className="game" width={1080} height={720} />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <button onClick={() => test.move_up()}>Up</button>
        <button onClick={() => test.move_down()}>Down</button>
        <button onClick={() => test.move_right()}>Right</button>
        <button onClick={() => test.move_left()}>Left</button>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
