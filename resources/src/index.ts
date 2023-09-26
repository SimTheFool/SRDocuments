import shrimp from "../assets/characters/_shrimp/shrimp.json";
import type {
  Schema as Character,
  RangeLabels,
  RangeScores,
} from "../assets/characters/character";

export type * from "../assets/characters/character";
export type { Schema as Character } from "../assets/characters/character";

export const characters = {
  shrimp: shrimp as Character,
};

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
