use crate::gl_token::Token;
use crate::gl_token_position::TokenPosition;

pub struct Exception {
	pub token: Token,
}

impl Exception {
	pub fn new(token: Token) -> Exception {
		return Exception { token };
	}
}
