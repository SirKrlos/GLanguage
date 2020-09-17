pub struct Ast {
	pub value: String,
	pub filename: String,
	pub lineno: u32,
	pub linetext: String,
}

impl Ast {
	pub fn new(value: String, filename: String, lineno: u32, linetext: String) -> Ast {
		Ast { value, filename, lineno, linetext }
	}

	pub fn copy(&self) -> Ast {
		let value: String = String::from(&self.value);
		let filename: String = String::from(&self.filename);
		let linetext: String = String::from(&self.linetext);
		Ast { value, filename, lineno: self.lineno, linetext }
	}
}
