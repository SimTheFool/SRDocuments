use crate::utils::result::{AppError, AppResult};
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Value,
};

use super::yml_loader::YmlLoader;

pub struct YmlAggregator<'a> {
    loader: &'a YmlLoader,
}

impl YmlAggregator<'_> {
    const INCLUDE_TAG_PREFIX: &'static str = "!inc::";

    pub fn new<'a>(loader: &'a YmlLoader) -> YmlAggregator<'a> {
        YmlAggregator { loader }
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

    fn on_include_tag(&self, identifier: &str, variables: Option<Mapping>) -> AppResult<Value> {
        let yml = self
            .loader
            .load(identifier, variables.unwrap_or(Mapping::new()))?;
        let yml = self.visit(&yml)?;
        Ok(yml)
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
