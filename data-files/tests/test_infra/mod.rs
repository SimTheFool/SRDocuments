use data_files::{
    adapters::ReaderAdapter,
    utils::result::{AppError, AppResult},
    App,
};
use serde_yaml::Value;
use std::{path::PathBuf, rc::Rc};

pub struct TestFileSystemReader {
    yml_filesystem_path: PathBuf,
}
impl TestFileSystemReader {
    pub fn new(path: PathBuf) -> Self {
        TestFileSystemReader {
            yml_filesystem_path: path,
        }
    }
}
impl ReaderAdapter for TestFileSystemReader {
    fn get_value(&self, identifier: &str) -> AppResult<Value> {
        let path = self.yml_filesystem_path.join(format!("{identifier}.yml"));
        let file = std::fs::File::open(path).map_err(AppError::other)?;
        let yml: Value = serde_yaml::from_reader(file).map_err(AppError::other)?;
        Ok(yml)
    }
}

pub fn get_test_infra(root: &str) -> Rc<TestFileSystemReader> {
    Rc::new(TestFileSystemReader::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/{root}")),
    ))
}

pub fn get_test_app() -> App {
    let reader = get_test_infra("yml_test_files");
    App::new(reader)
}
