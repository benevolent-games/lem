
use super::*;

#[test]
fn parens() {
	let input = "()";
	let tokens = tokenize(input);
	assert_eq!(tokens, vec![Token::Open, Token::Close]);
}

#[test]
fn parens_with_whitespace() {
	let input = "( \n\t )";
	let tokens = tokenize(input);
	assert_eq!(tokens, vec![Token::Open, Token::Close]);
}

#[test]
fn number() {
	let input = "42";
	let tokens = tokenize(input);
	assert_eq!(tokens, vec![Token::Number(42)]);
}

// #[test]
// fn number_as_float() {
// 	let input = "42.0";
// 	let tokens = lex(input);
// 	assert_eq!(tokens, vec![Token::Number(42)]);
// }

#[test]
fn symbol() {
	let input = "define x";
	let tokens = tokenize(input);
	assert_eq!(tokens, vec![Token::Symbol("define".to_string()), Token::Symbol("x".to_string())]);
}

