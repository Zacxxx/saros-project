import { useState } from "react";
import Homescreen from "./screens/Homescreen";
import GameHUD from "./screens/GameHUD";

export type Screen = "home" | "game";

export default function App() {
  const [screen, setScreen] = useState<Screen>("home");

  return (
    <div id="ui-overlay">
      {screen === "home" && <Homescreen onEnterGame={() => setScreen("game")} />}
      {screen === "game" && <GameHUD />}
    </div>
  );
}
