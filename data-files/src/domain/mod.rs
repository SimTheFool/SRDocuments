pub mod character;
pub mod descriptions;

#[test]
fn should_apply_transformations() {
    use crate::domain::character::Character;
    use crate::domain::character::CharacterEffect;
    use crate::domain::descriptions::SpecializationDescription;
    use crate::domain::descriptions::{CharacterDescription, CharacterTransformation};

    let wicca = SpecializationDescription {
        transform: CharacterTransformation {
            transform: Box::new(|character| Character {
                magic: Some(6),
                effects: {
                    let mut effects = character.effects.clone();
                    effects.push(CharacterEffect {
                        name: "Wicca".to_string(),
                        description: "A mage is a spellcaster".to_string(),
                    });
                    effects
                },
                ..character
            }),
        },
    };

    let description = CharacterDescription {
        constitution: 10,
        willpower: 10,
        strength: 10,
        magic: None,
        spec_descriptions: vec![wicca],
    };

    let character = description.apply_transformations();

    assert_eq!(character.constitution, 10);
    assert_eq!(character.willpower, 10);
    assert_eq!(character.magic, Some(6));
    assert_eq!(character.effects.len(), 1);
    let effect = &character.effects[0];
    assert_eq!(effect.name, "Wicca");
    assert_eq!(effect.description, "A mage is a spellcaster");
}

/* #[test]
fn should_add_effects() {} */
