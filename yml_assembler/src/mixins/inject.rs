use super::MixIns;
use crate::utils::result::{AppError, AppResult};
use serde_yaml::Value;

impl MixIns {
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
}

#[cfg(test)]
mod test {
    use super::*;

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
