use crate::utils::result::{AppError, AppResult};
use evalexpr::eval;
use regex::Regex;
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Number, Value,
};
use std::str::FromStr;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Clone)]
pub struct Variables(HashMap<String, String>);
impl Deref for Variables {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Variables {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn inject(&self, val: &Value) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => self.on_tag(t),
            Value::Mapping(map) => self.on_mapping(map),
            Value::Sequence(seq) => self.on_sequence(seq),
            Value::String(str) => self.on_string(str),
            x => Ok(x.clone()),
        }
    }

    fn on_tag(&self, val: &TaggedValue) -> AppResult<Value> {
        let tag = self.inject_variables_in_string(&val.tag.to_string())?;
        let value = self.inject(&val.value)?;
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(tag),
            value,
        })))
    }

    fn on_sequence(&self, val: &Vec<Value>) -> AppResult<Value> {
        let mut new_seq: Vec<Value> = vec![];
        for value in val {
            let yml = self.inject(&value)?;
            new_seq.push(yml)
        }
        Ok(Value::Sequence(new_seq))
    }

    fn on_mapping(&self, val: &Mapping) -> AppResult<Value> {
        let mut new_map = Mapping::new();
        for (key, value) in val {
            let yml = self.inject(&value)?;
            new_map.insert(key.clone(), yml);
        }
        Ok(Value::Mapping(new_map))
    }

    fn on_string(&self, val: &str) -> AppResult<Value> {
        let new_string = self.inject_variables_in_string(val)?;

        fn contains_multibyte(s: &str) -> bool {
            for c in s.chars() {
                if c.len_utf8() > 1 {
                    return true;
                }
            }
            false
        }

        if contains_multibyte(&new_string) {
            return Ok(Value::String(new_string));
        }

        let result = match eval(&new_string) {
            Ok(evaluated) => match evaluated {
                evalexpr::Value::Boolean(b) => Value::Bool(b),
                evalexpr::Value::Int(n) => {
                    let number = Number::from_str(&n.to_string());
                    match number {
                        Ok(number) => Value::Number(number),
                        Err(_) => Value::String(new_string),
                    }
                }
                evalexpr::Value::Float(n) => {
                    let number = Number::from_str(&n.to_string());
                    match number {
                        Ok(number) => Value::Number(number),
                        Err(_) => Value::String(new_string),
                    }
                }
                _ => Value::String(new_string),
            },
            Err(_) => Value::String(new_string),
        };
        Ok(result)
    }

    fn inject_variables_in_string(&self, str: &str) -> AppResult<String> {
        let mut new_string = str.to_string();
        let mut replaced = true;

        while replaced {
            let folded: AppResult<(String, bool)> =
                self.iter()
                    .try_fold((new_string, false), |(acc, replaced), (key, value)| {
                        let regex = Regex::new(&format!(r"\${}\b", key)).map_err(|e| {
                            AppError::ParseYml(format!(
                                "{key} can't be used as variable: {}",
                                e.to_string()
                            ))
                        })?;

                        let new_string = regex.replace_all(&acc, &*value).to_string();
                        if new_string != acc {
                            return Ok((new_string, true));
                        }
                        return Ok((new_string, replaced));
                    });

            let folded = folded?;
            new_string = folded.0;
            replaced = folded.1;
        }

        Ok(new_string)
    }
}

impl TryInto<Variables> for Value {
    type Error = AppError;

    fn try_into(self) -> Result<Variables, Self::Error> {
        let mut variables = Variables::new();

        match self {
            Value::Mapping(map) => {
                for (key, value) in map {
                    let key = match key {
                        Value::String(str) => Ok(str),
                        _ => Err(AppError::ParseYml(format!("Variable key is not a string"))),
                    }?;

                    let value = match value {
                        Value::String(str) => Ok(str),
                        Value::Number(n) => Ok(n.to_string()),
                        Value::Bool(b) => Ok(b.to_string()),
                        _ => Err(AppError::ParseYml(format!(
                            "Variable {key} can not be parsed as string"
                        ))),
                    }?;

                    variables.insert(key, value);
                }
                Ok(variables)
            }
            Value::Null => Ok(variables),
            _ => Err(AppError::ParseYml(format!("Cannot parse as variables"))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_yml_part() -> &'static str {
        let yml_part = r#"
            foo: $test + $test2
        "#;
        yml_part
    }

    #[test]
    fn it_should_apply_variables_and_evaluate() {
        let yml_part: Value = serde_yaml::from_str(get_yml_part()).unwrap();
        let mut variables = Variables::new();
        variables.insert("test".to_string(), "3".to_string());
        variables.insert("test2".to_string(), "0.5".to_string());

        let expected_yml: Value = serde_yaml::from_str(
            r#"
            foo: 3.5
        "#,
        )
        .unwrap();

        let yml = variables.inject(&yml_part).unwrap();
        assert_eq!(yml, expected_yml);

        let number = match &yml {
            Value::Mapping(map) => {
                let foo = map.get("foo".to_string()).unwrap().clone();
                match foo {
                    Value::Number(number) => number.as_f64(),
                    _ => panic!("foo should be a number"),
                }
            }
            _ => panic!("yml should be a mapping"),
        };

        assert_eq!(number, Some(3.5));
    }
}
