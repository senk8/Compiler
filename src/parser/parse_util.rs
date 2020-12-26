use core::iter::Peekable;
use crate::tokenizer::Tokenizer;
use crate::types::token::TokenKind::*;
use crate::types::token::*;


//TODO below functions are not appropritate because it is not correspond LA.

pub fn expect_num<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> usize{
    //TODO TKNumでないとifの中に入れないのだが、中でパターンマッチしないとだめか？
    if let Num(_) = tokenizer.peek().unwrap() {
        match tokenizer.next().unwrap(){
            Num(n) => n,
            _ => panic!("Error! expect number,found other")
        }
    }else{
        panic!("Error! expect number,found other");
    }
}

pub fn expect<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:TokenKind)->(){
    if let Some(token)=tokenizer.peek() {
        if *token != expect_token {
            panic!("Error! expect number,found other");
        }
    }
    tokenizer.next();
}

pub fn consume<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:TokenKind)->bool {
    if let Some(token)=tokenizer.peek() {
        if *token == expect_token {
            tokenizer.next();
            return true;
        }else{
            return false;
        }
    }
    return false;
}

