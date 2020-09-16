use crate::gl_token::Token;

pub struct Exception {
	pub token: Token,
}

impl Exception {
	pub fn new(token: Token) -> Exception {
		return Exception { token };
	}

	fn generate_exception_string(&self, error_details: String) -> String {
		let mut linetext = String::from(&self.token.linetext);
		linetext = linetext.replace("\t", " ");
		let mut len_spaces_start: u32 = linetext.len() as u32;
		linetext = linetext.trim_start().to_string();
		len_spaces_start = len_spaces_start - linetext.len() as u32;

		if linetext.ends_with("\n") {
			linetext.remove(linetext.len() - 1);
		}
		if linetext.ends_with("\r") {
			linetext.remove(linetext.len() - 1);
		}

		linetext = linetext.trim_end().to_string();
		let pows: String = String::from("^");
		let mut spaces_start: String = String::new();
		let mut i: u32 = 0;
		while self.token.position_start.column > len_spaces_start && i < (self.token.position_start.column - len_spaces_start) {
			i += 1;
			spaces_start += " ";
		}

		let mut result: String = String::new();
		result.push_str(format!("  File \"{}\", line {}\n", self.token.filename, self.token.position_start.lineno + 1).as_str());
		result.push_str(format!("    {}\n", linetext).as_str());
		result.push_str(format!("    {}{}\n", spaces_start, pows).as_str());
		result.push_str(error_details.as_str());
		result
	}

	pub fn illegal_char(&self) {
		println!("{}", self.generate_exception_string("SyntaxError: Invalid character in identifier".to_string()));
	}

	pub fn invalid_syntax(&self, mut details: String) {
		if details.is_empty() {
			details = String::from("Invalid syntax");
		}
		println!("{}", self.generate_exception_string(format!("SyntaxError: {}", details)));
	}

	pub fn expected_char(&self, character: String) {
		println!("{}", self.generate_exception_string(format!("SyntaxError: Expected Character '{}'", character)));
	}

	pub fn run_time(&self, error_details: String) {
		let mut linetext = String::from(&self.token.linetext);
		linetext = linetext.replace("\t", " ");
		linetext = linetext.trim_start().to_string();

		if linetext.ends_with("\n") {
			linetext.remove(linetext.len() - 1);
		}
		if linetext.ends_with("\r") {
			linetext.remove(linetext.len() - 1);
		}

		linetext = linetext.trim_end().to_string();
		let mut result: String = String::from("Traceback (most recent call last):\n");
		result.push_str(format!("  File \"{}\", line {}, in <module>\n", self.token.filename, self.token.position_start.lineno + 1).as_str());
		result.push_str(format!("    {}\n", linetext).as_str());
		result.push_str(error_details.as_str());
		println!("{}", result);
	}
}
