export interface BiomeWeights {
  forest: number;
  plains: number;
  desert: number;
  tundra: number;
}

export interface WorldParams {
  name: string;
  seed: number;
  size: number;
  sea_level: number;
  cave_density: number;
  structure_frequency: number;
  biome_weights: BiomeWeights;
}

export interface WorldRecord {
  id: string;
  name: string;
  seed: number;
  created_at: string;
}
