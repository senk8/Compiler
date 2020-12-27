pub mod tokenizer;
pub mod parser;
pub mod lexer;
pub mod types;

use std::env;
use tokenizer::*;
use parser::*;
use lexer::assemble::gen;


fn main(){
    let arg = env::args().nth(1).unwrap();

    let tokens_iter = Tokenizer::new(arg.as_str()).peekable();
    let mut parser= Parser::new(tokens_iter);
    let trees=parser.parse();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    for tree in trees.iter(){
        gen(tree);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp"); 
    println!("  ret");

    return;
}