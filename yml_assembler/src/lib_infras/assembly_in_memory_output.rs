use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    adapters::{AssemblyOutputFormat, AssemblyOutputPort},
    utils::result::AppResult,
};

pub struct AssemblyIMOutput {
    pub value_yml: Rc<RefCell<Option<serde_yaml::Value>>>,
    pub value_json: Rc<RefCell<Option<serde_json::Value>>>,
}
impl AssemblyIMOutput {
    pub fn new() -> Self {
        AssemblyIMOutput {
            value_yml: Rc::new(RefCell::new(None)),
            value_json: Rc::new(RefCell::new(None)),
        }
    }
    pub fn get_yml_output(&self) -> Option<serde_yaml::Value> {
        let in_memory_ref = self.value_yml.borrow();
        in_memory_ref.clone()
    }
    pub fn get_json_output(&self) -> Option<serde_json::Value> {
        let in_memory_ref = self.value_json.borrow();
        in_memory_ref.clone()
    }
}
impl AssemblyOutputPort for AssemblyIMOutput {
    fn output(
        &self,
        value: serde_yaml::Value,
        _: &PathBuf,
        format: &AssemblyOutputFormat,
    ) -> AppResult<()> {
        match format {
            AssemblyOutputFormat::Yml => {
                let mut in_memory_ref = self.value_yml.borrow_mut();
                *in_memory_ref = Some(value.clone());
            }
            AssemblyOutputFormat::Json => {
                let mut in_memory_ref = self.value_json.borrow_mut();
                let json = serde_json::to_value(value).map_err(|e| {
                    anyhow::anyhow!(format!("Could not transform yml to json: {}", e))
                })?;
                *in_memory_ref = Some(json);
            }
        }

        Ok(())
    }
}
