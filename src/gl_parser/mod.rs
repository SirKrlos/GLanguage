use crate::gl_statement::Statement;
use crate::gl_token::Token;

pub struct Parser {
	tokens: Vec<Token>,
	current_tok: Token,
	asts: Vec<Statement>,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Parser {
		let mut tokens: Vec<Token> = tokens;
		let current_tok: Token = tokens.remove(0);
		let asts: Vec<Statement> = Vec::new();
		Parser { tokens, current_tok, asts }
	}

	fn advance(&mut self) {
		if self.tokens.len() > 0 {
			self.current_tok = self.tokens.remove(0);
		}
	}

	pub fn run(&mut self) -> bool {
		return false;
	}
}
