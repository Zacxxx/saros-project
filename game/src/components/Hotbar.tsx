import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface HotbarSlot { item_id: number; quantity: number; }
interface HotbarState { slots: HotbarSlot[]; active_slot: number; }

export default function Hotbar() {
  const [state, setState] = useState<HotbarState>({ slots: Array(9).fill({ item_id: 0, quantity: 0 }), active_slot: 0 });

  useEffect(() => {
    const id = setInterval(async () => {
      const s = await invoke<HotbarState>("get_hotbar");
      setState(s);
    }, 500);
    return () => clearInterval(id);
  }, []);

  return (
    <div style={{
      position: "absolute", bottom: 16, left: "50%", transform: "translateX(-50%)",
      display: "flex", gap: 4, pointerEvents: "all",
    }}>
      {state.slots.map((slot, i) => (
        <div key={i} style={{
          width: 48, height: 48,
          background: i === state.active_slot ? "rgba(232,201,122,0.3)" : "rgba(0,0,0,0.6)",
          border: `2px solid ${i === state.active_slot ? "#e8c97a" : "#555"}`,
          display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center",
          fontSize: 10, color: "#ccc",
        }}>
          {slot.item_id > 0 && <>
            <div style={{ width: 28, height: 28, background: `hsl(${slot.item_id * 40},60%,50%)`, borderRadius: 2 }} />
            <span>{slot.quantity}</span>
          </>}
        </div>
      ))}
    </div>
  );
}
