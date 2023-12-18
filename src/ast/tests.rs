
use super::*;
use crate::lex::*;

#[test]
fn parse_number() {
	let tokens = vec![Token::Number(42)];
	let expected = vec![Expression::Number(42)];
	assert_eq!(parse(tokens).unwrap(), expected);
}

#[test]
fn parse_symbol() {
	let tokens = vec![Token::Symbol("foo".to_string())];
	let expected = vec![Expression::Symbol("foo".to_string())];
	assert_eq!(parse(tokens).unwrap(), expected);
}

#[test]
fn parse_fails_on_unexpected_close() {
	let tokens = vec![Token::Close];
	assert!(parse(tokens).is_err());
}

#[test]
fn parse_empty_list() {
	let tokens = vec![
		Token::Open,
		Token::Close,
	];
	let expected = vec![
		Expression::List(vec![]),
	];
	assert_eq!(parse(tokens).unwrap(), expected);
}

#[test]
fn parse_list_with_stuff_in_it() {
	let tokens = vec![
		Token::Open,
		Token::Symbol("add".to_string()),
		Token::Number(1),
		Token::Number(2),
		Token::Close,
	];
	let expected = vec![
		Expression::List(vec![
			Expression::Symbol("add".to_string()),
			Expression::Number(1),
			Expression::Number(2),
		]),
	];
	assert_eq!(parse(tokens).unwrap(), expected);
}

#[test]
fn parse_fails_on_nested_unexpected_close() {
	let tokens = vec![
		Token::Open,
		Token::Close,
		Token::Close,
	];
	assert!(parse(tokens).is_err());
}

