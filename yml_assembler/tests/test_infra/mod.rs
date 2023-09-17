use std::{path::PathBuf, rc::Rc};
use yml_assembler::{
    adapters::{ValidationSchemaFileSystemReader, YmlFileSystemReader},
    App,
};

pub fn get_test_infra(
    root: &str,
) -> (
    Rc<YmlFileSystemReader>,
    Rc<ValidationSchemaFileSystemReader>,
) {
    let yml_reader = Rc::new(YmlFileSystemReader::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/{root}")),
    ));
    let schema_reader = Rc::new(ValidationSchemaFileSystemReader::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/{root}")),
    ));
    (yml_reader, schema_reader)
}

pub fn get_test_app() -> App {
    let (yml_reader, schema_reader) = get_test_infra("yml_test_files");
    App::new(yml_reader, schema_reader)
}
