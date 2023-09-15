use serde_yaml::{self, Value};

pub mod test_infra;

static TEST_FILE: &str = "null_values";

#[tokio::test]
async fn it_should_not_output_yml_null_leaves() {
    let app = test_infra::get_test_app();
    let yml = app.compile_and_validate_yml(TEST_FILE, None, None).unwrap();

    let story = match yml {
        Value::Mapping(m) => {
            let summary = m.get(&Value::String("summary".to_string()));
            assert!(summary.is_none());

            let story = m
                .get(&Value::String("story".to_string()))
                .expect("story should exist");
            story.clone()
        }
        _ => panic!("Yml should be a mapping"),
    };

    match story {
        Value::Mapping(m) => {
            let content = m.get(&Value::String("content".to_string()));
            assert!(content.is_none());

            let content_bis = m.get(&Value::String("content_bis".to_string()));
            assert!(content_bis.is_none());
        }
        _ => panic!("Story should be a mapping"),
    };
}
