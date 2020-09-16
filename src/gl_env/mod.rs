use crate::gl_std::Object;
use std::collections::HashMap;

pub struct Env {
	env: HashMap<String, Object>,
}

impl Env {
	pub fn new() -> Env {
		Env { env: std::collections::HashMap::new() }
	}

	pub fn set(&mut self, key: &String, value: Object) {
		self.env.insert(String::from(key), value);
	}

	pub fn get(&self, key: &String) -> Option<Object> {
		self.env.get(key).map(|value| value.copy())
	}

	pub fn exist(&self, key: &String) -> bool {
		self.env.contains_key(key)
	}
}
