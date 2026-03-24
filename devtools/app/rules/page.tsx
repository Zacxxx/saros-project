"use client";
import { useEffect, useState } from "react";
import { get, put } from "@/lib/api";

interface Rule { id: string; label: string; type: string; value: number; min: number; max: number; }

export default function RulesPage() {
  const [rules, setRules] = useState<Rule[]>([]);
  const [saved, setSaved] = useState<string | null>(null);
  const [error, setError] = useState("");

  useEffect(() => { get<Rule[]>("/api/rules").then(setRules).catch(e => setError(e.message)); }, []);

  const update = async (id: string, value: number) => {
    setRules(r => r.map(rule => rule.id === id ? { ...rule, value } : rule));
    await put(`/api/rules/${id}`, { value }).catch(() => {});
    setSaved(id);
    setTimeout(() => setSaved(null), 1500);
  };

  if (error) return <p className="text-red-400">{error}</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">Game Rules</h1>
      <div className="flex flex-col gap-4 max-w-lg">
        {rules.map(r => (
          <div key={r.id} className="bg-gray-900 border border-gray-800 rounded p-4">
            <div className="flex justify-between items-center mb-2">
              <label className="text-white text-sm font-medium">{r.label}</label>
              <span className="text-yellow-400 text-sm font-mono">{r.value}{saved === r.id && <span className="text-green-400 ml-2 text-xs">saved</span>}</span>
            </div>
            <input
              type="range" min={r.min} max={r.max} step={r.type === "float" ? 0.05 : 1}
              value={r.value}
              onChange={e => update(r.id, +e.target.value)}
              className="w-full accent-yellow-400"
            />
            <div className="flex justify-between text-gray-600 text-xs mt-1">
              <span>{r.min}</span><span>{r.max}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
