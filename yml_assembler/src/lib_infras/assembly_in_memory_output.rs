use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{adapters::AssemblyOutputPort, utils::result::AppResult};

pub struct AssemblyIMOutput {
    pub value: Rc<RefCell<Option<serde_yaml::Value>>>,
}
impl AssemblyIMOutput {
    pub fn new() -> Self {
        AssemblyIMOutput {
            value: Rc::new(RefCell::new(None)),
        }
    }
    pub fn get_output(&self) -> Option<serde_yaml::Value> {
        let in_memory_ref = self.value.borrow();
        in_memory_ref.clone()
    }
}
impl AssemblyOutputPort for AssemblyIMOutput {
    fn output(&self, value: &serde_yaml::Value, _: &PathBuf) -> AppResult<()> {
        let mut in_memory_ref = self.value.borrow_mut();
        *in_memory_ref = Some(value.clone());
        Ok(())
    }
}
