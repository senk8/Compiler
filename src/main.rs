pub mod lexer;
pub mod parser;
pub mod semantic_analyzer;
pub mod types;

use lexer::*;
use parser::*;
use semantic_analyzer::assemble::gen;
use std::env;

use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

#[cfg(test)]
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn show_message(error:&ParseError,input:&[u8])->(){
    match error {
        UnexpectedToken(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
        },
        UnexpectedKeyword(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        UnexpectedDelimitor(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        UnclosedDelimitor(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        ExpectedNumeric(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        MissingExpression(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("expected an experession, but found other. : {}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("Suggestion : It may be missing some expression. Add some expression here. ");
        },
        MissingSemicolon(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("expected \";\" , but found other. :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("Suggestion : It may be missing \";\".  Add \";\" here.");
        },
        Eof(pos) => {
            let code = input.last().unwrap();
            eprintln!("Parsing process reached EOF. Your input may lack a delimitor. :{}",pos);
            eprintln!("{}\n", *code as char);
            eprintln!("{}", "Suggestion: ");
        },
        SegmentationFault(pos) => {
            let code = input.last().unwrap();
            eprintln!("Segmentation Fault:{}",pos);
            eprintln!("{}\n", *code as char);
            eprintln!("{}", "Suggestion: ");
        }
    }

}

fn main() -> Result<(), ParseError> {
    let arg = env::args().nth(1).unwrap();
    let input = arg.as_bytes();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    let lexer = Lexer::new(arg.as_str());
    let parser = Parser::new(lexer);

    let asts = parser.parse().map_err(|error| {
        show_message(&error,input);
        error
    })?;
 
    for ast in asts.iter() {
        gen(&ast,&mut 0);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");

    return Ok(());
}
