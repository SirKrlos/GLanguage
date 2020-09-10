use crate::gl_lexer::Lexer;
use crate::gl_parser::Parser;

pub struct RunTime {
	lineno: u32,
}

impl RunTime {
	pub fn new(lineno: u32) -> RunTime {
		RunTime { lineno: lineno }
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
