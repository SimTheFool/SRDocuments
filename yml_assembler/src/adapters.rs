use crate::utils::result::{AppError, AppResult};
use std::{env, path::PathBuf};

pub trait YmlReaderAdapter {
    fn get_value(&self, identifier: &str) -> AppResult<serde_yaml::Value>;
}

pub trait ValidationSchemaReaderAdapter {
    fn get_validation_schema(&self, identifier: &str) -> AppResult<serde_json::Value>;
}

/// Implems

pub struct YmlFileSystemReader {
    context: PathBuf,
}
impl YmlFileSystemReader {
    pub fn new(path: PathBuf) -> Self {
        YmlFileSystemReader { context: path }
    }
}
impl YmlReaderAdapter for YmlFileSystemReader {
    fn get_value(&self, identifier: &str) -> AppResult<serde_yaml::Value> {
        let path = self.context.join(format!("{identifier}"));
        let extension = path.extension();
        let path = match extension {
            Some(_) => path.clone(),
            None => path.with_extension("yml"),
        };
        println!("loading: {:?}", path);
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let yml: serde_yaml::Value = serde_yaml::from_reader(file).map_err(AppError::other)?;
        Ok(yml)
    }
}

pub struct ValidationSchemaFileSystemReader {
    context: PathBuf,
}
impl ValidationSchemaFileSystemReader {
    pub fn new(path: PathBuf) -> Self {
        ValidationSchemaFileSystemReader { context: path }
    }
}
impl ValidationSchemaReaderAdapter for ValidationSchemaFileSystemReader {
    fn get_validation_schema(&self, identifier: &str) -> AppResult<serde_json::Value> {
        let path = self.context.join(format!("{identifier}"));
        let extension = path.extension();
        let path = match extension {
            Some(_) => path.clone(),
            None => path.with_extension("json"),
        };
        println!("loading: {:?}", path);
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let schema: serde_json::Value = serde_json::from_reader(file).map_err(AppError::other)?;
        Ok(schema)
    }
}
