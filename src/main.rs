pub mod tokenizer;
pub mod parser;
pub mod types;
pub mod assemble;

use std::env;
use tokenizer::tokenizer::Tokenizer;
use parser::stmt::program;
use assemble::assemble::gen;

fn main(){
    let arg = env::args().nth(1).unwrap();

    let mut tokens_iter = Tokenizer::new(arg.as_str()).peekable();
    let trees=program(&mut tokens_iter);

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