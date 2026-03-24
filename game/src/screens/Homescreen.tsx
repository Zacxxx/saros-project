import { useState } from "react";
import NewWorldForm from "../components/NewWorldForm";
import WorldList from "../components/WorldList";
import styles from "./Homescreen.module.css";

type Tab = "new" | "load" | "settings";

interface Props {
  onEnterGame: () => void;
}

export default function Homescreen({ onEnterGame }: Props) {
  const [tab, setTab] = useState<Tab>("new");

  return (
    <div className={styles.root} data-interactive>
      <h1 className={styles.title}>SAROS</h1>
      <div className={styles.tabs}>
        {(["new", "load", "settings"] as Tab[]).map((t) => (
          <button
            key={t}
            className={tab === t ? styles.activeTab : styles.tab}
            onClick={() => setTab(t)}
          >
            {t === "new" ? "New World" : t === "load" ? "Load World" : "Settings"}
          </button>
        ))}
      </div>
      <div className={styles.panel}>
        {tab === "new" && <NewWorldForm onStart={onEnterGame} />}
        {tab === "load" && <WorldList onLoad={onEnterGame} />}
        {tab === "settings" && <p className={styles.placeholder}>Settings coming soon.</p>}
      </div>
    </div>
  );
}
