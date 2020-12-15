pub mod tokenizer;
pub mod parser;

use std::env;
use self::tokenizer::Tokenizer;
use self::tokenizer::Token;
use self::parser::*;
use self::parser::Node::*;


pub fn 

pub fn gen(node:&Node)->() {
    if let NdNum(n) = node {
      println!("  push {}", n);
      return;
    }
  
    gen(*node.0);
    gen(*node.1);
  
    println!("  pop rdi");
    println!("  pop rax");
 
    match node {
        NdAdd(_,_) => println!("  add rax, rdi"),
        NdSub(_,_) => println!("  sub rax, rdi"),
        NdMul(_,_) => println!("  imul rax, rdi"),
        NdDiv(_,_) => {
            println!("  cqo");
            println!("  idiv rdi");
        },
        _ => panic!("unexpected token")
    }

    println!("  push rax");
  }

fn main() {
    let arg = env::args().nth(1).unwrap();
    let mut tokens_iter = Tokenizer::new(arg.as_str()).peekable();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}" ,expect_num(&mut tokens_iter));

    while let Some(token) = tokens_iter.next(){
        let n = expect_num(&mut tokens_iter);
        match token {
            Token::TkPlus => println!("  add rax, {}", n),
            Token::TkMinus => println!("  sub rax, {}", n),
            Token::TkNum(_) => panic!("unexpected character!"),
            _ => panic!("undefined")
        }
    }
    
    println!("  ret");
    return;
}
