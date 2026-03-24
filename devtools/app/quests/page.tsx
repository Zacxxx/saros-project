"use client";
import { useEffect, useState } from "react";
import { get } from "@/lib/api";

interface Objective { description: string; completed: boolean; }
interface Quest { id: number; title: string; giver: string; objectives: Objective[]; }

export default function QuestsPage() {
  const [quests, setQuests] = useState<Quest[]>([]);
  const [error, setError] = useState("");

  useEffect(() => { get<Quest[]>("/api/quests").then(setQuests).catch(e => setError(e.message)); }, []);

  if (error) return <p className="text-red-400">{error}</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">Quests</h1>
      <div className="flex flex-col gap-3">
        {quests.map(q => (
          <div key={q.id} className="bg-gray-900 border border-gray-800 rounded p-4">
            <div className="font-bold text-white">{q.title}</div>
            <div className="text-gray-500 text-xs mb-2">Given by: {q.giver}</div>
            <div className="flex flex-col gap-1">
              {q.objectives.map((o, i) => (
                <div key={i} className="text-sm text-gray-400">○ {o.description}</div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
