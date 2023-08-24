use data_files::{infra_yml::read_yml_description::YmlDescriptionReader, App};
use std::path::PathBuf;

pub fn get_test_infra(root: &str) -> YmlDescriptionReader {
    YmlDescriptionReader::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/{root}")),
    )
}

pub fn get_test_app() -> App {
    App::new(Box::new(get_test_infra("yml_character")))
}
