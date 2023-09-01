use super::{character::CharacterEffect, transformation::Transformation};

#[derive(Debug, Default)]
pub struct CharacterDescription {
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
    pub essence: u8,
    pub edge_rank: u8,
    pub specializations: Vec<SpecializationDescription>,
}

impl CharacterDescription {
    pub fn get_transformations(&self) -> Vec<&Transformation> {
        self.specializations
            .iter()
            .flat_map(|spec| &spec.transform)
            .collect()
    }

    pub fn get_effects(&self) -> Vec<CharacterEffect> {
        self.specializations
            .iter()
            .filter_map(|spec| {
                if spec.name.is_none() && spec.description.is_none() {
                    return None;
                }

                Some(CharacterEffect {
                    name: spec.name.clone().unwrap_or("".to_string()),
                    description: spec.description.clone().unwrap_or("".to_string()),
                })
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct SpecializationDescription {
    pub name: Option<String>,
    pub description: Option<String>,
    pub transform: Vec<Transformation>,
}
