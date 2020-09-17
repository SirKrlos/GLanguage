use crate::gl_env::Env;
use crate::gl_exception::Exception;
use crate::gl_statement::{Expression, Statement};
use crate::gl_std::{GLInteger, Object};

pub struct Eval {}

impl Eval {
	pub fn new() -> Eval {
		Eval {}
	}

	fn eval_expression(&self, expression: Expression, env: &mut Env) -> (Object, bool) {
		match expression {
			Expression::NULL => (Object::Null, false),
			Expression::Integer(integer) => {
				let obj_integer: GLInteger = GLInteger::from_string(integer.value);
				match obj_integer {
					GLInteger::Overflow => {
						Exception::run_time(integer.filename, integer.lineno, integer.linetext, "Overflow: integer literal is too large".to_string());
						return (Object::Null, true);
					}
					_ => (),
				}
				(Object::Integer(obj_integer), false)
			}
			Expression::String(string) => (Object::String(String::from(&string.value)), false),
			Expression::Identifier(identifier) => {
				if env.exist(&identifier.value) {
					return (env.get(&identifier.value).expect(""), false);
				} else {
					Exception::run_time(
						identifier.filename,
						identifier.lineno,
						identifier.linetext,
						String::from(format!("NameError: name '{}' is not defined", identifier.value)),
					);
					return (Object::Null, true);
				}
			}
		}
	}
}
