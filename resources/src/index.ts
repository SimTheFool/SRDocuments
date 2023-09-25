import shrimp from "../assets/characters/_shrimp/shrimp.json";
import * as CharacterSchema from "../assets/characters/character";

export type Character = CharacterSchema.Schema;
export type Identity = CharacterSchema.Identity;
export type BaseItem = CharacterSchema.BaseItem;

export const characters = {
  shrimp: shrimp as Character,
};
