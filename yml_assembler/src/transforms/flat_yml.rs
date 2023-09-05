use crate::{
    utils::result::{AppError, AppResult},
    App,
};
use serde_yaml::{Mapping, Sequence, Value};
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Debug)]
pub enum FlatYmlValue {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FlatYml(Vec<(String, FlatYmlValue)>);
impl Deref for FlatYml {
    type Target = Vec<(String, FlatYmlValue)>;

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
        FlatYml(vec![])
    }

    pub fn insert(&mut self, key: String, value: FlatYmlValue) {
        self.0.push((key, value));
    }
}

impl TryInto<Value> for FlatYml {
    type Error = AppError;

    fn try_into(self) -> Result<Value, Self::Error> {
        if self.is_empty() {
            return Ok(Value::Null);
        }

        let (first_key, first_value) = self.iter().next().unwrap();

        if first_key == "" {
            match first_value {
                FlatYmlValue::String(s) => return Ok(Value::String(s.clone())),
                FlatYmlValue::Number(n) => {
                    return Ok(Value::Number(serde_yaml::Number::from(n.clone())))
                }
                FlatYmlValue::Bool(b) => return Ok(Value::Bool(b.clone())),
                FlatYmlValue::Null => return Ok(Value::Null),
            }
        }

        let first_part = first_key
            .split('.')
            .next()
            .ok_or_else(|| AppError::ApplyFormula(format!("No key segment found")))?;

        let first_part_as_number = first_part.parse::<usize>();
        let first_part_as_string = first_part.parse::<String>();

        let mut yml = match (first_part_as_number, first_part_as_string) {
            (Ok(_), _) => Value::Sequence(Sequence::new()),
            (_, Ok(_)) => Value::Mapping(Mapping::new()),
            _ => Err(AppError::ApplyFormula(format!(
                "{first_part:?} is not a valid key",
            )))?,
        };

        for (key, value) in &*self {
            let mut current = &mut yml;
            let mut parts = key.split('.').peekable();
            let value = match value {
                FlatYmlValue::String(s) => Value::String(s.clone()),
                FlatYmlValue::Number(n) => Value::Number(serde_yaml::Number::from(n.clone())),
                FlatYmlValue::Bool(b) => Value::Bool(b.clone()),
                FlatYmlValue::Null => Value::Null,
            };

            while let Some(part) = parts.next() {
                enum NextPart {
                    Number(usize),
                    String(String),
                    None,
                }
                impl NextPart {
                    fn try_new(next_part: Option<&str>) -> AppResult<Self> {
                        match next_part {
                            Some(part) => match (part.parse::<usize>(), part.parse::<String>()) {
                                (Ok(i), _) => Ok(NextPart::Number(i)),
                                (_, Ok(s)) => Ok(NextPart::String(s)),
                                _ => Err(AppError::ApplyFormula(format!(
                                    "{next_part:?} is not a valid key",
                                )))?,
                            },
                            None => Ok(NextPart::None),
                        }
                    }

                    fn to_next_container_or_value(&self, val: &Value) -> Value {
                        match self {
                            NextPart::Number(_) => Value::Sequence(Sequence::new()),
                            NextPart::String(_) => Value::Mapping(Mapping::new()),
                            NextPart::None => val.clone(),
                        }
                    }
                }

                let next_part = NextPart::try_new(parts.peek().map(|x| *x))?;

                match current {
                    Value::Sequence(seq) => {
                        let index = part.parse::<usize>().map_err(|_| {
                            AppError::ApplyFormula(format!("Expected a number, got {part:?}"))
                        })?;
                        if index >= seq.len() {
                            seq.resize_with(index + 1, || Value::Null);
                        }
                        let entry = seq.get_mut(index).ok_or_else(|| {
                            AppError::ApplyFormula(format!("Nothing at {part:?}"))
                        })?;
                        match entry {
                            Value::Null => {
                                *entry = next_part.to_next_container_or_value(&value);
                            }
                            _ => {}
                        }
                        current = seq.get_mut(index).ok_or_else(|| {
                            AppError::ApplyFormula(format!("Can't get mutable at {part:?}"))
                        })?;
                    }
                    Value::Mapping(map) => {
                        let key = part.parse::<String>().map_err(|_| {
                            AppError::ApplyFormula(format!("Expected a string, got {part:?}"))
                        })?;
                        let entry = map.get(&key);
                        if let None = entry {
                            map.insert(
                                Value::String(key.clone()),
                                next_part.to_next_container_or_value(&value),
                            );
                        }
                        current = map.get_mut(key).unwrap();
                    }
                    _ => Err(AppError::ApplyFormula(format!(
                        "Can only insert something in mapping or sequence, got {current:?}"
                    )))?,
                }
            }
        }

        Ok(yml)
    }
}

impl TryFrom<Value> for FlatYml {
    type Error = AppError;

    fn try_from(value: Value) -> AppResult<Self> {
        fn visit(val: &Value, parent_key: &str) -> AppResult<FlatYml> {
            let mut flat_yml = FlatYml::new();
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
    use serde::Serialize;

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

    let get_value = |key: &str| &flat_yml.iter().find(|(k, _)| k == key).unwrap().1;

    assert_eq!(flat_yml.len(), 8);
    assert_eq!(
        get_value("structure.sub_entry"),
        &FlatYmlValue::String("I'm a sub entry".to_string())
    );
    assert_eq!(
        get_value("structure.sub_content.0"),
        &FlatYmlValue::String("I'm a sub content 0".to_string())
    );
    assert_eq!(
        get_value("structure.sub_content.1"),
        &FlatYmlValue::String("I'm a sub content 1".to_string())
    );
    assert_eq!(
        get_value("structure.sub_content.2"),
        &FlatYmlValue::String("I'm a sub content 2".to_string())
    );
    assert_eq!(get_value("structure.sub_flag"), &FlatYmlValue::Bool(false));
    assert_eq!(
        get_value("entry"),
        &FlatYmlValue::String("I'm an entry".to_string())
    );
    assert_eq!(
        get_value("content.0"),
        &FlatYmlValue::String("I'm a content 0".to_string())
    );
    assert_eq!(get_value("flag"), &FlatYmlValue::Bool(true));
}

#[test]
fn it_should_unflatten_hashmap_to_yml() {
    use serde::Serialize;

    let mut flat_yml = FlatYml::new();

    flat_yml.insert(
        "structure.sub_entry".to_string(),
        FlatYmlValue::String("I'm a sub entry".to_string()),
    );
    flat_yml.insert(
        "structure.sub_content.0".to_string(),
        FlatYmlValue::String("I'm a sub content 0".to_string()),
    );
    flat_yml.insert(
        "structure.sub_content.1".to_string(),
        FlatYmlValue::String("I'm a sub content 1".to_string()),
    );
    flat_yml.insert(
        "structure.sub_content.2".to_string(),
        FlatYmlValue::String("I'm a sub content 2".to_string()),
    );
    flat_yml.insert("structure.sub_flag".to_string(), FlatYmlValue::Bool(false));
    flat_yml.insert(
        "entry".to_string(),
        FlatYmlValue::String("I'm an entry".to_string()),
    );
    flat_yml.insert(
        "content.0".to_string(),
        FlatYmlValue::String("I'm a content 0".to_string()),
    );
    flat_yml.insert("flag".to_string(), FlatYmlValue::Bool(true));

    let yml: Value = flat_yml.try_into().unwrap();

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

    assert_eq!(yml, test_yml);
}
