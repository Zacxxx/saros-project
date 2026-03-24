"use client";
import { useEffect, useState } from "react";
import { get } from "@/lib/api";

interface Block { id: number; name: string; is_solid: boolean; is_transparent: boolean; hardness: number; }

export default function BlocksPage() {
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [error, setError] = useState("");

  useEffect(() => { get<Block[]>("/api/blocks").then(setBlocks).catch(e => setError(e.message)); }, []);

  if (error) return <p className="text-red-400">{error}</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">Blocks</h1>
      <table className="w-full text-sm">
        <thead><tr className="text-gray-500 border-b border-gray-800">
          <th className="text-left py-2 pr-4">ID</th>
          <th className="text-left py-2 pr-4">Name</th>
          <th className="text-left py-2 pr-4">Solid</th>
          <th className="text-left py-2 pr-4">Transparent</th>
          <th className="text-left py-2">Hardness</th>
        </tr></thead>
        <tbody>
          {blocks.map(b => (
            <tr key={b.id} className="border-b border-gray-900 hover:bg-gray-900">
              <td className="py-2 pr-4 text-gray-500">{b.id}</td>
              <td className="py-2 pr-4 text-white">{b.name}</td>
              <td className="py-2 pr-4 text-gray-400">{b.is_solid ? "✓" : "—"}</td>
              <td className="py-2 pr-4 text-gray-400">{b.is_transparent ? "✓" : "—"}</td>
              <td className="py-2 text-gray-400">{b.hardness}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
