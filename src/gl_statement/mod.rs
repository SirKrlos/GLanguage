use crate::gl_ast::Ast;

pub enum Statement {
	NULL,
	Expression(Expression),
}

pub enum Expression {
	NULL,
	Integer(Ast),
	Identifier(Ast),
	String(Ast),
}

impl Statement {
	pub fn copy(&self) -> Statement {
		match &self {
			Statement::NULL => Statement::NULL,
			Statement::Expression(expr) => Statement::Expression(expr.copy()),
		}
	}
}

impl Expression {
	pub fn copy(&self) -> Expression {
		match &self {
			Expression::NULL => Expression::NULL,
			Expression::Integer(integer) => Expression::Integer(integer.copy()),
			Expression::String(string) => Expression::String(string.copy()),
			Expression::Identifier(identifier) => Expression::Identifier(identifier.copy()),
		}
	}
}
