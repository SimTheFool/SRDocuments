use std::fmt::Debug;

use super::descriptions::CharacterDescription;

type ModdedStat<T> = (T, Option<T>);

#[derive(Debug, Default)]
pub struct Character {
    pub name: String,
    pub description: String,

    pub con: ModdedStat<u8>,
    pub agi: ModdedStat<u8>,
    pub rea: ModdedStat<u8>,
    pub str: ModdedStat<u8>,
    pub wil: ModdedStat<u8>,
    pub log: ModdedStat<u8>,
    pub int: ModdedStat<u8>,
    pub cha: ModdedStat<u8>,

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

            con: (cd.con, None),
            agi: (cd.agi, None),
            rea: (cd.rea, None),
            str: (cd.str, None),
            wil: (cd.wil, None),
            log: (cd.log, None),
            int: (cd.int, None),
            cha: (cd.cha, None),

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
            natural_healing: modded.con.0 + modded.wil.0,
            physical_hit: 8 + f32::floor((modded.con.0 + modded.con.1.unwrap_or(0)) as f32 / 2.0)
                as u8,
            mental_hit: 8 + f32::floor((modded.wil.0 + modded.wil.1.unwrap_or(0)) as f32 / 2.0)
                as u8,
            resist_physical: f32::floor((modded.con.0 + modded.con.1.unwrap_or(0)) as f32 / 2.0)
                as u8,
            over_hit: modded.con.0,
            minor_actions: 1 + modded.init,
            ..modded
        };

        final_character
    }
}
