use data_files::adapters::ReadDescriptionsAdapter;
use data_files::domain::transformation::{Operation, Property};

pub mod test_infra;

#[tokio::test]
async fn it_should_read_description_from_filesystem() {
    let reader = test_infra::get_test_infra();

    let descriptions = reader
        .get_characters_descriptions(vec!["Aragola".to_string()])
        .await
        .unwrap();

    assert_eq!(descriptions.len(), 1);
    let description = &descriptions[0];
    assert_eq!(description.constitution, 5);
    assert_eq!(description.willpower, 3);
    assert_eq!(description.strength, 4);
    assert_eq!(description.magic, None);
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
