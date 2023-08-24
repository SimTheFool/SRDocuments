pub mod character;
pub mod descriptions;
pub mod transformation;

#[test]
fn should_apply_transformations() {
    use crate::domain::character::Character;
    use crate::domain::descriptions::CharacterDescription;
    use crate::domain::descriptions::SpecializationDescription;
    use crate::domain::transformation::{Operation, Property, Transformation};

    let wicca = SpecializationDescription {
        name: "Wicca".to_string(),
        description: "A mage is a spellcaster".to_string(),
        transform: vec![Transformation {
            property: Property::Magic,
            operation: Operation::Add(6),
        }],
    };

    let description = CharacterDescription {
        con: 10,
        wil: 10,
        str: 10,
        specializations: vec![wicca],
        ..CharacterDescription::default()
    };

    let character: Character = description.into();

    assert_eq!(character.con, (10, None));
    assert_eq!(character.wil, (10, None));
    assert_eq!(character.magic, Some(6));
    assert_eq!(character.effects.len(), 1);
    let effect = &character.effects[0];
    assert_eq!(effect.name, "Wicca");
    assert_eq!(effect.description, "A mage is a spellcaster");
}
