use super::yml_reader::YmlReader;
use crate::utils::result::{AppError, AppResult};
use evalexpr::eval;
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Number, Value,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::PathBuf,
    str::FromStr,
};

pub struct YmlAggregatorVisitor {
    reader: YmlReader,
}

impl YmlAggregatorVisitor {
    const INCLUDE_TAG_PREFIX: &'static str = "!inc::";

    pub fn new(path: PathBuf) -> Self {
        YmlAggregatorVisitor {
            reader: YmlReader::new(path),
        }
    }

    pub fn visit_from_path(&self, path: &str) -> AppResult<Value> {
        let mut yml_mixer = YmlMixerVisitor::new();

        let yml = self.reader.load(path)?;
        let yml = self.visit(&yml)?;
        let yml = yml_mixer.mix(&yml)?;
        Ok(yml)
    }

    pub fn visit(&self, val: &Value) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => {
                let tag = t.tag.to_string();
                let value = &t.value;

                match tag.starts_with(Self::INCLUDE_TAG_PREFIX) {
                    true => {
                        let file = tag.trim_start_matches(Self::INCLUDE_TAG_PREFIX);

                        let variables = match value {
                            Value::Mapping(map) => Some(map.clone()),
                            Value::Null => None,
                            _ => {
                                return Err(AppError::ParseYml(format!(
                                    "No valid variables for {tag}"
                                )))
                            }
                        };

                        self.on_include_tag(file, variables)
                    }
                    false => {
                        let yml = self.visit(value)?;
                        Ok(Value::Tagged(Box::new(TaggedValue {
                            tag: Tag::new(tag),
                            value: yml,
                        })))
                    }
                }
            }
            Value::Mapping(map) => self.on_mapping(map),
            Value::Sequence(seq) => self.on_sequence(seq),
            x => Ok(x.clone()),
        }
    }

    fn on_include_tag(&self, include_path: &str, variables: Option<Mapping>) -> AppResult<Value> {
        let yml = self.reader.load(include_path)?;
        let yml = self.visit(&yml)?;

        match variables {
            Some(variables) => {
                let variable_injector = YmlVariableInjectionVisitor::new(variables)?;
                let injected_yml = variable_injector.visit(&yml)?;
                Ok(injected_yml)
            }
            None => Ok(yml),
        }
    }

    fn on_mapping(&self, val: &Mapping) -> AppResult<Value> {
        let mut new_map = Mapping::new();
        for (key, value) in val {
            let yml = self.visit(&value)?;
            new_map.insert(key.clone(), yml);
        }
        Ok(Value::Mapping(new_map))
    }

    fn on_sequence(&self, val: &Vec<Value>) -> AppResult<Value> {
        let mut new_seq: Vec<Value> = vec![];
        for value in val {
            let yml = self.visit(&value)?;
            new_seq.push(yml)
        }
        Ok(Value::Sequence(new_seq))
    }
}

struct Variables(HashMap<String, String>);
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

struct YmlVariableInjectionVisitor {
    variables: Variables,
}

impl YmlVariableInjectionVisitor {
    pub fn new<T>(variables: T) -> AppResult<Self>
    where
        T: TryInto<Variables, Error = AppError>,
    {
        let variables = variables.try_into()?;

        Ok(YmlVariableInjectionVisitor { variables })
    }

