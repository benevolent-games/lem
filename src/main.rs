
mod lex;
mod ast;
mod exe;

fn main() {
	let input = "(let x 42)";
	let tokens = lex::tokenize(input);
	let syntax = ast::parse(tokens);
	let result = exe::execute(syntax.unwrap());
	println!("{:?}", result);
}

