use data_files::{infra_yml::read_yml_description::YmlDescriptionReader, App};
use std::path::PathBuf;

pub fn get_test_infra() -> YmlDescriptionReader {
    YmlDescriptionReader::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/yml_fixtures"))
}

pub fn get_test_app() -> App {
    App::new(Box::new(get_test_infra()))
}
