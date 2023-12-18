
use std::str::Chars;
use std::iter::Peekable;
use crate::types::*;

pub fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();

	while let Some(&c) = chars.peek() {
		match c {
			'(' => {
				tokens.push(Token::Open);
				chars.next();
			},
			')' => {
				tokens.push(Token::Close);
				chars.next();
			},
			_ if c.is_whitespace() => {
				chars.next();
			},
			_ if c.is_digit(10) => {
				tokens.push(lex_number(&mut chars));
			},
			_ if c.is_alphabetic() => {
				tokens.push(lex_symbol(&mut chars));
			},
			_ => panic!("Unexpected character: {}", c),
		}
	}

	tokens
}

fn lex_number(chars: &mut Peekable<Chars>) -> Token {
	let mut number = String::new();
	while let Some(&c) = chars.peek() {
		if c.is_digit(10) {
			number.push(c);
			chars.next();
		} else {
			break;
		}
	}
	Token::Number(number.parse().unwrap())
}

fn lex_symbol(chars: &mut Peekable<Chars>) -> Token {
	let mut symbol = String::new();
	while let Some(&c) = chars.peek() {
		if c.is_alphabetic() || c == '_' {
			symbol.push(c);
			chars.next();
		} else {
			break;
		}
	}
	Token::Symbol(symbol)
}

