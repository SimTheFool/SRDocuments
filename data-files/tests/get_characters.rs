pub mod test_infra;

#[tokio::test]
async fn it_should_read_base_stats() {
    let app = test_infra::get_test_app();

    let schrimp = &app
        .get_all_characters(vec!["characters/Schrimp".to_string()])
        .await
        .unwrap()[0];

    assert_eq!(schrimp.name, "Schrimp");
    assert_eq!(schrimp.description, "Humain - 13 ans - Technorigger");

    assert_eq!(schrimp.con, 1);
    assert_eq!(schrimp.agi, 4);
    assert_eq!(schrimp.rea, 2);
    assert_eq!(schrimp.str, 1);
    assert_eq!(schrimp.wil, 4);
    assert_eq!(schrimp.log, 5);
    assert_eq!(schrimp.int, 4);
    assert_eq!(schrimp.cha, 2);
    assert_eq!(schrimp.ess, 6);

    assert_eq!(schrimp.resonance, Some(7));
    assert_eq!(schrimp.submersion, Some(1));

    assert_eq!(schrimp.resist_drain, Some(3));
    assert_eq!(schrimp.resist_physical, 0);
    assert_eq!(schrimp.natural_healing, 5);

    assert_eq!(schrimp.init, 1);
    assert_eq!(schrimp.minor_actions, 2);
    assert_eq!(schrimp.edge, 4);

    assert_eq!(schrimp.physical_hit, 8);
    assert_eq!(schrimp.mental_hit, 10);
    assert_eq!(schrimp.over_hit, 1);
}