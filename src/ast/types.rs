
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	List(Vec<Expression>),
	Symbol(String),
	Number(i32),
	Builtin(fn(&[Expression]) -> Result<Expression, String>),
	Fn(Rc<Expression>, Rc<Expression>),
}

