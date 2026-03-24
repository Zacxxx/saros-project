"use client";
import { useEffect, useState } from "react";
import { get } from "@/lib/api";

interface Ingredient { item_id: number; quantity: number; }
interface Recipe { id: number; name: string; inputs: Ingredient[]; output_item_id: number; output_quantity: number; }

export default function RecipesPage() {
  const [recipes, setRecipes] = useState<Recipe[]>([]);
  const [error, setError] = useState("");

  useEffect(() => { get<Recipe[]>("/api/recipes").then(setRecipes).catch(e => setError(e.message)); }, []);

  if (error) return <p className="text-red-400">{error}</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">Recipes</h1>
      <div className="flex flex-col gap-4">
        {recipes.map(r => (
          <div key={r.id} className="bg-gray-900 border border-gray-800 rounded p-4">
            <div className="font-bold text-white mb-2">{r.name}</div>
            <div className="flex items-center gap-3 text-sm">
              <div className="flex gap-2">
                {r.inputs.map((inp, i) => (
                  <div key={i} className="bg-gray-800 border border-gray-700 px-3 py-2 rounded text-center">
                    <div className="text-gray-400 text-xs">item #{inp.item_id}</div>
                    <div className="text-white">×{inp.quantity}</div>
                  </div>
                ))}
              </div>
              <span className="text-gray-500 text-lg">→</span>
              <div className="bg-gray-800 border border-yellow-800 px-3 py-2 rounded text-center">
                <div className="text-gray-400 text-xs">item #{r.output_item_id}</div>
                <div className="text-yellow-400">×{r.output_quantity}</div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
