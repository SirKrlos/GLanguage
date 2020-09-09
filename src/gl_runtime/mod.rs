use crate::gl_lexer::Lexer;

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
		return false;
	}
}
