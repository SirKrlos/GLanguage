pub const SPACES: [&str; 4] = ["\r", "\n", "\t", " "];
pub const PUNCTUATIONS: [&str; 1] = [";"];
pub const DIGITS: &str = "0123456789";
pub const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
pub const LETTERS_DIGITS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

pub fn is_token(v: &str) -> bool {
	return if SPACES.contains(&v) { true } else { false };
}

pub enum Tokens {
	EOF,
}

impl Tokens {
	pub fn copy(&self) -> Tokens {
		match &self {
			Tokens::EOF => Tokens::EOF,
		}
	}
}
