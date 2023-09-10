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

#[ignore = "reason"]
#[tokio::test]
async fn it_should_mix_on_exisiting_property() {
    /* let app = test_infra::get_test_app();
    let yml = app.compile_and_validate_yml(TEST_FILE, None).unwrap();

    let book: DataFromYml = serde_yaml::from_value(yml).unwrap();

    assert_eq!(book.tags, vec!["childhood", "adult"]);
    assert_eq!(book.covers.len(), 2);
    assert_eq!(book.covers[0].color, "green");
    assert_eq!(book.covers[0].size, 41.0);
    assert_eq!(book.covers[1].color, "rose");
    assert_eq!(book.covers[0].size, 15.0); */

    panic!("TODO");
}
