"use client";
import { useEffect, useState, useRef } from "react";
import { get, post } from "@/lib/api";

interface Texture { name: string; url: string; assignedTo: string; }

export default function TexturesPage() {
  const [textures, setTextures] = useState<Texture[]>([]);
  const [loading, setLoading] = useState(true);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    get<Texture[]>("/api/textures")
      .then(setTextures)
      .finally(() => setLoading(false));
  }, []);

  const uploadFiles = (files: File[]) => {
    files.forEach(file => {
      const url = URL.createObjectURL(file);
      post<{ ok: boolean, data: Texture }>("/api/textures", { name: file.name, url })
        .then(res => {
          if (res.ok) {
            setTextures(t => [...t, res.data]);
          }
        })
        .catch(console.error);
    });
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    const files = Array.from(e.dataTransfer.files).filter(f => f.type.startsWith("image/"));
    uploadFiles(files);
  };

  const handleFiles = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(e.target.files ?? []);
    uploadFiles(files);
  };

  const assign = (name: string, target: string) => {
    setTextures(t => t.map(tx => tx.name === name ? { ...tx, assignedTo: target } : tx));
  };

  if (loading) return <p className="text-gray-500">Loading textures...</p>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-yellow-400 mb-6">Textures</h1>
      <div
        onDrop={handleDrop}
        onDragOver={e => e.preventDefault()}
        onClick={() => inputRef.current?.click()}
        className="border-2 border-dashed border-gray-700 rounded p-8 text-center text-gray-500 cursor-pointer hover:border-yellow-600 transition-colors mb-6"
      >
        Drop PNG/JPG files here or click to upload
        <input ref={inputRef} type="file" accept="image/*" multiple className="hidden" onChange={handleFiles} />
      </div>
      <div className="grid grid-cols-4 gap-4">
        {textures.map(tx => (
          <div key={tx.name} className="bg-gray-900 border border-gray-800 rounded overflow-hidden">
            <img src={tx.url} alt={tx.name} className="w-full h-24 object-cover" />
            <div className="p-2">
              <div className="text-xs text-gray-400 truncate mb-1">{tx.name}</div>
              <input
                value={tx.assignedTo}
                onChange={e => assign(tx.name, e.target.value)}
                placeholder="Assign to block/item..."
                className="w-full text-xs bg-gray-800 border border-gray-700 text-white px-2 py-1 rounded"
              />
            </div>
          </div>
        ))}
        {textures.length === 0 && (
          <div className="col-span-4 py-12 text-center text-gray-700">No textures uploaded yet</div>
        )}
      </div>
    </div>
  );
}
