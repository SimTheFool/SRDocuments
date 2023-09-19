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
  slots?: Slot[];
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
export type Outfit = BaseItem & {
  slots?: Slot[];
};
export type Weapon = BaseItem & {
  actions?: {
    attaquer?: ContactAction;
    recharger?: LoadAction;
    tir?: ShotAction;
    tir_rafale?: ShotAction;
    tir_semi_auto?: ShotAction;
  } & {
    [k: string]: BaseAction1;
  };
};

export interface Schema {
  actions?: {
    [k: string]: SpecificAction;
  };
  drones?: {
    [k: string]: Drone;
  };
  effects?: Effect[];
  identities?: Identities[];
  name?: string;
  outfits?: {
    [k: string]: Outfit;
  };
  skills: Skills;
  sprites?: {
    [k: string]: Companion;
  };
  stats: Stats;
  tech?: {
    [k: string]:
      | BaseItem
      | {
          name: "Persona incarné";
          stats: {
            attaque: number;
            corruption: number;
            firewall: number;
            traitement: number;
          };
        };
  };
  weapons?: {
    [k: string]: Weapon;
  };
}
export interface BaseAction {
  description?: string;
  maintained?: boolean;
  major: number;
  minor: number;
}
export interface BaseItem {
  concealment: number;
  description?: string;
  legal: boolean;
  licenced: boolean;
  manufacturer?: string;
  name: string;
  price: number;
  type?: string;
}
export interface Slot {
  concealment?: number;
  name: string;
  size: string;
}
export interface Effect {
  description?: string;
  name?: string;
  type?: string;
}
export interface Identities {
  contacts?: Contact[];
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
  athletisme?: Skill;
  combat_rapp?: Skill;
  electronique?: Skill;
  furtivite?: Skill;
  ingenierie?: Skill;
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
  cha: number;
  con: number;
  con_mod?: number;
  edge: number;
  ess: number;
  for: number;
  for_mod?: number;
  hit_over: number;
  hit_phy: number;
  hit_stun: number;
  init: number;
  initiation?: number;
  int: number;
  int_mod?: number;
  log: number;
  log_mod?: number;
  mag?: number;
  max_edge: number;
  natural_heal: number;
  rea: number;
  rea_mod?: number;
  res?: number;
  resist_drain?: number;
  resist_phy: number;
  submersion?: number;
  vol: number;
  vol_mod?: number;
}
export interface ContactAction {
  damage: number;
  damage_type: string;
  description?: string;
  major: number;
  minor: number;
  ranges: {
    contact: number;
  };
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
  ranges: {
    contact: number;
    far: number;
    medium: number;
    near: number;
    short: number;
  };
}
export interface BaseAction1 {
  description?: string;
  maintained?: boolean;
  major: number;
  minor: number;
}
