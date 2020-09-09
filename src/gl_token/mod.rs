use crate::gl_token_position::TokenPosition;
use crate::gl_tokens::Tokens;

pub struct Token {
	typer: Tokens,
	filename: String,
	linetext: String,
	position_start: TokenPosition,
	position_end: TokenPosition,
}

impl Token {
	pub fn new(typer: Tokens, filename: String, linetext: String, position_start: TokenPosition, position_end: TokenPosition) -> Token {
		Token { typer, filename, linetext, position_start, position_end }
	}

	pub fn copy(&self) -> Token {
		Token::new(self.typer.copy(), String::from(&self.filename), String::from(&self.linetext), self.position_start.copy(), self.position_end.copy())
	}
}
