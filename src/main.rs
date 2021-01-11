pub mod lexer;
pub mod parser;
pub mod semantic_analyzer;
pub mod types;

use lexer::*;
use parser::*;
use semantic_analyzer::assemble::gen_stmt;
use std::env;

use crate::types::error::ParseError;


/*
use once_cell::sync::Lazy;
static INPUT:Lazy<[u8; 10000]> = Lazy::new(||
    *(env::args().nth(1).unwrap().as_bytes())
);
*/

fn main() -> Result<(), ParseError> {
    let arg = env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    let lexer = Lexer::new(arg.as_str());
    let parser = Parser::new(lexer);

    let asts = parser.parse().map_err(|m| {
        eprintln!("{}", m);
        m
    })?;

    gen_stmt(&asts);

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");

    return Ok(());
}
