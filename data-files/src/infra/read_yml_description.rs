use std::path::PathBuf;

use crate::{
    domain::{
        descriptions::{CharacterDescription, SpecializationDescription},
        transformation::{self, Operation, Property, Transformation},
    },
    utils::result::{AppError, AppResult},
    ReadDescriptionsAdapter,
};
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};

#[derive(Debug, Serialize, Deserialize)]
struct CharacterYml {
    pub name: String,
    #[serde(alias = "CON")]
    pub constitution: u8,
    #[serde(alias = "VOL")]
    pub willpower: u8,
    #[serde(alias = "STR")]
    pub strength: u8,
    #[serde(alias = "specialisation")]
    pub specializations: Vec<String>,
}

impl<'de> Deserialize<'de> for Transformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let transf_yml = serde_yaml::Value::deserialize(deserializer)?;

        let transf_map = match transf_yml {
            Value::Mapping(map) => Ok(map),
            _ => Err(serde::de::Error::custom(
                "Expected a mapping for transformation deserialization",
            )),
        }?;

        let operation = transf_map.get("operation");
        let parameter = transf_map.get("parameter");
        let property = transf_map.get("property");

        let (operation, parameter) = match (operation, parameter) {
            (None, _) => Err(serde::de::Error::custom("No operation specified")),
            (Some(Value::String(operation)), Some(Value::Number(parameter))) => Ok((
                operation,
                parameter
                    .as_u64()
                    .ok_or_else(|| serde::de::Error::custom("Invalid parameter specified"))?
                    as u8,
            )),
            _ => Err(serde::de::Error::custom(
                "Invalid operation or parameter specified",
            )),
        }?;

        let operation = match operation.as_str() {
            "ADD" => Ok(Operation::Add(parameter)),
            "CEIL_FRAC" => Ok(Operation::CeilFrac(parameter)),
            _ => Err(serde::de::Error::custom("Invalid operation specified")),
        }?;

        let property = match property {
            Some(Value::String(property)) => match property.as_str() {
                "CON" => Ok(Property::Constitution),
                "VOL" => Ok(Property::Willpower),
                "STR" => Ok(Property::Strength),
                "MAG" => Ok(Property::Magic),
                _ => Err(serde::de::Error::custom("Invalid property specified")),
            },
            _ => Err(serde::de::Error::custom("No property specified")),
        }?;

        Ok(Transformation {
            operation,
            property,
        })
    }
}

impl<'de> Deserialize<'de> for SpecializationDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct SpecializationYml {
            name: String,
            description: String,
            transform: Vec<Transformation>,
        }

        let xxx = SpecializationYml::deserialize(deserializer)?;

        Ok(SpecializationDescription {
            name: xxx.name,
            description: xxx.description,
            transform: xxx.transform,
        })
    }
}

pub struct YmlDescriptionReader {
    path: PathBuf,
}
impl YmlDescriptionReader {
    pub fn new(path: PathBuf) -> Self {
        YmlDescriptionReader { path }
    }
}
#[async_trait::async_trait]
impl ReadDescriptionsAdapter for YmlDescriptionReader {
    async fn get_characters_descriptions(
        &self,
        ids: Vec<String>,
    ) -> AppResult<Vec<CharacterDescription>> {
        let id = &ids[0];
        let file = std::fs::File::open(&self.path.join("characters").join(format!("{id}.yml")))
            .map_err(AppError::as_other)?;

        let charac_yml: CharacterYml = serde_yaml::from_reader(file).map_err(AppError::as_other)?;

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

        let description = CharacterDescription {
            constitution: charac_yml.constitution,
            willpower: charac_yml.willpower,
            strength: charac_yml.strength,
            magic: None,
            specializations,
        };

        return Ok(vec![description]);
    }
}
