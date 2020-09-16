use crate::gl_exception::Exception;
use crate::gl_token_position::TokenPosition;
use crate::gl_tokens::Tokens;

pub struct Token {
	pub typer: Tokens,
	pub filename: String,
	pub linetext: String,
	pub position_start: TokenPosition,
	pub position_end: TokenPosition,
}

impl Token {
	pub fn new(typer: Tokens, filename: String, linetext: String, position_start: TokenPosition, position_end: TokenPosition) -> Token {
		Token { typer, filename, linetext, position_start, position_end }
	}

	pub fn copy(&self) -> Token {
		Token::new(self.typer.copy(), String::from(&self.filename), String::from(&self.linetext), self.position_start.copy(), self.position_end.copy())
	}

	pub fn illegal_char(&self) {
		let exception: Exception = Exception::new(self.copy());
		exception.illegal_char();
	}

	pub fn invalid_syntax(&self, details: String) {
		let exception: Exception = Exception::new(self.copy());
		exception.invalid_syntax(details);
	}

	pub fn expected_char(&self, character: String) {
		let exception: Exception = Exception::new(self.copy());
		exception.expected_char(character);
	}

	pub fn run_time(&self, error_details: String) {
		let exception: Exception = Exception::new(self.copy());
		exception.run_time(error_details);
	}
}