    pub fn visit(&self, val: &Value) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => self.on_tag(t),
            Value::Mapping(map) => self.on_mapping(map),
            Value::Sequence(seq) => self.on_sequence(seq),
            Value::String(str) => self.on_string(str),
            x => Ok(x.clone()),
        }
    }

    fn on_tag(&self, val: &TaggedValue) -> AppResult<Value> {
        let tag = self.inject_variables_in_string(&val.tag.to_string());
        let value = self.visit(&val.value)?;
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(tag),
            value,
        })))
    }

    fn on_sequence(&self, val: &Vec<Value>) -> AppResult<Value> {
        let mut new_seq: Vec<Value> = vec![];
        for value in val {
            let yml = self.visit(&value)?;
            new_seq.push(yml)
        }
        Ok(Value::Sequence(new_seq))
    }

    fn on_mapping(&self, val: &Mapping) -> AppResult<Value> {
        let mut new_map = Mapping::new();
        for (key, value) in val {
            let yml = self.visit(&value)?;
            new_map.insert(key.clone(), yml);
        }
        Ok(Value::Mapping(new_map))
    }

    fn on_string(&self, val: &str) -> AppResult<Value> {
        let new_string = self.inject_variables_in_string(val);

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

    fn inject_variables_in_string(&self, str: &str) -> String {
        let mut new_string = str.to_string();
        let mut replaced = true;

        while replaced {
            (new_string, replaced) = self.variables.clone().into_iter().fold(
                (new_string, false),
                |(acc, replaced), (key, value)| {
                    let new_string = acc.replace(&format!("${key}"), &value);
                    if new_string != acc {
                        return (new_string, true);
                    }
                    return (new_string, replaced);
                },
            );
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

pub struct YmlMixerVisitor {
    mixed_in: HashMap<Value, Vec<Value>>,
}

impl YmlMixerVisitor {
    const MIX_TAG: &'static str = "!mix";

    pub fn new() -> Self {
        YmlMixerVisitor {
            mixed_in: HashMap::new(),
        }
    }

    pub fn mix(&mut self, val: &Value) -> AppResult<Value> {
        let yml = self.visit(&val)?;

        let yml = match (yml, &self.mixed_in) {
            (yml, mixed_in) if mixed_in.is_empty() => Ok(yml.clone()),
            (Value::Mapping(map), mixed_in) => {
                let mut new_map = map.clone();
                for (key, value) in mixed_in {
                    let current_value = map.get(&key);
                    let new_value = match current_value {
                        None => value.clone(),
                        Some(Value::Sequence(current_value)) => {
                            let mut new_value = value.clone();
                            new_value.extend(current_value.clone());
                            new_value
                        }
                        Some(current_value) => {
                            let mut new_value = value.clone();
                            new_value.push(current_value.clone());
                            new_value
                        }
                    };
                    new_map.insert(key.clone(), Value::Sequence(new_value));
                }

                Ok(Value::Mapping(new_map))
            }
            _ => Err(AppError::ParseYml(format!(
                "Yml root should be composed of key-value pairs if you want to use !mix tag"
            ))),
        }?;

        Ok(yml)
    }

    fn visit(&mut self, val: &Value) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => self.on_tag(t),
            Value::Mapping(map) => self.on_mapping(map),
            Value::Sequence(seq) => self.on_sequence(seq),
            x => Ok(x.clone()),
        }
    }

    fn on_tag(&mut self, val: &TaggedValue) -> AppResult<Value> {
        let tag = &val.tag.to_string();
        let value = &val.value;

        let yml = self.visit(value)?;
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(tag),
            value: yml,
        })))
    }

    fn on_sequence(&mut self, val: &Vec<Value>) -> AppResult<Value> {
        let mut new_seq: Vec<Value> = vec![];
        for value in val {
            let yml = self.visit(&value)?;
            if let Value::Null = yml {
                continue;
            }
            new_seq.push(yml)
        }
        if new_seq.is_empty() {
            return Ok(Value::Null);
        }
        Ok(Value::Sequence(new_seq))
    }

    fn on_mapping(&mut self, val: &Mapping) -> AppResult<Value> {
        let mut new_map = Mapping::new();
        for (key, value) in val {
            let value = match value {
                Value::Tagged(t) => {
                    let tag = &t.tag.to_string();
                    let value = &t.value;

                    match tag.starts_with(Self::MIX_TAG) {
                        true => {
                            let yml = self.visit(value)?;

                            self.mixed_in
                                .entry(key.clone())
                                .or_insert(Vec::new())
                                .extend(match yml {
                                    Value::Sequence(seq) => seq,
                                    _ => vec![yml],
                                });

                            Ok(Value::Null)
                        }
                        false => self.visit(value),
                    }?
                }
                _ => self.visit(value)?,
            };

            if let Value::Null = value {
                continue;
            }
            let yml = self.visit(&value)?;
            new_map.insert(key.clone(), yml);
        }
        if new_map.is_empty() {
            return Ok(Value::Null);
        }
        Ok(Value::Mapping(new_map))
    }
}
