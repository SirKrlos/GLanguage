use crate::gl_ast::Ast;
use crate::gl_statement::{Expression, Statement};
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

	fn build_new_ast(&self, value: String) -> Ast {
		Ast::new(String::from(value), String::from(&self.current_tok.filename), self.current_tok.position_start.lineno, String::from(&self.current_tok.linetext))
	}

	fn parse_expression(&mut self) -> (Expression, bool) {
		let expression = match self.current_tok.typer.copy() {
			Tokens::INTEGER(v) => {
				self.advance();
				Expression::Integer(self.build_new_ast(v))
			}
			Tokens::STRING(v) => {
				self.advance();
				Expression::String(self.build_new_ast(v))
			}
			Tokens::IDENTIFIER(v) => {
				self.advance();
				Expression::Identifier(self.build_new_ast(v))
			}
			_ => {
				self.current_tok.invalid_syntax(String::new());
				return (Expression::NULL, true);
			}
		};
		(expression, false)
	}

	fn parser_statement(&mut self) -> (Statement, bool) {
		let statement = match self.current_tok.typer.copy() {
			Tokens::EOF => (Statement::NULL),
			_ => {
				let (expr, error) = self.parse_expression();
				if error {
					return (Statement::NULL, true);
				}
				Statement::Expression(expr)
			}
		};

		match self.current_tok.typer.copy() {
			Tokens::SEMICOLON => {
				self.advance();
			}
			_ => {
				self.current_tok.expected_char(String::from(";"));
				return (Statement::NULL, true);
			}
		}
		return (statement, false);
	}

	pub fn run(&mut self) -> bool {
		loop {
			match self.current_tok.typer.copy() {
				Tokens::EOF => break,
				_ => {
					let (statement, error) = self.parser_statement();
					if error {
						return true;
					}
					self.asts.push(statement);
				}
			}
		}
		return false;
	}
}
