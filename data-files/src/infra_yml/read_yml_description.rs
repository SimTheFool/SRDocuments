use crate::{
    adapters::ReadDescriptionsAdapter,
    domain::descriptions::{CharacterDescription, SpecializationDescription},
    utils::result::{AppError, AppResult},
};
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::path::PathBuf;

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
        let files = ids
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
                    constitution: charac_yml.constitution,
                    willpower: charac_yml.willpower,
                    strength: charac_yml.strength,
                    magic: None,
                    specializations,
                })
            })
            .collect::<AppResult<Vec<_>>>()?;

        return Ok(descriptions);
    }
}
