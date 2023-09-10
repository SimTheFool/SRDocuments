use serde_yaml;

pub mod test_infra;

#[derive(Debug, serde::Deserialize)]
struct DataFromYml {
    tags: Vec<String>,
    covers: Vec<CoverFromYml>,
}

#[derive(Debug, serde::Deserialize)]
struct CoverFromYml {
    color: String,
    size: f64,
}

static TEST_FILE: &str = "mix_in_existing_key";

#[tokio::test]
async fn it_should_mix_on_exisiting_property() {
    let app = test_infra::get_test_app();
    let yml = app.compile_and_validate_yml(TEST_FILE, None).unwrap();

    let book: DataFromYml = serde_yaml::from_value(yml).unwrap();

    assert_eq!(book.tags.len(), 2);
    assert!(book.tags.contains(&"childhood".to_string()));
    assert!(book.tags.contains(&"adult".to_string()));

    assert_eq!(book.covers.len(), 2);
    let green_cover = book.covers.iter().find(|c| c.color == "green").unwrap();
    assert_eq!(green_cover.size, 41.0);
    let rose_cover = book.covers.iter().find(|c| c.color == "rose").unwrap();
    assert_eq!(rose_cover.size, 15.0);
}
