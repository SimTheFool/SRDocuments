use crate::{
    adapters::ReadDescriptionsAdapter,
    domain::descriptions::CharacterDescription,
    utils::result::{AppError, AppResult},
};
use serde_yaml::Value;

use self::yml_aggregator::YmlAggregator;

pub mod deserialization;
pub mod yml_aggregator;
pub mod yml_loader;
pub mod yml_mixer;
pub mod yml_reader;

#[async_trait::async_trait]
impl ReadDescriptionsAdapter for YmlAggregator {
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
