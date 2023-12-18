
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Token {
	Open,
	Close,
	Symbol(String),
	Number(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	List(Vec<Expression>),
	Symbol(String),
	Number(i32),
	Builtin(String, fn(&[Expression]) -> Result<Expression, String>),
	Fn(Rc<Expression>, Rc<Expression>),
}

