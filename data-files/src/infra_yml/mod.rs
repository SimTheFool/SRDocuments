use self::yml_visitors::YmlAggregatorVisitor;
use crate::{
    adapters::ReadDescriptionsAdapter,
    domain::descriptions::CharacterDescription,
    utils::result::{AppError, AppResult},
};
use serde_yaml::Value;

pub mod deserialization;
pub mod yml_reader;
pub mod yml_visitors;

#[async_trait::async_trait]
impl ReadDescriptionsAdapter for YmlAggregatorVisitor {
    async fn get_characters_descriptions(
        &self,
        ids: Vec<String>,
    ) -> AppResult<Vec<CharacterDescription>> {
        let character_ymls = ids
            .iter()
            .map(|id| self.visit_from_path(id))
            .collect::<AppResult<Vec<Value>>>()?;

        let descriptions: Vec<CharacterDescription> = character_ymls
            .iter()
            .map(|yml_value| serde_yaml::from_value(yml_value.clone()).map_err(AppError::other))
            .collect::<AppResult<Vec<CharacterDescription>>>()?;

        return Ok(descriptions);
    }
}
