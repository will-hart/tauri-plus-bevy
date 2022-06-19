import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    let unlisten: UnlistenFn | undefined = undefined;
    listen("send_state", (event) => {
      setCount(event.payload as number);
    }).then((r) => (unlisten = r));

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

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
