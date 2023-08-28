use data_files::{infra_yml::yml_visitors::YmlAggregatorVisitor, App};
use std::path::PathBuf;

pub fn get_test_infra(root: &str) -> Box<YmlAggregatorVisitor> {
    Box::new(YmlAggregatorVisitor::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/{root}")),
    ))
}

pub fn get_test_app() -> App {
    App::new(get_test_infra("yml_character"))
}
