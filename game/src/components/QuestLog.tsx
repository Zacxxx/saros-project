import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Objective { description: string; completed: boolean; }
interface Quest { id: number; title: string; giver: string; objectives: Objective[]; accepted: boolean; }

interface Props { onClose: () => void; }

export default function QuestLog({ onClose }: Props) {
  const [quests, setQuests] = useState<Quest[]>([]);

  useEffect(() => { invoke<Quest[]>("get_quests").then(setQuests); }, []);

  const accept = async (id: number) => {
    await invoke("accept_quest", { questId: id });
    setQuests(q => q.map(quest => quest.id === id ? { ...quest, accepted: true } : quest));
  };

  return (
    <div style={{
      position: "absolute", top: "50%", right: 16, transform: "translateY(-50%)",
      width: 300, background: "rgba(10,10,10,0.95)", border: "1px solid #444",
      padding: 16, pointerEvents: "all",
    }}>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 12 }}>
        <h3 style={{ color: "#e8c97a", margin: 0 }}>Quest Log</h3>
        <button onClick={onClose} style={{ background: "none", border: "none", color: "#888", cursor: "pointer" }}>✕</button>
      </div>
      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {quests.map(q => (
          <div key={q.id} style={{ padding: 8, background: "#1a1a1a", border: "1px solid #333" }}>
            <div style={{ color: "#ccc", fontWeight: "bold", fontSize: 13 }}>{q.title}</div>
            <div style={{ color: "#888", fontSize: 11, marginBottom: 4 }}>From: {q.giver}</div>
            {q.objectives.map((o, i) => (
              <div key={i} style={{ color: o.completed ? "#55dd88" : "#666", fontSize: 11 }}>
                {o.completed ? "✓" : "○"} {o.description}
              </div>
            ))}
            {!q.accepted && (
              <button onClick={() => accept(q.id)} style={{ marginTop: 6, padding: "2px 10px", background: "transparent", border: "1px solid #e8c97a", color: "#e8c97a", cursor: "pointer", fontSize: 11 }}>
                Accept
              </button>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
