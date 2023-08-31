use serde_yaml::Mapping;
use serde_yaml::Value;
use std::rc::Rc;
use utils::result::AppResult;

pub mod adapters;
pub mod domain;
pub mod services_yml;
pub mod utils;

pub struct App {
    reader: Rc<dyn adapters::ReaderAdapter>,
}

impl App {
    pub fn new(reader: Rc<dyn adapters::ReaderAdapter>) -> Self {
        Self { reader }
    }

    pub async fn get_transformed_yml(&self, id: &str) -> AppResult<Value> {
        let yml_loader = services_yml::yml_loader::YmlLoader::new(Rc::clone(&self.reader));
        let yml_aggregator = services_yml::yml_aggregator::YmlAggregator::new(&yml_loader);
        let mut yml_mixer = services_yml::yml_mixer::YmlMixer::new();

        let yml = yml_loader.load(id, Mapping::new())?;
        let yml = yml_aggregator.visit(&yml)?;
        let yml = yml_mixer.mix(&yml)?;

        Ok(yml)
    }
}
