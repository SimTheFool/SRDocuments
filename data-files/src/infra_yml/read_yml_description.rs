use crate::{
    adapters::ReadDescriptionsAdapter,
    domain::descriptions::{CharacterDescription, SpecializationDescription},
    utils::result::{AppError, AppResult},
};
use serde::{de::Visitor, Deserialize, Serialize};
use serde_yaml::{self, from_value, value::TaggedValue, Mapping, Value};
use std::{clone, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct CharacterYml {
    pub name: String,
    pub description: String,
    #[serde(alias = "CON")]
    pub constitution: u8,
    #[serde(alias = "AGI")]
    pub agility: u8,
    #[serde(alias = "REA")]
    pub reaction: u8,
    #[serde(alias = "FOR")]
    pub strength: u8,
    #[serde(alias = "VOL")]
    pub willpower: u8,
    #[serde(alias = "LOG")]
    pub logic: u8,
    #[serde(alias = "INT")]
    pub intuition: u8,
    #[serde(alias = "CHA")]
    pub charisma: u8,
    #[serde(alias = "ESS")]
    pub essence: u8,
    #[serde(alias = "Edge")]
    pub edge: u8,
    #[serde(alias = "specialisation")]
    pub specializations: Vec<String>,
}

pub struct YmlAggregatorVisitor {
    yml_filesystem_path: PathBuf,
}

impl YmlAggregatorVisitor {
    pub fn new(path: PathBuf) -> Self {
        YmlAggregatorVisitor {
            yml_filesystem_path: path,
        }
    }

    pub fn visit_from_path(&self, path: &str) -> AppResult<Value> {
        let path = self.yml_filesystem_path.join(format!("{path}.yml"));
        let file = std::fs::File::open(path).map_err(AppError::as_other)?;
        let yml: Value = serde_yaml::from_reader(file).map_err(AppError::as_other)?;
        let yml = self.visit(&yml)?;
        Ok(yml)
    }

    pub fn visit(&self, val: &Value) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => self.on_tagged_value(t),
            Value::Mapping(map) => self.on_mapping_value(map),
            Value::Sequence(seq) => self.on_sequence_value(seq),
            x => Ok(x.clone()),
        }
    }

    fn on_tagged_value(&self, val: &TaggedValue) -> AppResult<Value> {
        let tag = val.tag.to_string();

        let file_path = tag.trim_start_matches('!');
        let path = self.yml_filesystem_path.join(format!("{file_path}.yml"));
        let file = std::fs::File::open(path).map_err(AppError::as_other)?;

        let yml: Value = serde_yaml::from_reader(file).map_err(AppError::as_other)?;
        let yml = self.visit(&yml)?;
        Ok(yml)
    }

    fn on_mapping_value(&self, val: &Mapping) -> AppResult<Value> {
        let mut new_map = Mapping::new();
        for (key, value) in val {
            let yml = self.visit(&value)?;
            new_map.insert(key.clone(), yml);
        }
        Ok(Value::Mapping(new_map))
    }

    fn on_sequence_value(&self, val: &Vec<Value>) -> AppResult<Value> {
        let mut new_seq: Vec<Value> = vec![];
        for value in val {
            let yml = self.visit(&value)?;
            new_seq.push(yml)
        }
        Ok(Value::Sequence(new_seq))
    }
}

pub struct YmlDescriptionReader {
    yml_visitor: YmlAggregatorVisitor,
}
impl YmlDescriptionReader {
    pub fn new(path: PathBuf) -> Self {
        let visitor = YmlAggregatorVisitor::new(path);

        YmlDescriptionReader {
            yml_visitor: visitor,
        }
    }

    pub fn get_aggregated_yml(&self, path: &str) -> AppResult<Value> {
        let yml = self.yml_visitor.visit_from_path(path)?;
        Ok(yml)
    }
}
#[async_trait::async_trait]
impl ReadDescriptionsAdapter for YmlDescriptionReader {
    async fn get_characters_descriptions(
        &self,
        ids: Vec<String>,
    ) -> AppResult<Vec<CharacterDescription>> {
        /* let character_ymls = ids
        .iter()
        .map(|id| self.get_yml("characters", id))
        .collect::<AppResult<Vec<Value>>>()?; */

        /* character_ymls
        .iter()
        .map(|c| {

        })
        .collect::<AppResult<Vec<Value>>>()?; */

        /*         let files = ids
            .iter()
            .map(|id| {
                let path = self.path.join("characters").join(format!("{id}.yml"));

                std::fs::File::open(path).map_err(AppError::as_other)
            })
            .collect::<AppResult<Vec<_>>>()?;

        let descriptions = files
            .into_iter()
            .map(|file| {
                let charac_yml: CharacterYml =
                    serde_yaml::from_reader(file).map_err(AppError::as_other)?;

                let specialization_files = charac_yml
                    .specializations
                    .iter()
                    .map(|spec| {
                        let path = self
                            .path
                            .join("specializations")
                            .join(format!("{spec}.yml"));

                        std::fs::File::open(path).map_err(AppError::as_other)
                    })
                    .collect::<AppResult<Vec<_>>>()?;

                let specializations = specialization_files
                    .iter()
                    .map(|file| serde_yaml::from_reader(file).map_err(AppError::as_other))
                    .collect::<AppResult<Vec<SpecializationDescription>>>()?;

                Ok(CharacterDescription {
                    name: charac_yml.name,
                    description: charac_yml.description,
                    con: charac_yml.constitution,
                    agi: charac_yml.agility,
                    rea: charac_yml.reaction,
                    str: charac_yml.strength,
                    wil: charac_yml.willpower,
                    log: charac_yml.logic,
                    int: charac_yml.intuition,
                    cha: charac_yml.charisma,
                    essence: charac_yml.essence,
                    edge_rank: charac_yml.edge,
                    specializations,
                    ..CharacterDescription::default()
                })
            })
            .collect::<AppResult<Vec<_>>>()?; */

        return Ok(vec![CharacterDescription::default()]);
        //return Ok(descriptions);
    }
}
