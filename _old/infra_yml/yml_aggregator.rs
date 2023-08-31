use std::path::PathBuf;

use crate::infra_yml::yml_loader::YmlLoader;
use crate::utils::result::{AppError, AppResult};
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Value,
};

use super::{yml_mixer::YmlMixer, yml_reader::YmlReader};

pub struct YmlAggregator {
    reader: YmlReader,
}

impl YmlAggregator {
    const INCLUDE_TAG_PREFIX: &'static str = "!inc::";

    pub fn new(path: PathBuf) -> Self {
        YmlAggregator {
            reader: YmlReader::new(path),
        }
    }

    pub fn visit_from_path(&self, path: &str) -> AppResult<Value> {
        let mut yml_mixer = YmlMixer::new();

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
                let variable_injector = YmlLoader::new(variables)?;
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
