use super::{character::CharacterEffect, transformation::Transformation};

#[derive(Debug, Default)]
pub struct CharacterDescription {
    pub constitution: u8,
    pub willpower: u8,
    pub strength: u8,
    pub magic: Option<u8>,
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
            .map(|spec| CharacterEffect {
                name: spec.name.clone(),
                description: spec.description.clone(),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct SpecializationDescription {
    pub name: String,
    pub description: String,
    pub transform: Vec<Transformation>,
}
