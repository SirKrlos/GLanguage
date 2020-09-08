pub struct TokenPosition {
	pub index: u32,
	pub lineno: u32,
	pub column: u32,
}

impl TokenPosition {
	pub fn new(index: u32, lineno: u32, column: u32) -> TokenPosition {
		TokenPosition { index, lineno, column }
	}

	pub fn copy(&self) -> TokenPosition {
		TokenPosition::new(self.index, self.lineno, self.column)
	}
}
