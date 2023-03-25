use ti_lang::{
    frontend::{lexer::lexer::Lexer, parser::parser::Parser},
    vm::vm_ast,
};

fn main() {
    let src = "
fn mid(a, b) => (a + b) / 2

fn main() {
	mid(1, 3)
}
";
    let bytes = src.bytes().collect();
    let mut lexer = Lexer::from_bytes(bytes).unwrap();
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse();
    let mut vm = vm_ast::TiVM::new();
    println!("{:?}", vm.execute(parser.ast, "main"));
}
