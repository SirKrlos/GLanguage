use crate::gl_lexer::Lexer;
use crate::gl_parser::Parser;

pub struct RunTime {}

impl RunTime {
	pub fn new() -> RunTime {
		RunTime {}
	}

	pub fn run_codetext(&mut self, filename: String, codetext: String) -> bool {
		let mut lexer: Lexer = Lexer::new(String::from(filename), String::from(codetext));
		if lexer.run() == true {
			return true;
		}

		let mut parser: Parser = Parser::new(lexer.tokens);
		if parser.run() == true {
			return true;
		}

		return false;
	}
}
