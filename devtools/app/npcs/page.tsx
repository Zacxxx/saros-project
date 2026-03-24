"use client";
import { useEffect, useState } from "react";
import { get } from "@/lib/api";

interface Npc { id: number; name: string; traits: string[]; x: number; y: number; z: number; }

export default function NpcsPage() {
  const [npcs, setNpcs] = useState<Npc[]>([]);
  const [error, setError] = useState("");

  useEffect(() => { get<Npc[]>("/api/npcs").then(setNpcs).catch(e => setError(e.message)); }, []);

  if (error) return <p className="text-red-400">{error}</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">NPCs</h1>
      <div className="flex flex-col gap-3">
        {npcs.map(n => (
          <div key={n.id} className="bg-gray-900 border border-gray-800 rounded p-4 flex items-start justify-between">
            <div>
              <div className="font-bold text-white">{n.name}</div>
              <div className="flex gap-2 mt-1">
                {n.traits.map(t => <span key={t} className="text-xs bg-gray-800 text-gray-400 px-2 py-0.5 rounded">{t}</span>)}
              </div>
            </div>
            <div className="text-gray-500 text-xs text-right">
              <div>x: {n.x}</div><div>y: {n.y}</div><div>z: {n.z}</div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
