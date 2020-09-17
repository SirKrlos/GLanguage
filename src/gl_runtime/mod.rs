use crate::gl_env::Env;
use crate::gl_eval::Eval;
use crate::gl_lexer::Lexer;
use crate::gl_parser::Parser;
use crate::gl_std::{load_std_to_env, Object};

pub struct RunTime {
	lineno: u32,
	env: Env,
}

impl RunTime {
	pub fn new(lineno: u32) -> RunTime {
		let mut env: Env = Env::new();
		load_std_to_env(&mut env);
		RunTime { lineno, env }
	}

	pub fn run_codetext(&mut self, filename: String, codetext: String) -> bool {
		let mut lexer: Lexer = Lexer::new(String::from(filename), String::from(codetext), self.lineno);
		if lexer.run() == true {
			self.lineno = lexer.position.lineno;
			return true;
		}
		self.lineno = lexer.position.lineno;

		let mut parser: Parser = Parser::new(lexer.copy_tokens());
		if parser.run() == true {
			return true;
		}

		let eval: Eval = Eval::new();
		let results: Vec<Object> = eval.eval_return_scope(parser.asts, &mut self.env);
		for result in &results {
			match result {
				Object::Null => (),
				_ => println!("{}", result),
			}
		}

		return false;
	}
}
