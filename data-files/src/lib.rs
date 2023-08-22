use domain::descriptions::CharacterDescription;
use utils::result::AppResult;

pub mod domain;
pub mod infra;
pub mod utils;

#[async_trait::async_trait]
pub trait ReadDescriptionsAdapter {
    async fn get_characters_descriptions(
        &self,
        identifiers: Vec<String>,
    ) -> AppResult<Vec<CharacterDescription>>;
}

pub async fn get_all_characters(
    read_descriptions_adapter: &dyn ReadDescriptionsAdapter,
) -> AppResult<Vec<domain::character::Character>> {
    let descriptions = read_descriptions_adapter
        .get_characters_descriptions(vec![
            "wicca".to_string(),
            "warrior".to_string(),
            "thief".to_string(),
        ])
        .await?;

    let result = descriptions
        .into_iter()
        .map(|description| description.into())
        .collect();

    Ok(result)
}
