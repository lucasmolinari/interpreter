use super::object::Object;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Environment) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        let obj = self.store.get(name);
        obj.or_else(|| match &self.outer {
            Some(outer) => outer.get(name),
            None => None,
        })
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}
