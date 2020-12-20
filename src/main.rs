pub mod tokenizer;
pub mod parser;
pub mod assemble;

use std::env;
use std::fs::File;
use std::io::{self,BufReader,BufWriter};

use tokenizer::Tokenizer;
use parser::expr::expr;
use assemble::gen;

fn main() {
    let arg = env::args().nth(1).unwrap();

    /*
    let mut buf = "";

    for result in BufReader::new(File::open(arg.as_str())?).lines() {
        let l = result?;
        format!("{}{}",buf,l);
    }
    */

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