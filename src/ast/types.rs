
#[derive(Debug, PartialEq)]
pub enum Expression {
	List(Vec<Expression>),
	Symbol(String),
	Number(i32),
}

