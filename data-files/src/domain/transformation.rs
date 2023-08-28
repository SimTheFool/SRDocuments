use evalexpr::{context_map, Context, HashMapContext};

use crate::utils::result::{AppError, AppResult};

use super::character::Character;
use std::fmt::Debug;

/* #[derive(Debug, PartialEq, Eq)]
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
} */

#[derive(Debug)]
pub struct Transformation(pub String);
impl Transformation {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn transform(&self, c: Character) -> AppResult<Character> {
        let mut context = context_map! {
            "MAG" => c.magic.unwrap_or(0) as i64,
            "RESIST_DRAIN" => c.resist_drain.unwrap_or(0) as i64,
            "INT" => c.int as i64,
        }
        .map_err(AppError::other)?;

        fn get_u8(key: &str, context: &HashMapContext) -> AppResult<u8> {
            let mag_value = context.get_value(key).ok_or_else(|| {
                AppError::ApplyFormula(format!("No {key} value set in context").to_string())
            })?;

            let mag_value = match mag_value {
                evalexpr::Value::Int(i) => u8::try_from(i.clone()).map_err(|_| {
                    AppError::ApplyFormula(format!("{key}  value is not an integer").to_string())
                })?,
                _ => Err(AppError::ApplyFormula(
                    format!("{key}  value is not an integer").to_string(),
                ))?,
            };

            return Ok(mag_value);
        }

        println!("sdfsdfsdfsdf");

        evalexpr::eval_with_context_mut(&self.0, &mut context)
            .map_err(|e| AppError::ApplyFormula(e.to_string()))?;

        println!("uuiiooo");

        Ok(Character {
            magic: Some(get_u8("MAG", &context)?),
            resist_drain: Some(get_u8("RESIST_DRAIN", &context)?),
            ..c
        })

        /* match &self.operation {
            Operation::Inc(increment) => {
                let new_val = self.output.get(&c) + increment;
                self.output.set(c, new_val)
            }
            Operation::CeilFrac(property, divider) => {
                let new_val = f32::ceil(property.get(&c) as f32 / *divider as f32) as u8;
                self.output.set(c, new_val)
            }
        } */
    }
}
