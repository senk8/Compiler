pub mod tokenizer;
pub mod parser;

use std::env;
use std::num;
use self::tokenizer::Tokenizer;
use self::tokenizer::Token;
use self::parser::*;
use self::parser::Node::*;


pub fn gen(node:&Node)->() {
    if let NdNum(n) = node {
      println!("  push {}", n);
      return;
    }

    let mut opr = "";
    match node{
        NdAdd(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr = "  add rax, rdi";
        },
        NdSub(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr = "  sub rax, rdi";
        },
        NdMul(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  imul rax, rdi";
        },
        NdDiv(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  cqo\n  idiv rdi";
        },
        _ => panic!("unexpected token")
    }
 
    println!("  pop rdi");
    println!("  pop rax");
    println!("{}",opr);
    println!("  push rax");
  }

fn main() {
    let arg = env::args().nth(1).unwrap();
    let mut tokens_iter = Tokenizer::new(arg.as_str()).peekable();
    let node=expr(&mut tokens_iter);
    println!("{:?}",&node);
    /*
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
    */
    return;
}