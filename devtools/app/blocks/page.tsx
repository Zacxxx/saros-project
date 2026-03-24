"use client";
import { useEffect, useState } from "react";
import { get, put } from "@/lib/api";

interface Block {
  id: number;
  name: string;
  texture_id: string;
  is_solid: boolean;
  is_transparent: boolean;
  hardness: number;
}

interface Texture {
  name: string;
  url: string;
}

export default function BlocksPage() {
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [textures, setTextures] = useState<Texture[]>([]);
  const [error, setError] = useState("");

  useEffect(() => {
    Promise.all([
      get<Block[]>("/api/blocks"),
      get<Texture[]>("/api/textures")
    ]).then(([b, t]) => {
      setBlocks(b);
      setTextures(t);
    }).catch(e => setError(e.message));
  }, []);

  const updateTexture = (block: Block, textureId: string) => {
    const updatedBlock = { ...block, texture_id: textureId };
    put<{ ok: boolean, data: Block }>(`/api/blocks/${block.id}`, updatedBlock)
      .then(res => {
        if (res.ok) {
          setBlocks(prev => prev.map(b => b.id === block.id ? res.data : b));
        }
      })
      .catch(e => setError(e.message));
  };

  if (error) return <p className="text-red-400">{error}</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">Blocks</h1>
      <table className="w-full text-sm">
        <thead><tr className="text-gray-500 border-b border-gray-800">
          <th className="text-left py-2 pr-4">ID</th>
          <th className="text-left py-2 pr-4">Name</th>
          <th className="text-left py-2 pr-4">Texture</th>
          <th className="text-left py-2 pr-4">Solid</th>
          <th className="text-left py-2 pr-4">Transparent</th>
          <th className="text-left py-2">Hardness</th>
        </tr></thead>
        <tbody>
          {blocks.map(b => (
            <tr key={b.id} className="border-b border-gray-900 hover:bg-gray-900">
              <td className="py-2 pr-4 text-gray-500">{b.id}</td>
              <td className="py-2 pr-4 text-white">{b.name}</td>
              <td className="py-2 pr-4">
                <select
                  value={b.texture_id}
                  onChange={e => updateTexture(b, e.target.value)}
                  className="bg-gray-800 border border-gray-700 text-white rounded px-2 py-1 text-xs focus:ring-1 focus:ring-yellow-500 outline-none"
                >
                  <option value="">None</option>
                  {textures.map(t => (
                    <option key={t.name} value={t.name}>{t.name}</option>
                  ))}
                </select>
              </td>
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
