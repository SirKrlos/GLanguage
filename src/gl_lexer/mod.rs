use crate::gl_token::Token;
use crate::gl_token_position::TokenPosition;
use crate::gl_tokens::Tokens;

pub struct Lexer {
	filename: String,
	codetext: String,
	chars: Vec<char>,
	linestext: Vec<String>,
	position: TokenPosition,
	current_char: String,
	current_linetext: String,
	pub tokens: Vec<Token>,
}

impl Lexer {
	pub fn new(filename: String, codetext: String) -> Lexer {
		let mut chars: Vec<char> = codetext.chars().collect::<Vec<char>>();
		let mut linestext: Vec<String> = codetext.lines().map(|line| line.to_string()).collect::<Vec<String>>();
		let position: TokenPosition = TokenPosition::new(0, 0, 0);
		let current_char: String = if chars.len() > 0 { chars.remove(0).to_string() } else { String::new() };
		let current_linetext: String = if linestext.len() > 0 { linestext.remove(0) } else { String::from(&codetext) };
		let tokens: Vec<Token> = Vec::new();
		Lexer { filename, codetext, chars, linestext, position, current_char, current_linetext, tokens }
	}

	fn advance_position(&mut self) {
		self.position.index += 1;
		self.position.column += 1;
	}

	fn advance_char(&mut self) {
		if self.chars.len() > 0 {
			self.current_char = self.chars.remove(0).to_string();
		} else {
			self.current_char = String::new();
		}

		if self.current_char == "\n" {
			self.position.lineno += 1;
			if self.linestext.len() > 0 {
				self.current_linetext = self.linestext.remove(0);
			}
		}
	}

	fn advance(&mut self) {
		self.advance_position();
		self.advance_char();
	}

	fn build_new_token(&mut self, typer: Tokens, pos_start: TokenPosition) -> Token {
		let pos_end: TokenPosition = self.position.copy();
		let token: Token = Token::new(typer, String::from(&self.filename), String::from(&self.current_linetext), pos_start, pos_end);
		self.tokens.push(token.copy());
		token
	}

	fn illegal_char(&mut self) {
		let pos_start: TokenPosition = self.position.copy();
		self.advance();
		let token: Token = self.build_new_token(Tokens::EOF, pos_start);
		token.illegal_char();
	}

	fn make_next_token(&mut self) -> bool {
		let pos_start: TokenPosition = self.position.copy();

		if self.current_char.is_empty() {
			self.build_new_token(Tokens::EOF, pos_start);
			return false;
		} else {
			self.illegal_char();
			return true;
		}
	}

	pub fn run(&mut self) -> bool {
		if self.make_next_token() == true {
			return true;
		}
		return false;
	}
}
