use super::yml_reader::YmlReader;
use crate::utils::result::{AppError, AppResult};
use eval::{eval, Expr};
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Number, Value,
};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

pub struct YmlAggregatorVisitor {
    reader: YmlReader,
}

impl YmlAggregatorVisitor {
    pub fn new(path: PathBuf) -> Self {
        YmlAggregatorVisitor {
            reader: YmlReader::new(path),
        }
    }

    pub fn visit_from_path(&self, path: &str) -> AppResult<Value> {
        let yml = self.reader.load(path)?;
        let yml = self.visit(&yml)?;
        Ok(yml)
    }

    pub fn visit(&self, val: &Value) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => {
                let tag = t.tag.to_string();
                let file = match tag.starts_with('!') {
                    true => Ok(tag.trim_start_matches('!')),
                    false => Err(AppError::ParseYml(format!("Tag {tag} is not supported"))),
                }?;

                let value = &t.value;
                let variables = match value {
                    Value::Mapping(map) => Some(map.clone()),
                    Value::Null => None,
                    _ => return Err(AppError::ParseYml(format!("No valid variables for {tag}"))),
                };

                self.on_include_tag(file, variables)
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
                let variables = variables
                    .into_iter()
                    .map(|(key, value)| {
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

                        Ok((key, value))
                    })
                    .collect::<AppResult<HashMap<String, String>>>()?;
                let variable_injector = YmlVariableInjectionVisitor::new(variables);
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

pub struct YmlVariableInjectionVisitor {
    variables: HashMap<String, String>,
}

impl YmlVariableInjectionVisitor {
    pub fn new(variables: HashMap<String, String>) -> Self {
        YmlVariableInjectionVisitor { variables }
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
        println!("?????????");
        let new_string = self.inject_variables_in_string(val);

        if contains_multibyte(&new_string) {
            return Ok(Value::String(new_string));
        }

        println!("new_string: {}", new_string);
        let xxx = Expr::new(&new_string)
            .exec()
            .map_err(|_| AppError::ParseYml("sdfsdf".to_string()))?;
        println!("##### {:?}", xxx);

        let result = match eval(&new_string) {
            Ok(evaluated) => match evaluated {
                eval::Value::Bool(b) => Value::Bool(b),
                eval::Value::Number(n) => {
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
        println!("!!!!!");
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
