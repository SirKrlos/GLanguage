use crate::gl_lexer::Lexer;
use crate::gl_parser::Parser;

pub struct RunTime {
	lineno: u32,
}

impl RunTime {
	pub fn new(lineno: u32) -> RunTime {
		RunTime { lineno }
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

		return false;
	}
}
