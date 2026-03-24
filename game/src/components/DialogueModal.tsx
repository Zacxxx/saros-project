import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props { npcId: number; npcName: string; onClose: () => void; }

export default function DialogueModal({ npcId, npcName, onClose }: Props) {
  const [messages, setMessages] = useState<{ role: string; text: string }[]>([]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);

  const send = async () => {
    if (!input.trim() || loading) return;
    const userMsg = input.trim();
    setInput("");
    setMessages(m => [...m, { role: "user", text: userMsg }]);
    setLoading(true);
    try {
      const response = await invoke<string>("talk_to_npc", { npcId, message: userMsg });
      setMessages(m => [...m, { role: "npc", text: response }]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{
      position: "absolute", bottom: 80, left: "50%", transform: "translateX(-50%)",
      width: 480, background: "rgba(10,10,10,0.95)", border: "1px solid #444",
      padding: 16, pointerEvents: "all",
    }}>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 8 }}>
        <span style={{ color: "#e8c97a", fontWeight: "bold" }}>{npcName}</span>
        <button onClick={onClose} style={{ background: "none", border: "none", color: "#888", cursor: "pointer" }}>✕</button>
      </div>
      <div style={{ height: 160, overflowY: "auto", marginBottom: 8, display: "flex", flexDirection: "column", gap: 6 }}>
        {messages.map((m, i) => (
          <div key={i} style={{ textAlign: m.role === "user" ? "right" : "left" }}>
            <span style={{
              display: "inline-block", padding: "4px 10px", borderRadius: 4, fontSize: 13,
              background: m.role === "user" ? "#2a3a5a" : "#1a2a1a", color: "#ddd",
            }}>{m.text}</span>
          </div>
        ))}
        {loading && <div style={{ color: "#666", fontSize: 12 }}>...</div>}
      </div>
      <div style={{ display: "flex", gap: 8 }}>
        <input
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={e => e.key === "Enter" && send()}
          placeholder="Say something..."
          style={{ flex: 1, background: "#111", border: "1px solid #444", color: "white", padding: "4px 8px" }}
        />
        <button onClick={send} style={{ padding: "4px 12px", background: "#e8c97a", color: "#111", border: "none", cursor: "pointer" }}>
          Send
        </button>
      </div>
    </div>
  );
}
