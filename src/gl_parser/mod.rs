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
}
