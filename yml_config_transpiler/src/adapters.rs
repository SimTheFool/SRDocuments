use serde_yaml::Value;

use crate::{domain::descriptions::CharacterDescription, utils::result::AppResult};

#[async_trait::async_trait]
pub trait ReadDescriptionsAdapter {
    async fn get_characters_descriptions(
        &self,
        identifiers: Vec<String>,
    ) -> AppResult<Vec<CharacterDescription>>;
}

#[async_trait::async_trait]
pub trait ReaderAdapter {
    fn get_value(&self, identifier: &str) -> AppResult<Value>;
}
