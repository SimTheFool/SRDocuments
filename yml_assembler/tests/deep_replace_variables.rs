use serde_yaml;

pub mod test_infra;

#[derive(Debug, serde::Deserialize)]
struct DataFromYml {
    content: String,
    content_bis: String,
    chapter: Vec<u8>,
}

static TEST_FILE: &str = "deep_replace_variables";

#[tokio::test]
async fn it_should_deep_replace_variables() {
    let app = test_infra::get_test_app();
    let (yml, _) = app.compile_and_validate_yml(TEST_FILE, None, None).unwrap();
    let book: DataFromYml = serde_yaml::from_value(yml).unwrap();

    assert_eq!(book.content, "Some car crashed".to_string());
    assert_eq!(book.content_bis, "Some car crashed".to_string());
    assert!(book.chapter.contains(&2));
}

#[tokio::test]
async fn it_should_work_well_with_mixin() {
    let app = test_infra::get_test_app();
    let (yml, _) = app.compile_and_validate_yml(TEST_FILE, None, None).unwrap();
    let book: DataFromYml = serde_yaml::from_value(yml).unwrap();

    assert!(book.chapter.contains(&3));
    assert!(book.chapter.contains(&16));
}
