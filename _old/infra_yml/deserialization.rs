use serde::{de::Error, Deserialize};
use serde_yaml::Value;

use crate::domain::{
    descriptions::{CharacterDescription, SpecializationDescription},
    transformation::Transformation,
};

/* impl<'de> Deserialize<'de> for Property {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let transf_yml = serde_yaml::Value::deserialize(deserializer)?;

        let property = match transf_yml {
            Value::String(property) => match property.as_str() {
                "MAG" => Ok(Property::Magic),
                "INT" => Ok(Property::Intuition),
                "REA" => Ok(Property::Reaction),
                "STR" => Ok(Property::Strength),
                "CHA" => Ok(Property::Charisma),
                "CON" => Ok(Property::Constitution),
                "LOG" => Ok(Property::Logic),
                "WIL" => Ok(Property::Willpower),
                "AGI" => Ok(Property::Agility),
                "RESIST_DRAIN" => Ok(Property::ResistDrain),
                _ => Err(Error::custom("Invalid property specified")),
            },
            _ => Err(Error::custom("Cannot parse properties")),
        }?;

        Ok(property)
    }
}

impl<'de> Deserialize<'de> for Operation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let yml = serde_yaml::Value::deserialize(deserializer)?;

        let (operation, params) = match yml {
            Value::Sequence(seq) if seq.len() == 2 => {
                let op = seq[0].clone();
                let params = seq[1].clone();
                let op = match op {
                    Value::String(op) => Ok(op),
                    _ => Err(Error::custom("Specified operation is not a string")),
                }?;
                let params = match params {
                    Value::Sequence(params) => Ok(params),
                    _ => Err(Error::custom("Specified parameters should be an array")),
                }?;
                Ok((op, params))
            }
            _ => Err(Error::custom("Cannot parse operation")),
        }?;

        #[derive(Debug, Deserialize)]
        struct U8(u8);
        impl TryFrom<Value> for U8 {
            type Error = String;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                let result = serde_yaml::from_value(value.clone())
                    .map_err(|e| format!("Cannot convert as u8: {:?}", e))?;
                Ok(result)
            }
        }

        let operation = match operation.as_str() {
            "INC" => {
                if !params.len() == 1 {
                    return Err(Error::custom("Invalid number of parameters for INC"));
                }
                let incr_number = U8::try_from(params[0].clone())
                    .map_err(|e| Error::custom(format!("Cannot convert as u8: {:?}", e)))?;
                Ok(Operation::Inc(incr_number.0))
            }
            "CEIL_FRAC" => {
                if !params.len() == 2 {
                    return Err(Error::custom("Invalid number of parameters for INC"));
                }
                let tradition: Property = serde_yaml::from_value(params[0].clone())
                    .map_err(|e| Error::custom(format!("Cannot convert as property: {:?}", e)))?;

                let ratio = U8::try_from(params[1].clone())
                    .map_err(|e| Error::custom(format!("Cannot convert as u8: {:?}", e)))?;

                Ok(Operation::CeilFrac(tradition, ratio.0))
            }
            _ => Err(Error::custom("Invalid operation specified")),
        }?;

        Ok(operation)
    }
} */

impl<'de> Deserialize<'de> for Transformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /* let yml = serde_yaml::Value::deserialize(deserializer)?;
        let yml = match yml {
            Value::Mapping(map) => Ok(map),
            _ => Err(Error::custom(
                "Expected a mapping for transformation deserialization",
            )),
        }?;

        let operation_yml = match yml.get("operation") {
            Some(x) => Ok(x),
            None => Err(Error::custom("No operation specified")),
        }?;

        let empty_params = Value::Sequence(vec![]);
        let parameters_yml = match yml.get("parameters") {
            Some(x) => Ok(x),
            None => Ok(&empty_params),
        }?;

        let output_value = yml
            .get("output")
            .ok_or_else(|| "No output specified")
            .map_err(|e| Error::custom(e))?;

        let output: Property =
            serde_yaml::from_value(output_value.clone()).map_err(|e| Error::custom(e))?;

        let operation: Operation = serde_yaml::from_value(Value::Sequence(vec![
            operation_yml.clone(),
            parameters_yml.clone(),
        ]))
        .map_err(|e| Error::custom(e))?;

        Ok(Transformation { output, operation }) */

        let yml = serde_yaml::Value::deserialize(deserializer)?;

        match yml {
            Value::String(s) => Ok(Transformation::new(s.as_str())),
            _ => Err(Error::custom(
                "Expected a string for transformation deserialization",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for SpecializationDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Proxy {
            name: Option<String>,
            description: Option<String>,
            transform: Vec<Transformation>,
        }

        let xxx = Proxy::deserialize(deserializer)?;

        Ok(SpecializationDescription {
            name: xxx.name,
            description: xxx.description,
            transform: xxx.transform,
        })
    }
}

impl<'de> Deserialize<'de> for CharacterDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Proxy {
            pub name: String,
            pub description: String,
            #[serde(alias = "CON")]
            pub con: u8,
            #[serde(alias = "AGI")]
            pub agi: u8,
            #[serde(alias = "REA")]
            pub rea: u8,
            #[serde(alias = "FOR")]
            pub str: u8,
            #[serde(alias = "VOL")]
            pub wil: u8,
            #[serde(alias = "LOG")]
            pub log: u8,
            #[serde(alias = "INT")]
            pub int: u8,
            #[serde(alias = "CHA")]
            pub cha: u8,
            #[serde(alias = "ESS")]
            pub essence: u8,
            #[serde(alias = "Edge")]
            pub edge: u8,
            #[serde(alias = "specs")]
            pub specs: Vec<SpecializationDescription>,
        }

        let xxx = Proxy::deserialize(deserializer)?;

        Ok(CharacterDescription {
            name: xxx.name,
            description: xxx.description,
            con: xxx.con,
            agi: xxx.agi,
            rea: xxx.rea,
            str: xxx.str,
            wil: xxx.wil,
            log: xxx.log,
            int: xxx.int,
            cha: xxx.cha,
            essence: xxx.essence,
            edge_rank: xxx.edge,
            specializations: xxx.specs,
        })
    }
}
