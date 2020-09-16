pub enum GLInteger {
	I8(i8),
	U8(u8),
	I16(i16),
	U16(u16),
	I32(i32),
	U32(u32),
	I64(i64),
	U64(u64),
	I128(i128),
	U128(u128),
	Overflow,
}

impl GLInteger {
	pub fn from_string(number_string: String) -> GLInteger {
		let number = number_string.parse::<i8>();
		if number.is_ok() {
			return GLInteger::I8(number.unwrap());
		}

		let number = number_string.parse::<u8>();
		if number.is_ok() {
			return GLInteger::U8(number.unwrap());
		}

		let number = number_string.parse::<i16>();
		if number.is_ok() {
			return GLInteger::I16(number.unwrap());
		}

		let number = number_string.parse::<u16>();
		if number.is_ok() {
			return GLInteger::U16(number.unwrap());
		}

		let number = number_string.parse::<i32>();
		if number.is_ok() {
			return GLInteger::I32(number.unwrap());
		}

		let number = number_string.parse::<u32>();
		if number.is_ok() {
			return GLInteger::U32(number.unwrap());
		}

		let number = number_string.parse::<i64>();
		if number.is_ok() {
			return GLInteger::I64(number.unwrap());
		}

		let number = number_string.parse::<u64>();
		if number.is_ok() {
			return GLInteger::U64(number.unwrap());
		}

		let number = number_string.parse::<i128>();
		if number.is_ok() {
			return GLInteger::I128(number.unwrap());
		}

		let number = number_string.parse::<u128>();
		if number.is_ok() {
			return GLInteger::U128(number.unwrap());
		} else {
			return GLInteger::Overflow;
		}
	}

	pub fn to_string(&self) -> String {
		match &self {
			GLInteger::I8(v) => v.to_string(),
			GLInteger::I16(v) => v.to_string(),
			GLInteger::I32(v) => v.to_string(),
			GLInteger::I64(v) => v.to_string(),
			GLInteger::I128(v) => v.to_string(),
			GLInteger::U8(v) => v.to_string(),
			GLInteger::U16(v) => v.to_string(),
			GLInteger::U32(v) => v.to_string(),
			GLInteger::U64(v) => v.to_string(),
			GLInteger::U128(v) => v.to_string(),
			GLInteger::Overflow => String::new(),
		}
	}
}
