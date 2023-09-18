use crate::utils::result::AppResult;
use std::path::PathBuf;

pub trait PartReaderPort {
    fn get_value(&self, identifier: &str) -> AppResult<serde_yaml::Value>;
}

pub trait SchemaReaderPort {
    fn get_validation_schema(&self, identifier: &str) -> AppResult<serde_json::Value>;

    fn get_schema_from_yml(&self, path: &PathBuf) -> AppResult<serde_json::Value>;

    fn get_schema_from_json(&self, path: &PathBuf) -> AppResult<serde_json::Value>;
}

pub trait AssemblyOutputPort {
    fn output(&self, value: &serde_yaml::Value, file_path: &PathBuf) -> AppResult<()>;
}

pub trait SchemaOutputPort {
    fn output(&self, value: &serde_json::Value, schema_path: &PathBuf) -> AppResult<()>;
}
