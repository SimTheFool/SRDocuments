use jsonschema::JSONSchema;
use serde_yaml::Value;
use std::collections::HashMap;
use std::rc::Rc;
use transformable::TransformableList;
use utils::result::AppError;
use utils::result::AppResult;
use variables::Variables;

pub mod adapters;
mod aggregator;
mod mixins;
mod transformable;
mod utils;
mod variables;

pub struct App {
    yml_reader: Rc<dyn adapters::YmlReaderAdapter>,
    schema_reader: Rc<dyn adapters::ValidationSchemaReaderAdapter>,
}

impl App {
    pub fn new(
        yml_reader: Rc<dyn adapters::YmlReaderAdapter>,
        schema_reader: Rc<dyn adapters::ValidationSchemaReaderAdapter>,
    ) -> Self {
        Self {
            yml_reader,
            schema_reader,
        }
    }

    pub fn compile_and_validate_yml(
        &self,
        yml_id: &str,
        schema_id: Option<&str>,
        variables: Option<HashMap<String, String>>,
    ) -> AppResult<(Value, Option<serde_json::Value>)> {
        let mut aggregator = aggregator::YmlAggregator::new(Rc::clone(&self.yml_reader));

        let variables: Variables = variables.unwrap_or(HashMap::new()).into();
        let yml = aggregator.load(yml_id, &variables)?;
        let mixins = aggregator.mixins;
        let yml = mixins.inject(&yml)?;

        let mut list = TransformableList::try_from(yml)?;
        list.transform()?;
        let yml = list.try_into()?;

        let schema_json = match schema_id {
            Some(schema_id) => {
                let yml_json_representation =
                    serde_json::to_value(&yml).map_err(AppError::other)?;
                let schema_json = self.schema_reader.get_validation_schema(schema_id)?;
                let validator = JSONSchema::compile(&schema_json)
                    .map_err(|e| AppError::ValidateYml(format!("Schema is not valid: {}", e)))?;
                validator.validate(&yml_json_representation).map_err(|e| {
                    let str_errors = e
                        .into_iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join("\n");

                    AppError::ValidateYml(format!("Generated yml is not valid: {}", str_errors))
                })?;

                Some(schema_json)
            }
            None => None,
        };

        Ok((yml, schema_json))
    }
}
