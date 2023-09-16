use crate::utils::result::{AppError, AppResult};
use std::path::PathBuf;

pub trait YmlReaderAdapter {
    fn get_value(&self, identifier: &str) -> AppResult<serde_yaml::Value>;
}

pub trait ValidationSchemaReaderAdapter {
    fn get_validation_schema(&self, identifier: &str) -> AppResult<serde_json::Value>;

    fn get_schema_from_yml(&self, path: &PathBuf) -> AppResult<serde_json::Value>;

    fn get_schema_from_json(&self, path: &PathBuf) -> AppResult<serde_json::Value>;
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
        let path = self.context.join(format!("{identifier}.pyml"));
        println!("loading: {:?}", path);
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let yml: serde_yaml::Value = serde_yaml::from_reader(file).map_err(AppError::other)?;
        Ok(yml)
    }
}

pub struct ValidationSchemaFileSystemReader {}
impl ValidationSchemaFileSystemReader {
    pub fn new() -> Self {
        ValidationSchemaFileSystemReader {}
    }
}
impl ValidationSchemaReaderAdapter for ValidationSchemaFileSystemReader {
    fn get_validation_schema(&self, path_str: &str) -> AppResult<serde_json::Value> {
        let path = PathBuf::from(path_str);
        let extension = path
            .extension()
            .ok_or_else(|| {
                AppError::FileSystem(format!(
                    "{path_str} has no extension, load either a json, yml or yaml file"
                ))
            })?
            .to_str()
            .ok_or_else(|| {
                AppError::FileSystem(format!(
                    "{path_str} filename probably contains invalid characters"
                ))
            })?;

        let schema = match extension {
            "json" => self.get_schema_from_json(&path)?,
            "yml" => self.get_schema_from_yml(&path)?,
            "yaml" => self.get_schema_from_yml(&path)?,
            _ => {
                return Err(AppError::FileSystem(format!(
                    "{path_str} has an invalid extension, load either a json, yml or yaml file"
                )))
            }
        };

        Ok(schema)
    }

    fn get_schema_from_json(&self, path: &PathBuf) -> AppResult<serde_json::Value> {
        println!("loading json schema: {:?}", path);
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let schema: serde_json::Value = serde_json::from_reader(file).map_err(AppError::other)?;
        Ok(schema)
    }

    fn get_schema_from_yml(&self, path: &PathBuf) -> AppResult<serde_json::Value> {
        println!("loading yml schema: {:?}", path);
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let schema: serde_json::Value = serde_yaml::from_reader(file).map_err(AppError::other)?;
        Ok(schema)
    }
}
