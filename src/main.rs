use colored::Colorize;
use ti_lang::{
    frontend::{lexer::lexer::Lexer, parser::parser::Parser},
    vm::vm_ast,
};

fn main() {
    let src = "
fn mid(a, b) => add(a, b) / 2

fn f(a) {
    if a < 2 {
        1
    } else {
        a * f(a - 1)
    }
}

fn try_while() {
    let a = 0
    let i = \"\"
    while a < 10 {
        a = a + 1
        i = i + \"Ha\"
    }
    i
}

fn main() => try_while()
";
    let bytes = src.bytes().collect();
    let mut lexer = Lexer::from_bytes(bytes).unwrap();
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse();
    let mut vm = vm_ast::TiVM::new();
    vm.execute(parser.ast);
    println!("{}: {}", "compile".white().bold(), "ok".green().bold());
    println!("{:?}", vm.run_fn("main"));
}
