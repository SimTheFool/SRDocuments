use crate::{
    adapters,
    utils::result::{AppError, AppResult},
};
use evalexpr::eval;
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Number, Value,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
    str::FromStr,
};

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
        Variables(HashMap::new())
    }
}

impl TryInto<Variables> for Mapping {
    type Error = AppError;

    fn try_into(self) -> Result<Variables, Self::Error> {
        let mut variables = Variables::new();
        for (key, value) in self {
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
}

pub struct YmlLoader {
    reader: Rc<dyn adapters::YmlReaderAdapter>,
}

impl YmlLoader {
    pub fn new(reader: Rc<dyn adapters::YmlReaderAdapter>) -> Self {
        YmlLoader { reader }
    }

    pub fn load<T>(&self, identifier: &str, variables: T) -> AppResult<Value>
    where
        T: TryInto<Variables, Error = AppError>,
    {
        let variables: Variables = variables.try_into()?;
        let yml = self.reader.get_value(identifier)?;
        let yml = self.visit(&yml, &variables)?;

        Ok(yml)
    }

    fn visit(&self, val: &Value, variables: &Variables) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => self.on_tag(t, variables),
            Value::Mapping(map) => self.on_mapping(map, variables),
            Value::Sequence(seq) => self.on_sequence(seq, variables),
            Value::String(str) => self.on_string(str, variables),
            x => Ok(x.clone()),
        }
    }

    fn on_tag(&self, val: &TaggedValue, variables: &Variables) -> AppResult<Value> {
        let tag = self.inject_variables_in_string(&val.tag.to_string(), variables);
        let value = self.visit(&val.value, variables)?;
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(tag),
            value,
        })))
    }

    fn on_sequence(&self, val: &Vec<Value>, variables: &Variables) -> AppResult<Value> {
        let mut new_seq: Vec<Value> = vec![];
        for value in val {
            let yml = self.visit(&value, variables)?;
            new_seq.push(yml)
        }
        Ok(Value::Sequence(new_seq))
    }

    fn on_mapping(&self, val: &Mapping, variables: &Variables) -> AppResult<Value> {
        let mut new_map = Mapping::new();
        for (key, value) in val {
            let yml = self.visit(&value, variables)?;
            new_map.insert(key.clone(), yml);
        }
        Ok(Value::Mapping(new_map))
    }

    fn on_string(&self, val: &str, variables: &Variables) -> AppResult<Value> {
        let new_string = self.inject_variables_in_string(val, variables);

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

    fn inject_variables_in_string(&self, str: &str, variables: &Variables) -> String {
        let mut new_string = str.to_string();
        let mut replaced = true;

        while replaced {
            (new_string, replaced) =
                variables
                    .iter()
                    .fold((new_string, false), |(acc, replaced), (key, value)| {
                        let new_string = acc.replace(&format!("${key}"), &value);
                        if new_string != acc {
                            return (new_string, true);
                        }
                        return (new_string, replaced);
                    });
        }

        new_string
    }
}

fn contains_multibyte(s: &str) -> bool {
    for c in s.chars() {
        if c.len_utf8() > 1 {
            return true;
        }
    }
    false
}
