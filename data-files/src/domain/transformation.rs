use crate::utils::result::{AppError, AppResult};
use evalexpr::{context_map, Context, HashMapContext};

use super::character::Character;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Transformation(pub String);
impl Transformation {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn transform(&self, c: Character) -> AppResult<Character> {
        let mut context = context_map! {
            "INT" => c.int as i64,
            "LOG" => c.log as i64,
            "MAG" => c.magic.unwrap_or(0) as i64,
            "RES" => c.resonance.unwrap_or(0) as i64,
            "SUBM" => c.submersion.unwrap_or(0) as i64,
            "RESIST_DRAIN" => c.resist_drain.unwrap_or(0) as f64,
        }
        .map_err(AppError::other)?;

        fn get_u8(key: &str, context: &HashMapContext) -> AppResult<u8> {
            let val = context.get_value(key).ok_or_else(|| {
                AppError::ApplyFormula(format!("No {key} value set in context").to_string())
            })?;

            let val = match val {
                evalexpr::Value::Int(i) => u8::try_from(i.clone())
                    .map_err(|e| AppError::ApplyFormula(format!("Cannot parse {e}").to_string()))?,
                evalexpr::Value::Float(i) if i >= &0.0 && i <= &255.0 && i.fract() == 0.0 => {
                    i.clone() as u8
                }
                _ => Err(AppError::ApplyFormula(
                    format!("{key}  value is not an integer").to_string(),
                ))?,
            };

            return Ok(val);
        }

        evalexpr::eval_with_context_mut(&self.0, &mut context)
            .map_err(|e| AppError::ApplyFormula(e.to_string()))?;

        Ok(Character {
            magic: Some(get_u8("MAG", &context)?),
            resist_drain: Some(get_u8("RESIST_DRAIN", &context)?),
            resonance: Some(get_u8("RES", &context)?),
            submersion: Some(get_u8("SUBM", &context)?),
            ..c
        })
    }
}
