use std::fmt::Debug;

use super::descriptions::CharacterDescription;

#[derive(Debug, Default)]
pub struct Character {
    pub constitution: u8,
    pub willpower: u8,
    pub strength: u8,
    pub magic: Option<u8>,
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

        let character = Character {
            constitution: cd.constitution,
            willpower: cd.willpower,
            strength: cd.strength,
            magic: cd.magic,
            effects: vec![],
        };

        let character = transformations
            .iter()
            .fold(character, |acc, transf| transf.transform(acc));

        let character = effects.iter().fold(character, |mut acc, effect| {
            acc.effects.push(effect.clone());
            acc
        });

        character
    }
}
