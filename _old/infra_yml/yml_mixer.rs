use crate::utils::result::{AppError, AppResult};
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Value,
};
use std::collections::HashMap;

pub struct YmlMixer {
    mixed_in: HashMap<Value, Vec<Value>>,
}

impl YmlMixer {
    const MIX_TAG: &'static str = "!mix";

    pub fn new() -> Self {
        YmlMixer {
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
