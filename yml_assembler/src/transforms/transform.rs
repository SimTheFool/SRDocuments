use super::flat_yml::{FlatYml, FlatYmlValue};
use crate::utils::result::{AppError, AppResult};
use evalexpr::{ContextWithMutableVariables, HashMapContext, IterateVariablesContext};
use serde_yaml::Value;

static TRANSFORMATIONS_KEY: &str = "_transform";

pub fn apply_transform(yml: Value) -> AppResult<Value> {
    let mut yml = yml.clone();

    let operations = match &mut yml {
        Value::Mapping(m) => {
            let entry = m.remove(TRANSFORMATIONS_KEY);
            match entry {
                Some(Value::String(s)) => Some(vec![s.clone()]),
                Some(Value::Sequence(s)) => {
                    let transformations: Vec<String> =
                        serde_yaml::from_value(Value::Sequence(s.clone())).map_err(|_| {
                            AppError::ApplyFormula(format!("_transform should be a list of string"))
                        })?;
                    Some(transformations)
                }
                None => None,
                _ => {
                    return Err(AppError::ApplyFormula(
                        "_transform should be a string or a list of string".to_string(),
                    ))?
                }
            }
        }
        _ => None,
    };

    let result = match operations {
        Some(transformations) => {
            let mut flat_yml = FlatYml::try_from(yml)?;
            let flat_yml = apply_operations(transformations, &mut flat_yml)?;
            Ok(flat_yml.try_into()?)
        }
        None => Ok(yml),
    };

    result
}

fn apply_operations(operations: Vec<String>, yml: &mut FlatYml) -> AppResult<FlatYml> {
    let mut context = HashMapContext::new();

    for (key, value) in yml.iter() {
        let value = match value {
            FlatYmlValue::String(s) => evalexpr::Value::String(s.clone()),
            FlatYmlValue::Float(i) => evalexpr::Value::Float(i.clone()),
            FlatYmlValue::Integer(i) => evalexpr::Value::Float(*i as f64),
            FlatYmlValue::Bool(b) => evalexpr::Value::Boolean(b.clone()),
            FlatYmlValue::Null => evalexpr::Value::Empty,
        };

        context
            .set_value(key.clone(), value)
            .map_err(|e| AppError::ApplyFormula(format!("Transformation context error {}", e)))?;
    }

    for oper in operations {
        evalexpr::eval_with_context_mut(&oper, &mut context)
            .map_err(|e| AppError::ApplyFormula(e.to_string()))?;
    }

    let mut flat_yml = FlatYml::new();
    for (key, value) in context.iter_variables() {
        let value = match value {
            evalexpr::Value::String(s) => FlatYmlValue::String(s.clone()),
            evalexpr::Value::Float(i) => {
                let i_int: Option<i64> = {
                    let is_int = i.fract() == 0.0;
                    let is_int = is_int && i < (i64::MAX as f64);
                    let is_int = is_int && i > (i64::MIN as f64);
                    match is_int {
                        true => Some(i.clone() as i64),
                        false => None,
                    }
                };

                match i_int {
                    Some(i_int) => FlatYmlValue::Integer(i_int),
                    None => FlatYmlValue::Float(i.clone()),
                }
            }
            evalexpr::Value::Boolean(b) => FlatYmlValue::Bool(b.clone()),
            evalexpr::Value::Empty => FlatYmlValue::Null,
            _ => return Err(AppError::ApplyFormula(format!("Unhandled value: {}", key))),
        };

        flat_yml.insert(key.to_string(), value);
    }

    Ok(flat_yml)
}

#[test]
fn it_should_apply_transformations() {
    let mut yml = FlatYml::new();
    yml.insert("a.0.u".to_string(), FlatYmlValue::Float(1 as f64));
    yml.insert("a.0.v".to_string(), FlatYmlValue::Bool(false));
    yml.insert("b.x".to_string(), FlatYmlValue::Float(3.0));

    let operations = vec![
        "a.0.u = a.0.u + 1".to_string(),
        "a.0.v = a.0.v || true".to_string(),
    ];

    let result = apply_operations(operations, &mut yml).unwrap();

    let get_value = |key: &str| &result.iter().find(|(k, _)| k == key).unwrap().1;

    assert_eq!(get_value("a.0.u"), &FlatYmlValue::Integer(2));
    assert_eq!(get_value("a.0.v"), &FlatYmlValue::Bool(true));
    assert_eq!(get_value("b.x"), &FlatYmlValue::Integer(3));
}

#[test]
fn it_should_output_float_and_int() {
    let mut yml = FlatYml::new();
    yml.insert("my_int".to_string(), FlatYmlValue::Integer(12));
    yml.insert("my_float".to_string(), FlatYmlValue::Float(123.456));

    let operations = vec![
        "a = my_int + 1.0".to_string(),
        "b = my_float + 2".to_string(),
        "c = my_int / 3.0".to_string(),
    ];

    let result = apply_operations(operations, &mut yml).unwrap();

    let get_value = |key: &str| &result.iter().find(|(k, _)| k == key).unwrap().1;

    assert_eq!(get_value("a"), &FlatYmlValue::Integer(13));
    assert_eq!(get_value("b"), &FlatYmlValue::Float(125.456));
    assert_eq!(get_value("c"), &FlatYmlValue::Integer(4));
}
