use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{adapters::SchemaOutputPort, utils::result::AppResult};

pub struct SchemaIMOutput {
    pub value: Rc<RefCell<Option<serde_json::Value>>>,
}
impl SchemaIMOutput {
    pub fn new() -> Self {
        SchemaIMOutput {
            value: Rc::new(RefCell::new(None)),
        }
    }

    pub fn get_output(&self) -> Option<serde_json::Value> {
        let in_memory_ref = self.value.borrow();
        in_memory_ref.clone()
    }
}
impl SchemaOutputPort for SchemaIMOutput {
    fn output(&self, value: &serde_json::Value, _: &PathBuf) -> AppResult<()> {
        let mut in_memory_ref = self.value.borrow_mut();
        *in_memory_ref = Some(serde_json::from_value(value.clone()).unwrap());
        Ok(())
    }
}
