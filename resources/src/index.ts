import shrimp from "../assets/characters/_shrimp/shrimp.json";
import * as CharacterSchema from "../assets/characters/character";

export type Character = CharacterSchema.Schema;

export const characters = {
  shrimp: shrimp as Character,
};
