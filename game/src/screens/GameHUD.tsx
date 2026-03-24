import { useState } from "react";
import Hotbar from "../components/Hotbar";
import TurnUI from "../components/TurnUI";
import CraftingScreen from "../components/CraftingScreen";
import DialogueModal from "../components/DialogueModal";
import QuestLog from "../components/QuestLog";

export default function GameHUD() {
  const [showCrafting, setShowCrafting] = useState(false);
  const [showQuests, setShowQuests] = useState(false);
  const [dialogue, setDialogue] = useState<{ id: number; name: string } | null>(null);

  return (
    <div id="hud" style={{ position: "absolute", inset: 0, pointerEvents: "none" }}>
      <TurnUI />
      <Hotbar />

      {/* HUD buttons */}
      <div style={{ position: "absolute", bottom: 80, right: 16, display: "flex", flexDirection: "column", gap: 6, pointerEvents: "all" }}>
        <button onClick={() => setShowCrafting(v => !v)} style={btnStyle}>Craft</button>
        <button onClick={() => setShowQuests(v => !v)} style={btnStyle}>Quests</button>
        {/* Demo: talk to NPC 1 */}
        <button onClick={() => setDialogue({ id: 1, name: "Aldric" })} style={btnStyle}>Talk to Aldric</button>
      </div>

      {showCrafting && <CraftingScreen onClose={() => setShowCrafting(false)} />}
      {showQuests && <QuestLog onClose={() => setShowQuests(false)} />}
      {dialogue && <DialogueModal npcId={dialogue.id} npcName={dialogue.name} onClose={() => setDialogue(null)} />}
    </div>
  );
}

const btnStyle: React.CSSProperties = {
  padding: "6px 14px",
  background: "rgba(0,0,0,0.7)",
  border: "1px solid #555",
  color: "#ccc",
  cursor: "pointer",
};
