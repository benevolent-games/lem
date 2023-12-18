
use std::vec::IntoIter;
use crate::lex::Token;
use crate::ast::Expression;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expression>, String> {
	let mut expressions: Vec<Expression> = Vec::new();
	let mut token_iterator = tokens.into_iter();
	loop {
		match token_iterator.next() {
			Some(token) => expressions.push(
				parse_expression(token, &mut token_iterator)?
			),
			None => return Ok(expressions),
		};
	}
}

fn parse_expression(
		token: Token,
		mut token_iterator: &mut IntoIter<Token>,
	) -> Result<Expression, String> {

	match token {
		Token::Open => return Ok(parse_list(&mut token_iterator)?),
		Token::Symbol(s) => return Ok(Expression::Symbol(s)),
		Token::Number(n) => return Ok(Expression::Number(n)),
		Token::Close => return Err("unexpected close ')'".to_string()),
	};
}

fn parse_list(token_iterator: &mut IntoIter<Token>) -> Result<Expression, String> {
	let mut list: Vec<Expression> = vec![];
	while let Some(token) = token_iterator.next() {
		match token {
			Token::Close => return Ok(Expression::List(list)),
			_ => list.push(
				parse_expression(token, &mut token_iterator.by_ref())?
			),
		}
	}
	return Err("unexpected end of input".to_string())
}

