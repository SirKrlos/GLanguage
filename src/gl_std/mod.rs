mod gl_std_integer;
pub use gl_std_integer::GLInteger;
use crate::gl_env::Env;

pub enum Object {
	Null,
	Integer(GLInteger),
	String(String),
}

impl Object {
	pub fn copy(&self) -> Object {
		match &self {
			Object::Null => Object::Null,
			Object::Integer(integer) => Object::Integer(GLInteger::from_string(integer.to_string())),
			Object::String(string) => Object::String(String::from(string)),
		}
	}
}

impl std::fmt::Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match &self {
			Object::Null => write!(f, "Null"),
			Object::Integer(integer) => write!(f, "{}", integer.to_string()),
			Object::String(string) => write!(f, "{}", &string.to_string()),
		}
	}
}

pub fn load_std_to_env(env: &mut Env) {
	env.set(&String::from("Null"), Object::Null);
}
