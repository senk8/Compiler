pub mod parser;
pub mod semantic_analyzer;
pub mod lexer;
pub mod types;

use parser::*;
use semantic_analyzer::assemble::gen;
use std::env;
use lexer::*;

fn main() {
    let arg = env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    let lexer = Lexer::new(arg.as_str()).peekable();
    let mut parser = Parser::new(lexer);

    for tree in parser.parse().iter() {
        gen(tree);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");

    return;
}

mod tests{
    #[test]
    fn test_add() {
        println!("{}",b' ');
    }
}

