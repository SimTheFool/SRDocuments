use crate::utils::result::{AppError, AppResult};
use serde::Serialize;
use serde_yaml::Value;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Clone, PartialEq, Debug)]
enum FlatYmlValue {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Clone, PartialEq, Debug)]
struct FlatYml(HashMap<String, FlatYmlValue>);
impl Deref for FlatYml {
    type Target = HashMap<String, FlatYmlValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FlatYml {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl FlatYml {
    pub fn new() -> Self {
        FlatYml(HashMap::new())
    }
}

impl TryFrom<Value> for FlatYml {
    type Error = AppError;

    fn try_from(value: Value) -> AppResult<Self> {
        fn visit(val: &Value, parent_key: &str) -> AppResult<FlatYml> {
            let mut flat_yml = FlatYml(HashMap::new());
            match val {
                Value::String(s) => {
                    flat_yml.insert(format!("{parent_key}"), FlatYmlValue::String(s.clone()));
                }
                Value::Bool(s) => {
                    flat_yml.insert(format!("{parent_key}"), FlatYmlValue::Bool(s.clone()));
                }
                Value::Number(s) => {
                    flat_yml.insert(
                        format!("{parent_key}"),
                        FlatYmlValue::Number(s.as_f64().ok_or_else(|| {
                            AppError::ApplyFormula(format!("Your numbers must be f64"))
                        })?),
                    );
                }
                Value::Null => {
                    flat_yml.insert(format!("{parent_key}"), FlatYmlValue::Null);
                }
                Value::Mapping(m) => {
                    for (key, v) in m {
                        let k = match key {
                            Value::String(str) => Ok(str),
                            _ => {
                                let key = key.clone();
                                Err(AppError::ApplyFormula(format!(
                                    "Mapping keys is not a string: {key:?}",
                                )))
                            }
                        }?;

                        let new_key = match parent_key {
                            "" => format!("{k}"),
                            _ => format!("{parent_key}.{k}"),
                        };
                        let child_flat_yml = visit(v, &new_key)?;
                        flat_yml.extend(child_flat_yml.iter().map(|(k, v)| (k.clone(), v.clone())));
                    }
                }
                Value::Sequence(seq) => {
                    for (i, v) in seq.iter().enumerate() {
                        let new_key = match parent_key {
                            "" => format!("{i}"),
                            _ => format!("{parent_key}.{i}"),
                        };
                        let child_flat_yml = visit(v, &new_key)?;
                        flat_yml.extend(child_flat_yml.iter().map(|(k, v)| (k.clone(), v.clone())));
                    }
                }
                Value::Tagged(t) => {
                    let tag = &t.tag;
                    Err(AppError::ApplyFormula(format!("Unhandle tag: {tag:?}",)))?;
                }
            }

            Ok(flat_yml)
        }

        let flat_yml = visit(&value, "")?;
        Ok(flat_yml)
    }
}

#[test]
fn it_should_flatten_yml_value() {
    #[derive(Debug, Serialize)]
    struct TestStruct<'a> {
        structure: SubStruct<'a>,
        entry: &'a str,
        content: Vec<&'a str>,
        flag: bool,
    }

    #[derive(Debug, Serialize)]
    struct SubStruct<'a> {
        sub_entry: &'a str,
        sub_content: Vec<&'a str>,
        sub_flag: bool,
    }

    let test_struct = TestStruct {
        structure: SubStruct {
            sub_entry: "I'm a sub entry",
            sub_content: vec![
                "I'm a sub content 0",
                "I'm a sub content 1",
                "I'm a sub content 2",
            ],
            sub_flag: false,
        },
        entry: "I'm an entry",
        content: vec!["I'm a content 0"],
        flag: true,
    };

    let test_yml = serde_yaml::to_value(&test_struct).unwrap();
    let flat_yml = FlatYml::try_from(test_yml).unwrap();

    assert_eq!(flat_yml.len(), 8);
    assert_eq!(
        flat_yml.get("structure.sub_entry").unwrap(),
        &FlatYmlValue::String("I'm a sub entry".to_string())
    );
    assert_eq!(
        flat_yml.get("structure.sub_content.0").unwrap(),
        &FlatYmlValue::String("I'm a sub content 0".to_string())
    );
    assert_eq!(
        flat_yml.get("structure.sub_content.1").unwrap(),
        &FlatYmlValue::String("I'm a sub content 1".to_string())
    );
    assert_eq!(
        flat_yml.get("structure.sub_content.2").unwrap(),
        &FlatYmlValue::String("I'm a sub content 2".to_string())
    );
    assert_eq!(
        flat_yml.get("structure.sub_flag").unwrap(),
        &FlatYmlValue::Bool(false)
    );
    assert_eq!(
        flat_yml.get("entry").unwrap(),
        &FlatYmlValue::String("I'm an entry".to_string())
    );
    assert_eq!(
        flat_yml.get("content.0").unwrap(),
        &FlatYmlValue::String("I'm a content 0".to_string())
    );
    assert_eq!(flat_yml.get("flag").unwrap(), &FlatYmlValue::Bool(true));
}
