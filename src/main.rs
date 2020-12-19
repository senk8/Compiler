pub mod tokenizer;
pub mod parser;
pub mod grammar;
pub mod assemble;

use std::env;
use self::tokenizer::Tokenizer;
use self::grammar::expr;
use self::assemble::gen;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let mut tokens_iter = Tokenizer::new(arg.as_str()).peekable();
    let node=expr(&mut tokens_iter);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen(&node);

    println!("  pop rax");
    println!("  ret");

    return;
}