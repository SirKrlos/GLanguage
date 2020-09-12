use crate::gl_token::Token;

pub enum Statement {
	Expression(Expression),
}

pub enum Expression {
	Integer(ValueWithToken),
}

pub struct ValueWithToken {
	pub value: String,
	pub token: Token,
}
