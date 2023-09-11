use serde_yaml::Value;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

mod from_value;
mod inject;

#[derive(Clone)]
pub struct Variables(HashMap<String, Value>);
impl Deref for Variables {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Variables {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
