"use client";
import Link from "next/link";
import { usePathname } from "next/navigation";

const nav = [
  { href: "/", label: "Dashboard" },
  { href: "/blocks", label: "Blocks" },
  { href: "/items", label: "Items" },
  { href: "/recipes", label: "Recipes" },
  { href: "/npcs", label: "NPCs" },
  { href: "/mobs", label: "Mobs" },
  { href: "/quests", label: "Quests" },
  { href: "/rules", label: "Game Rules" },
  { href: "/textures", label: "Textures" },
];

export default function Sidebar() {
  const path = usePathname();
  return (
    <aside className="w-52 min-h-screen bg-gray-900 border-r border-gray-800 flex flex-col py-6 px-3 gap-1">
      <div className="text-yellow-400 font-bold text-lg px-3 mb-4 tracking-widest">SAROS</div>
      {nav.map(({ href, label }) => (
        <Link key={href} href={href} className={`px-3 py-2 rounded text-sm transition-colors ${path === href ? "bg-gray-700 text-white" : "text-gray-400 hover:text-white hover:bg-gray-800"}`}>
          {label}
        </Link>
      ))}
    </aside>
  );
}
