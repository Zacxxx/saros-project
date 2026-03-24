import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface CombatState { hp: number; max_hp: number; ap: number; max_ap: number; mp: number; max_mp: number; round: number; }

export default function TurnUI() {
  const [s, setS] = useState<CombatState | null>(null);

  useEffect(() => {
    const id = setInterval(async () => {
      const state = await invoke<CombatState>("get_combat_state");
      setS(state);
    }, 300);
    return () => clearInterval(id);
  }, []);

  if (!s) return null;

  return (
    <div style={{
      position: "absolute", top: 16, left: 16, pointerEvents: "all",
      background: "rgba(0,0,0,0.7)", border: "1px solid #555",
      padding: "8px 12px", display: "flex", flexDirection: "column", gap: 4, minWidth: 160,
    }}>
      <Bar label="HP" value={s.hp} max={s.max_hp} color="#e05555" />
      <Bar label="AP" value={s.ap} max={s.max_ap} color="#55aaff" />
      <Bar label="MP" value={s.mp} max={s.max_mp} color="#55dd88" />
      <div style={{ color: "#888", fontSize: 11, marginTop: 4 }}>Round {s.round} · Space = end turn</div>
    </div>
  );
}

function Bar({ label, value, max, color }: { label: string; value: number; max: number; color: string }) {
  return (
    <div style={{ display: "flex", alignItems: "center", gap: 6 }}>
      <span style={{ color: "#aaa", fontSize: 11, width: 20 }}>{label}</span>
      <div style={{ flex: 1, height: 8, background: "#222", borderRadius: 4, overflow: "hidden" }}>
        <div style={{ width: `${(value / max) * 100}%`, height: "100%", background: color, transition: "width 0.2s" }} />
      </div>
      <span style={{ color: "#ccc", fontSize: 10, width: 28, textAlign: "right" }}>{value}/{max}</span>
    </div>
  );
}
