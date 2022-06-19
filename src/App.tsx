import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import { useInterval } from "usehooks-ts";
import "./App.css";

function App() {
  const [count, setCount] = useState(0);

  useInterval(async () => {
    setCount(await invoke("get_state"));
  }, 50);

  return (
    <div className="App">
      <header className="App-header">
        <p>Hello Tauri + Bevy!</p>
        <p>The count is: {count}</p>
      </header>
    </div>
  );
}

export default App;
