"use client";
import { useEffect, useState } from "react";
import { get } from "@/lib/api";

interface Item { id: number; name: string; description: string; stack_size: number; item_type: string; }

export default function ItemsPage() {
  const [items, setItems] = useState<Item[]>([]);
  const [error, setError] = useState("");

  useEffect(() => { get<Item[]>("/api/items").then(setItems).catch(e => setError(e.message)); }, []);

  if (error) return <p className="text-red-400">{error}</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">Items</h1>
      <table className="w-full text-sm">
        <thead><tr className="text-gray-500 border-b border-gray-800">
          <th className="text-left py-2 pr-4">ID</th>
          <th className="text-left py-2 pr-4">Name</th>
          <th className="text-left py-2 pr-4">Type</th>
          <th className="text-left py-2 pr-4">Stack</th>
          <th className="text-left py-2">Description</th>
        </tr></thead>
        <tbody>
          {items.map(i => (
            <tr key={i.id} className="border-b border-gray-900 hover:bg-gray-900">
              <td className="py-2 pr-4 text-gray-500">{i.id}</td>
              <td className="py-2 pr-4 text-white">{i.name}</td>
              <td className="py-2 pr-4 text-gray-400">{i.item_type}</td>
              <td className="py-2 pr-4 text-gray-400">{i.stack_size}</td>
              <td className="py-2 text-gray-500">{i.description}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
