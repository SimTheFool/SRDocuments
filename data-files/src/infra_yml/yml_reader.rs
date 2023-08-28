use crate::utils::result::{AppError, AppResult};
use serde_yaml::Value;
use std::path::PathBuf;

pub struct YmlReader {
    yml_filesystem_path: PathBuf,
}

impl YmlReader {
    pub fn new(path: PathBuf) -> Self {
        YmlReader {
            yml_filesystem_path: path,
        }
    }

    pub fn load(&self, path: &str) -> AppResult<Value> {
        let path = self.yml_filesystem_path.join(format!("{path}.yml"));
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let yml: Value = serde_yaml::from_reader(file).map_err(AppError::other)?;
        Ok(yml)
    }
}
