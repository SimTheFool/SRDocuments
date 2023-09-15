use std::collections::HashMap;

use serde_yaml::{self, Value};

pub mod test_infra;

static TEST_FILE: &str = "labeled_transform";

#[tokio::test]
async fn it_should_apply_labeled_transform_in_abcd_order() {
    let app = test_infra::get_test_app();
    let mut variables = HashMap::new();
    variables.insert("T_LAYER".to_string(), "t30".to_string());
    let yml = app
        .compile_and_validate_yml(TEST_FILE, None, Some(variables))
        .unwrap();

    match yml {
        Value::Mapping(m) => {
            let a = m.get(&Value::String("a".to_string()));
            assert_eq!(a.unwrap().as_i64().unwrap(), 937);
        }
        _ => panic!("Yml should be a mapping"),
    };
}
