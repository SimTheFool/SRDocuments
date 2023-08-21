use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct CharacterEffect {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct Character {
    pub constitution: u8,
    pub willpower: u8,
    pub strength: u8,
    pub magic: Option<u8>,
    pub effects: Vec<CharacterEffect>,
}
