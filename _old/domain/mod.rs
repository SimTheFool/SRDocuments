pub mod character;
pub mod descriptions;
pub mod transformation;

#[test]
fn should_apply_transformations() {
    use crate::domain::character::Character;
    use crate::domain::descriptions::CharacterDescription;
    use crate::domain::descriptions::SpecializationDescription;
    use crate::domain::transformation::Transformation;

    let wicca = SpecializationDescription {
        name: Some("Wicca".to_string()),
        description: Some("A mage is a spellcaster".to_string()),
        transform: vec![Transformation::new("MAG += 6")],
    };

    let description = CharacterDescription {
        con: 10,
        wil: 10,
        str: 10,
        int: 10,
        specializations: vec![wicca],
        ..CharacterDescription::default()
    };

    let character: Character = description.try_into().unwrap();

    assert_eq!(character.con, 10);
    assert_eq!(character.wil, 10);
    assert_eq!(character.magic, Some(6));
    assert_eq!(character.effects.len(), 1);
    let effect = &character.effects[0];
    assert_eq!(effect.name, "Wicca");
    assert_eq!(effect.description, "A mage is a spellcaster");
}
