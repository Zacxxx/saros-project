import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Ingredient { item_id: number; quantity: number; }
interface Recipe { id: number; name: string; inputs: Ingredient[]; output_item_id: number; output_quantity: number; }

interface Props { onClose: () => void; }

export default function CraftingScreen({ onClose }: Props) {
  const [recipes, setRecipes] = useState<Recipe[]>([]);
  const [msg, setMsg] = useState("");

  useEffect(() => { invoke<Recipe[]>("get_recipes").then(setRecipes); }, []);

  const craft = async (id: number) => {
    const ok = await invoke<boolean>("craft_item", { recipeId: id });
    setMsg(ok ? "Crafted!" : "Not enough materials.");
    setTimeout(() => setMsg(""), 2000);
  };

  return (
    <div style={{
      position: "absolute", top: "50%", left: "50%", transform: "translate(-50%,-50%)",
      background: "rgba(10,10,10,0.95)", border: "1px solid #444",
      padding: 24, minWidth: 320, pointerEvents: "all",
    }}>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 16 }}>
        <h2 style={{ color: "#e8c97a", margin: 0 }}>Crafting</h2>
        <button onClick={onClose} style={{ background: "none", border: "none", color: "#888", cursor: "pointer", fontSize: 18 }}>✕</button>
      </div>
      {msg && <div style={{ color: "#e8c97a", marginBottom: 8, fontSize: 13 }}>{msg}</div>}
      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {recipes.map(r => (
          <div key={r.id} style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "6px 8px", background: "#1a1a1a", border: "1px solid #333" }}>
            <div>
              <div style={{ color: "#ccc", fontSize: 13 }}>{r.name}</div>
              <div style={{ color: "#666", fontSize: 11 }}>
                {r.inputs.map(i => `item#${i.item_id}×${i.quantity}`).join(", ")} → item#${r.output_item_id}×${r.output_quantity}
              </div>
            </div>
            <button onClick={() => craft(r.id)} style={{ padding: "4px 12px", background: "transparent", border: "1px solid #e8c97a", color: "#e8c97a", cursor: "pointer" }}>
              Craft
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
