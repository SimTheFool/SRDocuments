import {
  Schema as Character,
  RangeLabels,
  RangeScores,
} from "../assets/characters/character";
import shrimp from "../assets/characters/shrimp.json";

export const characters = {
  shrimp,
} as Record<string, Character>;

export const getSortedNumberScoresPair = (
  distanceByNb: RangeLabels,
  scoresByDistance: RangeScores
) => {
  const numberScorePair = Object.entries(distanceByNb).map(
    ([key, distance]) => {
      const k = key.replace("r", "");
      const nb = parseInt(k);
      const score = scoresByDistance[distance];
      return [nb, score];
    }
  );

  const sortedNumberScorePair = numberScorePair.sort((a, b) => a[0] - b[0]);
  const scores = sortedNumberScorePair.map((pair) => pair[1]);
  const numbers = sortedNumberScorePair.map((pair) => pair[0]);

  return {
    scores,
    numbers,
  };
};

export type * from "../assets/characters/character";
export type { Schema as Character } from "../assets/characters/character";
