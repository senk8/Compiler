pub mod tokenizer;

use std::env;
use self::tokenizer::Tokenizer;
use self::tokenizer::Token;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let mut tokens_iter = Tokenizer::new(arg.as_str());

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}" ,tokens_iter.next().unwrap().expect_num());

    while let Some(token) = tokens_iter.next(){
        let n = tokens_iter.next().unwrap().expect_num();
        match token {
            Token::TkPlus => println!("  add rax, {}", n),
            Token::TkMinus => println!("  sub rax, {}", n),
            Token::TkNum(_) => panic!("unexpected character!")
        }
    }

    println!("  ret");
    return;
}
