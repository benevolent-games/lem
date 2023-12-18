
#[derive(Debug, PartialEq)]
pub enum Token {
	Open,
	Close,
	Symbol(String),
	Number(i32),
}

