use domain::character::Character;
use utils::result::AppResult;

pub mod adapters;
pub mod domain;
pub mod infra_yml;
pub mod utils;

pub struct App {
    reader: Box<dyn adapters::ReadDescriptionsAdapter>,
}

impl App {
    pub fn new(reader: Box<dyn adapters::ReadDescriptionsAdapter>) -> Self {
        Self { reader }
    }

    pub async fn get_all_characters(&self, ids: Vec<String>) -> AppResult<Vec<Character>> {
        let descriptions = self.reader.get_characters_descriptions(ids).await?;

        let result = descriptions
            .into_iter()
            .map(|description| description.try_into())
            .collect::<AppResult<Vec<Character>>>()?;

        Ok(result)
    }
}
