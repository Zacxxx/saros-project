import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { WorldRecord } from "../types";
import styles from "./Form.module.css";

interface Props {
  onLoad: () => void;
}

export default function WorldList({ onLoad }: Props) {
  const [worlds, setWorlds] = useState<WorldRecord[]>([]);

  useEffect(() => {
    invoke<WorldRecord[]>("list_worlds").then(setWorlds);
  }, []);

  const handleLoad = async (id: string) => {
    await invoke("load_world", { id });
    onLoad();
  };

  if (worlds.length === 0) {
    return <p style={{ color: "#888", textAlign: "center", marginTop: "2rem" }}>No saved worlds.</p>;
  }

  return (
    <ul className={styles.worldList}>
      {worlds.map((w) => (
        <li key={w.id} className={styles.worldItem}>
          <span>{w.name}</span>
          <span style={{ color: "#888", fontSize: "0.8rem" }}>Seed: {w.seed}</span>
          <button onClick={() => handleLoad(w.id)}>Load</button>
        </li>
      ))}
    </ul>
  );
}
