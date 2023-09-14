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

        fn get_entry_to_mix_on<'a>(key: &str, val: &'a mut Value) -> AppResult<&'a mut Value> {
            let mut parts = key.split(".").into_iter();

            let mut val_to_be_mix_on: &mut Value = val;
            while let Some(part) = parts.next() {
                match part.parse::<usize>() {
                    Ok(index) => {
                        let val = val_to_be_mix_on.as_sequence_mut().ok_or_else(|| {
                            AppError::ParseYml(format!("Cannot find {part} in {key} mixin"))
                        })?;
                        if index >= val.len() {
                            val.resize_with(index + 1, || Value::Null);
                        }
                        val_to_be_mix_on = val.get_mut(index).unwrap();
                    }
                    Err(_) => {
                        let val = val_to_be_mix_on.as_mapping_mut().ok_or_else(|| {
                            AppError::ParseYml(format!("Cannot find {part} in {key} mixin"))
                        })?;
                        val_to_be_mix_on = val
                            .entry(Value::String(part.to_string()))
                            .or_insert(Value::Null);
                    }
                }
            }

            Ok(val_to_be_mix_on)
        }

        let val: AppResult<Value> = match (injected, self) {
            (yml, mixins) if mixins.is_empty() => Ok(yml.clone()),
            (yml, mixins) => {
                let mut yml = yml.clone();
                mixins.iter().try_for_each(
                    |(key_to_inject, values_to_inject)| -> AppResult<()> {
                        let entry_to_inject = get_entry_to_mix_on(key_to_inject, &mut yml)?;

                        let final_value: Value = values_to_inject.iter().try_fold(
                            entry_to_inject.clone(),
                            |entry_to_inject, value_to_inject| {
                                merge_values(&entry_to_inject, &value_to_inject)
                            },
                        )?;

                        *entry_to_inject = final_value.clone();
                        Ok(())
                    },
                )?;

                Ok(yml)
            }
        };

        val
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
        mixin.trim(&yml_part).unwrap();

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
        mixin.trim(&yml_part).unwrap();

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

    #[test]
    fn it_should_mix_compound_keys() {
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
            toto.a: !mix 3
        "#,
        )
        .unwrap();
        let mut mixin = MixIns::new();
        mixin.trim(&yml_part).unwrap();

        let injected_yml = mixin.inject(&root_yml).unwrap();
        let expected_yml: Value = serde_yaml::from_str(
            r#"
            toto:
                a:
                   - 1
                   - 3
                b: 2
            "#,
        )
        .unwrap();

        assert_eq!(injected_yml, expected_yml);
    }
}
