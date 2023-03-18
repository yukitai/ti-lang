use ti_lang::{frontend::{lexer::lexer::Lexer, parser::parser::Parser}, backend::codegen::codegen::Codegen};

fn main() {
	
	let src = "
fn mid(a: i32, b: i32) -> i32<i32, i32> = (a + b) / 2

fn main() = 
	println(\"* Mid 1 3 = {}\", mid(1, 3))
";
	let bytes = src.bytes().collect();
	let mut lexer = Lexer::from_bytes(bytes).unwrap();
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	parser.parse();
	let codegen = Codegen::new(parser);
	println!("{}", codegen.bytecode());

}