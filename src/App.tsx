import { ChangeEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { open } from '@tauri-apps/api/dialog';

function App() {
  const [path, setPath] = useState("");
  const [rows, setRows] = useState("");
  const [message, setMessage] = useState("");

  async function split() {
    const parsed = parseInt(rows);
    const rowN = !Number.isNaN(parsed) ? parsed : 0;
    console.log(path);
    await invoke("split", { file: path, rowN }).then((message) => { typeof message == 'string' ? setMessage(message) : setMessage("failed") });
    console.log(message);
  }


  const handleFiles = async () => {
    const selected = await open();
    console.log(selected);
    if (typeof selected == "string") {
      setPath(selected)
    }
  }

  return (
    <div className="container">
      <h1>CSV Splitter</h1>

      <p>分割したいCSVの行数を指定して「分割」するボタンを押下してください。</p>

      <div className="row">
        <div>
          <button type="button" onClick={() => handleFiles()}>
            {path ? path.split("/").at(-1) : "ファイルを選ぶ"}
          </button>
          <input
            id="greet-input"
            type="number"
            onChange={(e) => setRows(e.currentTarget.value)}
            placeholder="分割したい行数を入力してください"
          />
          <button type="button" onClick={() => split()}>
            分割する
          </button>
          <div style={{ marginTop: "20px", fontWeight: "bold" }}>{message}</div>
        </div>
      </div>
    </div>
  );
}

export default App;
