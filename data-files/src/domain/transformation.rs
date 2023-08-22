use super::character::Character;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Add(u8),
    CeilFrac(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Property {
    Constitution,
    Willpower,
    Strength,
    Magic,
}

#[derive(Debug)]
pub struct Transformation {
    pub property: Property,
    pub operation: Operation,
}
impl Transformation {
    pub fn transform(&self, c: Character) -> Character {
        let closure: Box<dyn Fn(u8) -> u8> = match self.operation {
            Operation::CeilFrac(divider) => {
                Box::new(move |nb: u8| f32::ceil(nb as f32 / divider as f32) as u8)
            }
            Operation::Add(addition) => Box::new(move |nb: u8| nb + addition),
        };

        match self.property {
            Property::Constitution => Character {
                constitution: closure(c.constitution),
                ..c
            },
            Property::Willpower => Character {
                willpower: closure(c.willpower),
                ..c
            },
            Property::Strength => Character {
                strength: closure(c.strength),
                ..c
            },
            Property::Magic => Character {
                magic: Some(closure(c.magic.unwrap_or(0))),
                ..c
            },
        }
    }
}
