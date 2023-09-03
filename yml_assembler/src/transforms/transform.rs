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

    match operations {
        Some(transformations) => {
            let mut flat_yml = FlatYml::try_from(yml)?;
            let flat_yml = apply_operations(transformations, &mut flat_yml)?;
            Ok(flat_yml.try_into()?)
        }
        None => Ok(yml),
    }
}

fn apply_operations(operations: Vec<String>, yml: &mut FlatYml) -> AppResult<FlatYml> {
    let mut context = HashMapContext::new();

    for (key, value) in yml.iter() {
        let value = match value {
            FlatYmlValue::String(s) => evalexpr::Value::String(s.clone()),
            FlatYmlValue::Number(i) => evalexpr::Value::Float(i.clone()),
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

    let mut yml = FlatYml::new();
    for (key, value) in context.iter_variables() {
        let value = match value {
            evalexpr::Value::String(s) => FlatYmlValue::String(s.clone()),
            evalexpr::Value::Float(i) => FlatYmlValue::Number(i.clone()),
            evalexpr::Value::Boolean(b) => FlatYmlValue::Bool(b.clone()),
            evalexpr::Value::Empty => FlatYmlValue::Null,
            _ => return Err(AppError::ApplyFormula(format!("Unhandled value: {}", key))),
        };

        yml.insert(key.to_string(), value);
    }

    Ok(yml)
}

#[test]
fn it_should_apply_transformations() {
    let mut yml = FlatYml::new();
    yml.insert("a.0.u".to_string(), FlatYmlValue::Number(1 as f64));
    yml.insert("a.0.v".to_string(), FlatYmlValue::Bool(false));
    yml.insert("b.x".to_string(), FlatYmlValue::Number(3.0));

    let operations = vec![
        "a.0.u = a.0.u + 1".to_string(),
        "a.0.v = a.0.v || true".to_string(),
    ];

    let result = apply_operations(operations, &mut yml).unwrap();

    assert_eq!(
        result.get("a.0.u").unwrap(),
        &FlatYmlValue::Number(2 as f64)
    );
    assert_eq!(result.get("a.0.v").unwrap(), &FlatYmlValue::Bool(true));
    assert_eq!(result.get("b.x").unwrap(), &FlatYmlValue::Number(3.0));
}
