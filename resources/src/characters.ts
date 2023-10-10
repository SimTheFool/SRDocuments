import shrimp from "../assets/characters/_shrimp/index.json";
import brigthorn from "../assets/characters/_brigthorn/index.json";
import { Character } from "./types";

export const characters = {
  shrimp,
  brigthorn,
} as Record<string, Character>;
