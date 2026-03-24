"use client";
import { useEffect, useState } from "react";
import { get, post, put } from "@/lib/api";

interface LootEntry {
    item_id: number;
    quantity_min: number;
    quantity_max: number;
    chance: number;
}

interface AiConfig {
    thought_interval: number;
    aggro_range?: number;
}

interface Mob {
    id: number;
    name: string;
    mob_type: string;
    can_talk: boolean;
    hp: number;
    max_hp: number;
    behavior: string;
    ai_config: AiConfig;
    loot_table: LootEntry[];
}

export default function MobsPage() {
    const [mobs, setMobs] = useState<Mob[]>([]);
    const [error, setError] = useState("");
    const [expandedId, setExpandedId] = useState<number | null>(null);

    useEffect(() => {
        get<Mob[]>("/api/mobs").then(setMobs).catch(e => setError(e.message));
    }, []);

    if (error) return <p className="text-red-400">{error}</p>;

    const typeColors: Record<string, string> = {
        Passive: "bg-green-900 text-green-300",
        Hostile: "bg-red-900 text-red-300",
        Neutral: "bg-yellow-900 text-yellow-300",
    };

    return (
        <div>
            <h1 className="text-2xl font-bold text-yellow-400 mb-6">Mobs</h1>
            <table className="w-full text-sm">
                <thead>
                    <tr className="text-gray-500 border-b border-gray-800">
                        <th className="text-left py-2 pr-4">ID</th>
                        <th className="text-left py-2 pr-4">Name</th>
                        <th className="text-left py-2 pr-4">Type</th>
                        <th className="text-left py-2 pr-4">Can Talk</th>
                        <th className="text-left py-2 pr-4">HP</th>
                        <th className="text-left py-2 pr-4">Behavior</th>
                        <th className="text-left py-2">Loot</th>
                    </tr>
                </thead>
                <tbody>
                    {mobs.map(m => (
                        <>
                            <tr
                                key={m.id}
                                className="border-b border-gray-900 hover:bg-gray-900 cursor-pointer"
                                onClick={() => setExpandedId(expandedId === m.id ? null : m.id)}
                            >
                                <td className="py-2 pr-4 text-gray-500">{m.id}</td>
                                <td className="py-2 pr-4 text-white font-medium">{m.name}</td>
                                <td className="py-2 pr-4">
                                    <span className={`text-xs px-2 py-0.5 rounded ${typeColors[m.mob_type] || "bg-gray-800 text-gray-400"}`}>
                                        {m.mob_type}
                                    </span>
                                </td>
                                <td className="py-2 pr-4 text-gray-400">{m.can_talk ? "✓ Yes" : "—"}</td>
                                <td className="py-2 pr-4 text-gray-400">{m.hp}/{m.max_hp}</td>
                                <td className="py-2 pr-4 text-gray-400">{m.behavior}</td>
                                <td className="py-2 text-gray-500">{m.loot_table.length} items</td>
                            </tr>
                            {expandedId === m.id && (
                                <tr key={`${m.id}-detail`} className="bg-gray-900/50">
                                    <td colSpan={7} className="p-4">
                                        <div className="grid grid-cols-2 gap-6">
                                            <div>
                                                <h3 className="text-xs uppercase text-gray-500 mb-2 tracking-wider">AI Configuration</h3>
                                                <div className="space-y-1 text-sm">
                                                    <div className="flex justify-between">
                                                        <span className="text-gray-400">Thought Interval</span>
                                                        <span className="text-white">{m.ai_config.thought_interval}s</span>
                                                    </div>
                                                    {m.ai_config.aggro_range && (
                                                        <div className="flex justify-between">
                                                            <span className="text-gray-400">Aggro Range</span>
                                                            <span className="text-white">{m.ai_config.aggro_range} blocks</span>
                                                        </div>
                                                    )}
                                                    <div className="flex justify-between">
                                                        <span className="text-gray-400">Uses AI Thoughts</span>
                                                        <span className="text-green-400">✓ Enabled</span>
                                                    </div>
                                                </div>
                                            </div>
                                            <div>
                                                <h3 className="text-xs uppercase text-gray-500 mb-2 tracking-wider">Loot Table</h3>
                                                {m.loot_table.length === 0 ? (
                                                    <p className="text-gray-600 text-sm italic">No loot drops</p>
                                                ) : (
                                                    <div className="space-y-1">
                                                        {m.loot_table.map((loot, i) => (
                                                            <div key={i} className="flex justify-between text-sm bg-gray-800 rounded px-2 py-1">
                                                                <span className="text-gray-300">Item #{loot.item_id}</span>
                                                                <span className="text-gray-400">
                                                                    {loot.quantity_min}-{loot.quantity_max} × {Math.round(loot.chance * 100)}%
                                                                </span>
                                                            </div>
                                                        ))}
                                                    </div>
                                                )}
                                            </div>
                                        </div>
                                    </td>
                                </tr>
                            )}
                        </>
                    ))}
                </tbody>
            </table>
        </div>
    );
}
