use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use crate::{
    adapters::PartReaderPort,
    utils::result::{AppError, AppResult},
};

pub struct PartFSReader {
    context: PathBuf,
    cache: RefCell<HashMap<String, serde_yaml::Value>>,
}
impl PartFSReader {
    pub fn new(path: PathBuf) -> Self {
        PartFSReader {
            context: path,
            cache: RefCell::new(HashMap::new()),
        }
    }
}
impl PartReaderPort for PartFSReader {
    fn get_value(&self, identifier: &str) -> AppResult<serde_yaml::Value> {
        let mut cache = self.cache.borrow_mut();
        let cached_value = cache.get(identifier);

        match cached_value {
            Some(value) => {
                println!("reading from cache: {}", identifier);
                Ok(value.clone())
            }
            None => {
                let path = self.context.join(format!("{identifier}.pyml"));
                println!("reading: {}", identifier);
                let file = std::fs::File::open(path).map_err(AppError::other)?;
                let yml: serde_yaml::Value =
                    serde_yaml::from_reader(file).map_err(AppError::other)?;
                cache.insert(identifier.to_string(), yml.clone());
                Ok(yml)
            }
        }
    }
}
