pub const SPACES: [&str; 4] = ["\r", "\n", "\t", " "];
pub const PUNCTUATIONS: [&str; 6] = [";", "(", ")", ",", "{", "}"];
pub const DIGITS: &str = "0123456789";
pub const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
pub const LETTERS_DIGITS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";
pub const KEYWORDS: [&str; 1] = ["fn"];

pub fn is_token(v: &str) -> bool {
	return if SPACES.contains(&v) || PUNCTUATIONS.contains(&v) || DIGITS.contains(&v) || LETTERS.contains(&v) { true } else { false };
}

pub enum Keywords {
	FN,
}

pub enum Tokens {
	INTEGER(String),
	STRING(String),
	IDENTIFIER(String),
	KEYWORD(Keywords),
	EOF,
	SEMICOLON,
	LPAREN,
	RPAREN,
	COMMA,
	LBRACE,
	RBRACE,
}

impl Tokens {
	pub fn copy(&self) -> Tokens {
		match &self {
			Tokens::INTEGER(v) => Tokens::INTEGER(String::from(v)),
			Tokens::STRING(v) => Tokens::STRING(String::from(v)),
			Tokens::IDENTIFIER(v) => Tokens::IDENTIFIER(String::from(v)),
			Tokens::KEYWORD(v) => match &v {
				Keywords::FN => Tokens::KEYWORD(Keywords::FN),
			},
			Tokens::EOF => Tokens::EOF,
			Tokens::SEMICOLON => Tokens::SEMICOLON,
			Tokens::LPAREN => Tokens::LPAREN,
			Tokens::RPAREN => Tokens::RPAREN,
			Tokens::COMMA => Tokens::COMMA,
			Tokens::LBRACE => Tokens::LBRACE,
			Tokens::RBRACE => Tokens::RBRACE,
		}
	}
}
