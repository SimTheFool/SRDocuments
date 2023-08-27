use super::character::Character;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Inc(u8),
    CeilFrac(Property, u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Property {
    Magic,
    Constitution,
    Agility,
    Reaction,
    Strength,
    Willpower,
    Logic,
    Intuition,
    Charisma,
    ResistDrain,
}
impl Property {
    pub fn get(&self, c: &Character) -> u8 {
        match self {
            Self::Magic => c.magic.unwrap_or(0),
            Self::Constitution => c.con,
            Self::Agility => c.agi,
            Self::Reaction => c.rea,
            Self::Strength => c.str,
            Self::Willpower => c.wil,
            Self::Logic => c.log,
            Self::Intuition => c.int,
            Self::Charisma => c.cha,
            Self::ResistDrain => c.resist_drain.unwrap_or(0),
        }
    }
    pub fn set(&self, mut c: Character, val: u8) -> Character {
        match self {
            Self::Magic => c.magic = Some(val),
            Self::Constitution => c.con = val,
            Self::Agility => c.agi = val,
            Self::Reaction => c.rea = val,
            Self::Strength => c.str = val,
            Self::Willpower => c.wil = val,
            Self::Logic => c.log = val,
            Self::Intuition => c.int = val,
            Self::Charisma => c.cha = val,
            Self::ResistDrain => c.resist_drain = Some(val),
        };
        c
    }
}

#[derive(Debug)]
pub struct Transformation {
    pub output: Property,
    pub operation: Operation,
}
impl Transformation {
    pub fn transform(&self, c: Character) -> Character {
        match &self.operation {
            Operation::Inc(increment) => {
                let new_val = self.output.get(&c) + increment;
                self.output.set(c, new_val)
            }
            Operation::CeilFrac(property, divider) => {
                let new_val = f32::ceil(property.get(&c) as f32 / *divider as f32) as u8;
                self.output.set(c, new_val)
            }
        }
    }
}
