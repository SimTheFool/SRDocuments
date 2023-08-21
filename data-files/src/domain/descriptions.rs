use super::character::Character;
use std::fmt::Debug;

pub struct CharacterTransformation {
    pub transform: Box<dyn Fn(Character) -> Character>,
}
impl Debug for CharacterTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CharacterTransformation")
            .field("transform", &"Box<dyn Fn(Character) -> Character>")
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct CharacterDescription {
    pub constitution: u8,
    pub willpower: u8,
    pub strength: u8,
    pub magic: Option<u8>,
    pub spec_descriptions: Vec<SpecializationDescription>,
}

impl CharacterDescription {
    pub fn apply_transformations(&self) -> Character {
        let character_transf: Vec<&CharacterTransformation> = self
            .spec_descriptions
            .iter()
            .map(|spec| &spec.transform)
            .collect();

        let character = Character {
            constitution: self.constitution,
            willpower: self.willpower,
            strength: self.strength,
            magic: self.magic,
            effects: vec![],
        };

        let character = character_transf
            .iter()
            .fold(character, |acc, transf| (transf.transform)(acc));

        character
    }
}

#[derive(Debug)]
pub struct SpecializationDescription {
    pub transform: CharacterTransformation,
}
