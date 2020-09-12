use crate::gl_statement::{Expression, Statement, ValueWithToken};
use crate::gl_token::Token;
use crate::gl_tokens::Tokens;

pub struct Parser {
	tokens: Vec<Token>,
	current_tok: Token,
	pub asts: Vec<Statement>,
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
		loop {
			match &self.current_tok.typer {
				Tokens::EOF => break,
				_ => self.parse_expression(),
			}
			match &self.current_tok.typer {
				Tokens::SEMICOLON => (),
				_ => {
					self.current_tok.expected_char(String::from(";"));
					return true;
				}
			}
			self.advance();
		}
		return false;
	}

	fn parse_expression(&mut self) {
		let expression = match &self.current_tok.typer {
			Tokens::INTEGER(v) => Expression::Integer(ValueWithToken { value: String::from(v), token: self.current_tok.copy() }),
			_ => {
				self.current_tok.invalid_syntax(String::new());
				return;
			}
		};
		self.advance();
		self.asts.push(Statement::Expression(expression));
	}
}
