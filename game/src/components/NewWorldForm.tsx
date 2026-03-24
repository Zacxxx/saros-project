import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { WorldParams } from "../types";
import styles from "./Form.module.css";

interface Props {
  onStart: () => void;
}

const defaults: WorldParams = {
  name: "New World",
  seed: Math.floor(Math.random() * 999999),
  size: 256,
  sea_level: 32,
  cave_density: 0.4,
  structure_frequency: 0.3,
  biome_weights: { forest: 0.4, plains: 0.3, desert: 0.2, tundra: 0.1 },
};

export default function NewWorldForm({ onStart }: Props) {
  const [params, setParams] = useState<WorldParams>(defaults);

  const set = (key: keyof WorldParams, value: unknown) =>
    setParams((p) => ({ ...p, [key]: value }));

  const setBiome = (key: string, value: number) =>
    setParams((p) => ({ ...p, biome_weights: { ...p.biome_weights, [key]: value } }));

  const handleSubmit = async () => {
    await invoke("create_world", { params });
    onStart();
  };

  return (
    <div className={styles.form}>
      <label>Name
        <input value={params.name} onChange={(e) => set("name", e.target.value)} />
      </label>
      <label>Seed
        <input type="number" value={params.seed} onChange={(e) => set("seed", +e.target.value)} />
      </label>
      <label>World Size
        <input type="range" min={64} max={1024} step={64} value={params.size} onChange={(e) => set("size", +e.target.value)} />
        <span>{params.size}</span>
      </label>
      <label>Sea Level
        <input type="range" min={8} max={128} value={params.sea_level} onChange={(e) => set("sea_level", +e.target.value)} />
        <span>{params.sea_level}</span>
      </label>
      <label>Cave Density
        <input type="range" min={0} max={1} step={0.05} value={params.cave_density} onChange={(e) => set("cave_density", +e.target.value)} />
        <span>{params.cave_density.toFixed(2)}</span>
      </label>
      <label>Structure Frequency
        <input type="range" min={0} max={1} step={0.05} value={params.structure_frequency} onChange={(e) => set("structure_frequency", +e.target.value)} />
        <span>{params.structure_frequency.toFixed(2)}</span>
      </label>
      <fieldset>
        <legend>Biome Weights</legend>
        {Object.entries(params.biome_weights).map(([k, v]) => (
          <label key={k}>{k}
            <input type="range" min={0} max={1} step={0.05} value={v} onChange={(e) => setBiome(k, +e.target.value)} />
            <span>{v.toFixed(2)}</span>
          </label>
        ))}
      </fieldset>
      <button className={styles.primary} onClick={handleSubmit}>Generate World</button>
    </div>
  );
}
