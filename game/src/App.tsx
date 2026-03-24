import { useState } from "react";
import Homescreen from "./screens/Homescreen";
import GameHUD from "./screens/GameHUD";
import WorldGenerationScreen from "./screens/WorldGenerationScreen";

export type Screen = "home" | "generating" | "game";

export default function App() {
  const [screen, setScreen] = useState<Screen>("home");
  const [worldId, setWorldId] = useState("");

  const handleStartGeneration = (id: string) => {
    setWorldId(id);
    setScreen("generating");
  };

  return (
    <div id="ui-overlay">
      {screen === "home" && <Homescreen onEnterGame={handleStartGeneration} />}
      {screen === "generating" && (
        <WorldGenerationScreen
          worldId={worldId}
          onComplete={() => setScreen("game")}
        />
      )}
      {screen === "game" && <GameHUD />}
    </div>
  );
}
