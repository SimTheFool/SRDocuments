use data_files::adapters::ReadDescriptionsAdapter;
use data_files::domain::transformation::{Operation, Property};
use serde_yaml;

pub mod test_infra;

/* #[tokio::test]
async fn it_should_read_description_from_filesystem() {
    let reader = test_infra::get_test_infra();

    let descriptions = reader
        .get_characters_descriptions(vec!["Aragola".to_string()])
        .await
        .unwrap();

    assert_eq!(descriptions.len(), 1);
    let description = &descriptions[0];
    assert_eq!(description.con, 5);
    assert_eq!(description.wil, 3);
    assert_eq!(description.str, 4);
    assert_eq!(description.specializations.len(), 1);
    let spec_description = &description.specializations[0];
    assert_eq!(spec_description.name, "Tradition Wicca");
    assert_eq!(
        spec_description.description,
        "Vous êtes adepte Wicca. Vous révérez la nature, la lune et les animaux."
    );
    assert_eq!(spec_description.transform.len(), 1);
    let transformation = &spec_description.transform[0];
    assert_eq!(transformation.property, Property::Magic);
    assert_eq!(transformation.operation, Operation::Add(4));
}
 */

#[tokio::test]
async fn it_should_aggregate_filesystem_yml_files() {
    let reader = test_infra::get_test_infra("yml_other");
    let yml = reader.visit_from_path("jul_21").unwrap();

    #[derive(Debug, serde::Deserialize)]
    struct BookFromYml {
        title: String,
        summary: String,
        story: StoryFromYml,
    }

    #[derive(Debug, serde::Deserialize)]
    struct StoryFromYml {
        content: String,
        chapter: u8,
    }

    let book: BookFromYml = serde_yaml::from_value(yml).unwrap();

    assert_eq!(book.title, "Juliette coupe le gateau");
    assert_eq!(book.summary, "L'anniversaire de Juliette tourne mal");
    assert_eq!(book.story.content, "Ca y est ! Elle a 21 ans, et a invité tout le monde à pré coustille. Malheureusement Juliette n'est pas très adroite et se coupe le doigt en coupant le gâteau. Elle est emmenée à l'hôpital et se fait recoudre le doigt. Elle est très déçue de rater sa fête d'anniversaire.");
    assert_eq!(book.story.chapter, 5);
}
