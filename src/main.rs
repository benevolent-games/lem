
mod lex;
mod ast;

fn main() {
	let input = "(let x 42)";
	let tokens = lex::tokenize(input);
	let syntax = ast::parse(tokens);
	println!("{:?}", syntax);
}

