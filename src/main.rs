pub mod tokenizer;
pub mod parser;
pub mod lexer;
pub mod types;

use std::env;
use tokenizer::*;
use parser::*;
use semantic_analyzer::assemble::gen;


fn main(){
    let arg = env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    let tokenizer = Tokenizer::new(arg.as_str()).peekable();
    let mut parser= Parser::new(tokenizer);

    for tree in parser.parse().iter(){
        gen(tree);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp"); 
    println!("  ret");

    return;
}