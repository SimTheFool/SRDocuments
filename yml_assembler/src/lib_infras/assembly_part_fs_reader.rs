use std::path::PathBuf;

use crate::{
    adapters::PartReaderPort,
    utils::result::{AppError, AppResult},
};

pub struct PartFSReader {
    context: PathBuf,
}
impl PartFSReader {
    pub fn new(path: PathBuf) -> Self {
        PartFSReader { context: path }
    }
}
impl PartReaderPort for PartFSReader {
    fn get_value(&self, identifier: &str) -> AppResult<serde_yaml::Value> {
        let path = self.context.join(format!("{identifier}.pyml"));
        println!("loading: {:?}", path);
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let yml: serde_yaml::Value = serde_yaml::from_reader(file).map_err(AppError::other)?;
        Ok(yml)
    }
}
