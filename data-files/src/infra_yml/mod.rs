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
            .map(|yml_value| serde_yaml::from_value(yml_value.clone()).map_err(AppError::as_other))
            .collect::<AppResult<Vec<CharacterDescription>>>()?;

        println!("descriptions: {:?}", descriptions);

        /* character_ymls
        .iter()
        .map(|c| {

        })
        .collect::<AppResult<Vec<Value>>>()?; */

        /*         let files = ids
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
                    name: charac_yml.name,
                    description: charac_yml.description,
                    con: charac_yml.constitution,
                    agi: charac_yml.agility,
                    rea: charac_yml.reaction,
                    str: charac_yml.strength,
                    wil: charac_yml.willpower,
                    log: charac_yml.logic,
                    int: charac_yml.intuition,
                    cha: charac_yml.charisma,
                    essence: charac_yml.essence,
                    edge_rank: charac_yml.edge,
                    specializations,
                    ..CharacterDescription::default()
                })
            })
            .collect::<AppResult<Vec<_>>>()?; */

        return Ok(descriptions);
    }
}
