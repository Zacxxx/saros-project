export default function Home() {
  return (
    <div>
      <h1 className="text-3xl font-bold text-yellow-400 mb-2">Saros Devtools</h1>
      <p className="text-gray-400">Select a section from the sidebar to manage game content.</p>
      <div className="mt-8 grid grid-cols-3 gap-4">
        {["Blocks","Items","Recipes","NPCs","Quests","Game Rules","Textures"].map(s => (
          <a key={s} href={`/${s.toLowerCase().replace(" ","")}`} className="p-4 bg-gray-900 border border-gray-800 rounded hover:border-yellow-400 transition-colors text-gray-300 hover:text-white">
            {s}
          </a>
        ))}
      </div>
    </div>
  );
}
