pub mod parser;
pub mod semantic_analyzer;
pub mod lexer;
pub mod types;

use parser::*;
use semantic_analyzer::assemble::gen;
use std::env;
use lexer::*;

use crate::types::error::ParseError;



fn main() -> Result<(),ParseError>{
    let arg = env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    let lexer = Lexer::new(arg.as_str());
    let parser = Parser::new(lexer);

    let asts = parser
                .parse()
                .map_err(|m|{
                    eprintln!("{}",m);
                    m
                })?;

    for ast in asts.iter() {
        gen(ast);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");

    return Ok(());
}
