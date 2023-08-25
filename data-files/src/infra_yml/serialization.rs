use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::domain::{
    descriptions::SpecializationDescription,
    transformation::{Operation, Property, Transformation},
};

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
