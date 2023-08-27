use std::fmt::Debug;

use super::descriptions::CharacterDescription;

#[derive(Debug, Default)]
pub struct Character {
    pub name: String,
    pub description: String,

    pub con: u8,
    pub agi: u8,
    pub rea: u8,
    pub str: u8,
    pub wil: u8,
    pub log: u8,
    pub int: u8,
    pub cha: u8,

    pub con_mod: Option<u8>,
    pub agi_mod: Option<u8>,
    pub rea_mod: Option<u8>,
    pub str_mod: Option<u8>,
    pub wil_mod: Option<u8>,
    pub log_mod: Option<u8>,
    pub int_mod: Option<u8>,
    pub cha_mod: Option<u8>,

    pub ess: u8,
    pub edge: u8,
    pub init: u8,
    pub minor_actions: u8,

    pub natural_healing: u8,
    pub physical_hit: u8,
    pub mental_hit: u8,
    pub over_hit: u8,
    pub resist_physical: u8,

    pub magic: Option<u8>,
    pub resonance: Option<u8>,
    pub initiative: Option<u8>,
    pub submersion: Option<u8>,
    pub resist_drain: Option<u8>,

    pub effects: Vec<CharacterEffect>,
}

#[derive(Debug, Clone)]
pub struct CharacterEffect {
    pub name: String,
    pub description: String,
}

impl From<CharacterDescription> for Character {
    fn from(cd: CharacterDescription) -> Self {
        let transformations = cd.get_transformations();
        let effects = cd.get_effects();

        let unmodded = Character {
            name: cd.name.clone(),
            description: cd.description.clone(),

            con: cd.con,
            agi: cd.agi,
            rea: cd.rea,
            str: cd.str,
            wil: cd.wil,
            log: cd.log,
            int: cd.int,
            cha: cd.cha,

            ess: 6,
            edge: cd.edge_rank,
            init: 1,

            effects,

            ..Default::default()
        };

        let modded = transformations
            .iter()
            .fold(unmodded, |acc, transf| transf.transform(acc));

        let final_character = Character {
            natural_healing: modded.con
                + modded.con_mod.unwrap_or(0)
                + modded.wil
                + modded.wil_mod.unwrap_or(0),

            physical_hit: 8 + f32::floor((modded.con + modded.con_mod.unwrap_or(0)) as f32 / 2.0)
                as u8,
            mental_hit: 8 + f32::floor((modded.wil + modded.wil_mod.unwrap_or(0)) as f32 / 2.0)
                as u8,
            resist_physical: f32::floor((modded.con + modded.con_mod.unwrap_or(0)) as f32 / 3.0)
                as u8,
            over_hit: modded.con,
            minor_actions: 1 + modded.init,
            ..modded
        };

        final_character
    }
}
