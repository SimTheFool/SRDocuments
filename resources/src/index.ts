import shrimp from "../assets/characters/_shrimp/shrimp.json";
import type { Schema as Character } from "../assets/characters/character";

export type * from "../assets/characters/character";
export type { Schema as Character } from "../assets/characters/character";

export const characters = {
  shrimp: shrimp as Character,
};
