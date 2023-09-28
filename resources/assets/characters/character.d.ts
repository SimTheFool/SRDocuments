/* eslint-disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type SpecificAction = BaseAction & {
  type: "technomancy";
};
export type Drone = BaseItem & {
  hit: number;
  stats: {
    acceleration: number;
    armor: number;
    autopilot: number;
    maniability_flat: number;
    maniability_rough: number;
    max_speed: number;
    resistance: number;
    sensors: number;
    step: number;
  };
};
export type Outfit = BaseItem;
export type Tech = BaseItem & {
  stats?: {
    attaque: number;
    corruption: number;
    firewall: number;
    traitement: number;
  };
};
export type Weapon = BaseItem & {
  actions?: {
    recharger?: LoadAction;
    tir?: ShotAction;
    tir_rafale?: ShotAction;
    tir_semi_auto?: ShotAction;
  } & {
    [k: string]: BaseAction1;
  };
  ammo?: number;
  damage?: number;
  damage_type?: string;
  range_labels?: RangeLabels;
  ranges?: RangeScores;
};
/**
 * This interface was referenced by `RangeLabels`'s JSON-Schema definition
 * via the `patternProperty` "r[0-9]*".
 */
export type RangeEnum = "contact" | "near" | "short" | "medium" | "far";

export interface Schema {
  actions?: {
    [k: string]: SpecificAction;
  };
  drones?: {
    [k: string]: Drone;
  };
  effects?: Effect[];
  identities?: Identity[];
  knowledges?: string[];
  name?: string;
  outfits?: {
    [k: string]: Outfit;
  };
  skills: Skills;
  sprites?: {
    [k: string]: Companion;
  };
  stats: Stats;
  tags?: string[];
  tech?: {
    [k: string]: Tech;
  };
  weapons?: {
    [k: string]: Weapon;
  };
}
export interface BaseAction {
  damage?: number;
  damage_type?: string;
  description?: string;
  gauge?: number;
  maintained?: boolean;
  major: number;
  minor: number;
  score?: number;
}
export interface BaseItem {
  concealment?: number;
  description?: string;
  legal: boolean;
  licenced: boolean;
  manufacturer?: string;
  name: string;
  price: number;
  quantity?: number;
  slots?: Slot[];
  type?: string;
}
export interface Slot {
  concealment?: number;
  name: string;
  size: "S" | "M" | "L" | "XL";
}
export interface Effect {
  description?: string;
  name?: string;
  type?: string;
}
export interface Identity {
  contacts?: Contact[];
  description?: string;
  licences?: Licences[];
  lifestyle?: Lifestyle;
  name?: string;
  nuyens?: number;
  price?: number;
  quality?: string;
}
export interface Contact {
  connection: number;
  description?: string;
  loyalty: number;
  name: string;
}
export interface Licences {
  name: string;
  price: number;
  quality: string;
}
export interface Lifestyle {
  name: string;
  price: number;
}
export interface Skills {
  athlétisme?: Skill;
  "combat.rapp"?: Skill;
  electronique?: Skill;
  furtivité?: Skill;
  ingénierie?: Skill;
  perception?: Skill;
  pilotage?: Skill;
  technomancie?: Skill;
}
export interface Skill {
  base: number;
  expertises?: string[];
  specialisations?: string[];
}
export interface Companion {
  actions?: BaseAction[];
  effects?: Effect;
  skills?: ("Ingenierie" | "Electronique")[];
  stats?: {
    [k: string]: unknown;
  };
}
export interface Stats {
  action_maj: number;
  action_min: number;
  agi: number;
  agi_mod?: number;
  algo?: ReferStat;
  cha: number;
  cha_mod?: number;
  con: number;
  con_mod?: number;
  def_ment: ReferStat;
  def_phy: ReferStat;
  edge: number;
  ess: number;
  for: number;
  for_mod?: number;
  hit_over: number;
  hit_phy: number;
  hit_stun: number;
  init: ReferStat;
  initiation?: number;
  int: number;
  int_mod?: number;
  log: number;
  log_mod?: number;
  mag?: number;
  max_edge: number;
  natural_heal: ReferStat;
  rea: number;
  rea_mod?: number;
  res?: number;
  resist_ment: ReferStat;
  resist_phy: ReferStat;
  submersion?: number;
  trad?: ReferStat;
  vol: number;
  vol_mod?: number;
}
export interface ReferStat {
  score: number;
  stat: ("con" | "agi" | "rea" | "for" | "vol" | "log" | "int" | "cha")[];
}
export interface LoadAction {
  ammo: number;
  description?: string;
  major: number;
  minor: number;
}
export interface ShotAction {
  ammo_consumption: number;
  damage: number;
  damage_type: string;
  description?: string;
  major: number;
  minor: number;
  ranges: RangeScores;
}
export interface RangeScores {
  /**
   * This interface was referenced by `RangeScores`'s JSON-Schema definition
   * via the `patternProperty` ".*".
   */
  [k: string]: number;
}
export interface BaseAction1 {
  damage?: number;
  damage_type?: string;
  description?: string;
  gauge?: number;
  maintained?: boolean;
  major: number;
  minor: number;
  score?: number;
}
export interface RangeLabels {
  [k: string]: RangeEnum;
}
