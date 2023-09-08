use serde_yaml;

pub mod test_infra;

#[derive(Debug, serde::Deserialize)]
struct BookFromYml {
    title: String,
    summary: String,
    story: StoryFromYml,
    covers: Vec<CoverFromYml>,
    tags: Vec<String>,
    page: PagesFromYml,
}

#[derive(Debug, serde::Deserialize)]
struct PagesFromYml {
    number: f64,
    weight: f64,
}

#[derive(Debug, serde::Deserialize)]
struct StoryFromYml {
    content: String,
    chapter: f64,
}

#[derive(Debug, serde::Deserialize)]
struct CoverFromYml {
    color: String,
    size: f64,
}

static TEST_FILE: &str = "jul_21";

#[tokio::test]
async fn it_should_aggregate_filesystem_yml_files() {
    let app = test_infra::get_test_app();
    let yml = app.compile_and_validate_yml(TEST_FILE, None).unwrap();

    let book: BookFromYml = serde_yaml::from_value(yml).unwrap();

    assert_eq!(book.title, "Juliette coupe le gateau");
    assert_eq!(book.summary, "L'anniversaire de Juliette tourne mal");
    assert_eq!(book.story.content, "Ca y est ! Elle a 21 ans, et a invité tout le monde à pré coustille. Malheureusement Juliette n'est pas très adroite et se coupe le doigt en coupant le gâteau. Elle est emmenée à l'hôpital et se fait recoudre le doigt. Elle est très déçue de rater sa fête d'anniversaire.");
    assert_eq!(book.story.chapter, 5.0);
}

#[tokio::test]
async fn it_should_mix_properties() {
    let app = test_infra::get_test_app();
    let yml = app.compile_and_validate_yml(TEST_FILE, None).unwrap();

    let book: BookFromYml = serde_yaml::from_value(yml).unwrap();

    assert_eq!(book.covers.len(), 4);
    assert_eq!(book.tags.len(), 3);

    assert_eq!(book.covers[0].color, "yellow");
    assert_eq!(book.covers[0].size, 36 as f64);
    assert_eq!(book.covers[1].color, "rose");
    assert_eq!(book.covers[1].size, 15 as f64);
    assert_eq!(book.covers[2].color, "red");
    assert_eq!(book.covers[2].size, 10 as f64);
    assert_eq!(book.covers[3].color, "black");
    assert_eq!(book.covers[3].size, 20 as f64);

    assert_eq!(book.tags[0], "ivestigation");
    assert_eq!(book.tags[1], "adult");
    assert_eq!(book.tags[2], "horror");
}

#[tokio::test]
async fn it_should_validate_from_json() {
    let app = test_infra::get_test_app();
    app.compile_and_validate_yml(TEST_FILE, Some("book-schema.json"))
        .unwrap();
}

#[tokio::test]
async fn it_should_validate_from_yml() {
    let app = test_infra::get_test_app();
    app.compile_and_validate_yml(TEST_FILE, Some("book-schema.yml"))
        .unwrap();
}

#[tokio::test]
async fn it_should_transform_properties() {
    let app = test_infra::get_test_app();
    let yml = app.compile_and_validate_yml(TEST_FILE, None).unwrap();

    let book: BookFromYml = serde_yaml::from_value(yml).unwrap();

    assert_eq!(book.page.number, 40 as f64);
    assert_eq!(book.page.weight, 10.0);
}
