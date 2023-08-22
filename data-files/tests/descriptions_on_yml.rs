use std::path::PathBuf;

use data_files::domain::transformation::{Operation, Property};
use data_files::infra::read_yml_description::YmlDescriptionReader;
use data_files::ReadDescriptionsAdapter;

#[tokio::test]
async fn it_should_read_description_from_filesystem() {
    let reader = YmlDescriptionReader::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/yml_fixtures"),
    );

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
