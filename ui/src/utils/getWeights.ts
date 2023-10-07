import { Character, Companion } from "resources";

export const getCharWeights = (char: Character) => {
  const powers =
    Object.values(char.spells || {}).length +
    Object.values(char.rituals || {}).length +
    Object.values(char.complex_forms || {}).length;

  const companions =
    Object.values(char.other_companions || {}).reduce(
      (acc, c) => getCompanionWeight(c) + acc,
      0
    ) +
    Object.values(char.spirits || {}).reduce(
      (acc, c) => getCompanionWeight(c) + acc,
      0
    ) +
    Object.values(char.sprites || {}).reduce(
      (acc, c) => getCompanionWeight(c) + acc,
      0
    );

  return {
    powers,
    companions,
  };
};

const getCompanionWeight = (companion: Companion) => {
  return (
    (companion.effects || []).length +
    Object.values(companion.actions || {}).length +
    (companion.description ? 1 : 0) +
    1
  );
};
