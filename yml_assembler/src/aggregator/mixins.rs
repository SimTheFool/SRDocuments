use crate::{
    utils::result::{AppError, AppResult},
    App,
};
use serde_yaml::{
    value::{Tag, TaggedValue},
    Mapping, Value,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct MixIns(HashMap<String, Vec<Value>>);
impl Deref for MixIns {
    type Target = HashMap<String, Vec<Value>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MixIns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl MixIns {
    const MIX_TAG: &'static str = "!mix";

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn merge(&mut self, other: &Self) {
        other.iter().for_each(|(key, value)| {
            self.add(key.clone(), value.clone());
        });
    }

    pub fn add(&mut self, key: String, value: Vec<Value>) {
        let entry = self.entry(key).or_insert_with(Vec::new);
        entry.append(&mut value.clone());
    }

    pub fn inject(&self, injected: &Value) -> AppResult<Value> {
        fn merge_values(val_base: &Value, val_mix: &Value) -> AppResult<Value> {
            let val_base = val_base.clone();
            let val_mix = val_mix.clone();

            let val_mixed: Value = match (val_base, val_mix) {
                (Value::Null, val_mix) => val_mix,
                (val_base, Value::Null) => val_base,
                (Value::Mapping(mut val_base), Value::Mapping(val_mix)) => {
                    val_base.extend(val_mix);
                    Value::Mapping(val_base)
                }
                (Value::Sequence(mut val_base), Value::Sequence(val_mix)) => {
                    val_base.extend(val_mix);
                    Value::Sequence(val_base)
                }
                (Value::Sequence(val_base), Value::Mapping(val_mix)) => {
                    Err(AppError::ParseYml(format!(
                        "Cannot mix a mapping value into a sequence
                        val_base: {val_base:#?}
                        val_mix: {val_mix:#?}
                        "
                    )))?
                }
                (Value::Sequence(mut val_base), val_mix) => {
                    val_base.push(val_mix);
                    Value::Sequence(val_base)
                }
                (val_base, Value::Sequence(mut val_mix)) => {
                    val_mix.push(val_base);
                    Value::Sequence(val_mix)
                }
                (val_base, val_mix) => Value::Sequence(vec![val_base, val_mix]),
            };

            Ok(val_mixed)
        }

        let val = match (injected, self) {
            (yml, mixins) if mixins.is_empty() => Ok(yml.clone()),
            (Value::Mapping(map), mixins) => {
                let mut new_map = map.clone();
                mixins.iter().try_for_each(
                    |(key_to_inject, values_to_inject)| -> AppResult<()> {
                        let injected_entry =
                            new_map.get(&key_to_inject).unwrap_or(&Value::Null).clone();

                        let final_value: Value = values_to_inject.iter().try_fold(
                            injected_entry,
                            |injected_entry, value_to_inject| {
                                merge_values(&injected_entry, &value_to_inject)
                            },
                        )?;

                        new_map.insert(Value::String(key_to_inject.clone()), final_value);
                        Ok(())
                    },
                )?;

                Ok(Value::Mapping(new_map))
            }
            _ => Err(AppError::ParseYml(format!(
                "Yml root should be a map of key-value pairs if you want to use !mix tag"
            ))),
        }?;

        Ok(val)
    }

    pub fn trim(&mut self, val: &Value) -> AppResult<Value> {
        match val {
            Value::Tagged(t) => self.on_tag(t),
            Value::Mapping(map) => self.on_mapping(map),
            Value::Sequence(seq) => self.on_sequence(seq),
            x => Ok(x.clone()),
        }
    }

    fn on_tag(&mut self, val: &TaggedValue) -> AppResult<Value> {
        let tag = &val.tag.to_string();
        let value = &val.value;

        let yml = self.trim(value)?;
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(tag),
            value: yml,
        })))
    }

    fn on_sequence(&mut self, val: &Vec<Value>) -> AppResult<Value> {
        let mut new_seq: Vec<Value> = vec![];
        for value in val {
            let yml = self.trim(&value)?;

            if let Value::Null = yml {
                continue;
            }
            new_seq.push(yml)
        }
        if new_seq.is_empty() {
            return Ok(Value::Null);
        }
        Ok(Value::Sequence(new_seq))
    }

    fn on_mapping(&mut self, val: &Mapping) -> AppResult<Value> {
        let mut new_map = Mapping::new();
        for (key, value) in val {
            let value = match value {
                Value::Tagged(t) => {
                    let tag = &t.tag.to_string();
                    let value = &t.value;

                    match tag.starts_with(Self::MIX_TAG) {
                        true => {
                            let yml = self.trim(value)?;
                            let key = match key {
                                Value::String(key) => key,
                                _ => {
                                    return Err(AppError::ParseYml(format!(
                                        "Invalid key for mixin: {:?}",
                                        key
                                    )))
                                }
                            };

                            self.entry(key.clone()).or_insert(Vec::new()).push(yml);
                            Ok(Value::Null)
                        }
                        false => Ok(Value::Tagged(Box::new(TaggedValue {
                            tag: Tag::new(tag),
                            value: self.trim(value)?,
                        }))),
                    }
                }
                _ => self.trim(value),
            }?;

            if let Value::Null = value {
                continue;
            }
            let yml = self.trim(&value)?;
            new_map.insert(key.clone(), yml);
        }
        if new_map.is_empty() {
            return Ok(Value::Null);
        }
        Ok(Value::Mapping(new_map))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_mixin_yml() -> &'static str {
        r#"
            foo: abcde
            hue: !inc::hue
                a: 1
                b: 2
            bar: !mix
                my_mixin
            baz:
                bar: !mix
                    my_mixin_2
            toto: !mix
                tota: !mix
                    my_mixin_3
                totu: what
        "#
    }

    #[test]
    fn it_should_apply_trim_yml_mixins() {
        let yml_part: Value = serde_yaml::from_str(get_mixin_yml()).unwrap();
        let mut mixin = MixIns::new();

        mixin.trim(&yml_part).unwrap();

        assert_eq!(mixin.len(), 3);

        let barmixin = mixin.get("bar").unwrap();
        let bar_expected_value_1: Value = serde_yaml::from_str(
            r#"
                my_mixin
            "#,
        )
        .unwrap();
        let bar_expected_value_2: Value = serde_yaml::from_str(
            r#"
                my_mixin_2
            "#,
        )
        .unwrap();
        assert_eq!(barmixin, &vec![bar_expected_value_1, bar_expected_value_2]);

        let bazmixin = mixin.get("baz");
        assert_eq!(bazmixin, None);

        let toto = mixin.get("toto").unwrap();
        let toto_expected_value: Value = serde_yaml::from_str(
            r#"
                totu: what
            "#,
        )
        .unwrap();
        assert_eq!(toto, &vec![toto_expected_value]);

        let tota = mixin.get("tota").unwrap();
        let tota_expected_value: Value = serde_yaml::from_str(
            r#"
                my_mixin_3
            "#,
        )
        .unwrap();
        assert_eq!(tota, &vec![tota_expected_value]);
    }

    #[test]
    fn it_not_spread_sequence_into_several_mixins() {
        let yml_part: Value = serde_yaml::from_str(
            r#"
            hue: !mix
                - a: 1
                  b: 2
                - a: 3
                  b: 4
            "#,
        )
        .unwrap();
        let mut mixin = MixIns::new();
        mixin.trim(&yml_part).unwrap();

        let hue_mixin = mixin.get("hue").unwrap();
        let expected_hue_mixin: Vec<Value> = vec![serde_yaml::from_str(
            r#"
                - a: 1
                  b: 2
                - a: 3
                  b: 4
            "#,
        )
        .unwrap()];

        assert_eq!(hue_mixin, &expected_hue_mixin);
    }

    #[test]
    fn it_should_leave_non_mix_tags() {
        let yml_part: Value = serde_yaml::from_str(get_mixin_yml()).unwrap();
        let mut mixin = MixIns::new();

        let trimed_yml = mixin.trim(&yml_part).unwrap();
        let expected_yml: Value = serde_yaml::from_str(
            r#"
                foo: abcde
                hue: !inc::hue
                    a: 1
                    b: 2
            "#,
        )
        .unwrap();
        assert_eq!(trimed_yml, expected_yml);
    }

    #[test]
    fn it_should_mix_as_sequence_when_origin_is_leaf() {
        let root_yml: Value = serde_yaml::from_str(
            r#"
            toto: some_toto
        "#,
        )
        .unwrap();

        let yml_part: Value = serde_yaml::from_str(
            r#"
            toto: !mix my_mixin_3
        "#,
        )
        .unwrap();
        let mut mixin = MixIns::new();
        let trimed_yml = mixin.trim(&yml_part).unwrap();

        assert_eq!(trimed_yml, Value::Null);

        let injected_yml = mixin.inject(&root_yml).unwrap();
        let expected_yml: Value = serde_yaml::from_str(
            r#"
            toto:
                - some_toto
                - my_mixin_3
            "#,
        )
        .unwrap();

        assert_eq!(injected_yml, expected_yml);
    }

    #[test]
    fn it_should_mix_merging_mappings() {
        let root_yml: Value = serde_yaml::from_str(
            r#"
            toto:
                a: 1
                b: 2
        "#,
        )
        .unwrap();

        let yml_part: Value = serde_yaml::from_str(
            r#"
            toto: !mix
                c: 3
                d: 4
        "#,
        )
        .unwrap();
        let mut mixin = MixIns::new();
        let trimed_yml = mixin.trim(&yml_part).unwrap();

        assert_eq!(trimed_yml, Value::Null);

        let injected_yml = mixin.inject(&root_yml).unwrap();
        let expected_yml: Value = serde_yaml::from_str(
            r#"
            toto:
                a: 1
                b: 2
                c: 3
                d: 4
            "#,
        )
        .unwrap();

        assert_eq!(injected_yml, expected_yml);
    }
}
