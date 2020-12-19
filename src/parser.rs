
use crate::tokenizer::*;
use crate::tokenizer::Tokenizer;
use crate::tokenizer::Token::*;
use core::iter::Peekable;

//TODO below functions are not appropritate because it is not correspond LA.

pub fn expect_num<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> usize{
    //TODO TKNumでないとifの中に入れないのだが、中でパターンマッチしないとだめか？
    if let TkNum(_) = tokenizer.peek().unwrap() {
        match tokenizer.next().unwrap(){
            TkNum(n) => n,
            _ => panic!("Error! expect number,found other")
        }
    }else{
        panic!("Error! expect number,found other");
    }
}

pub fn expect<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:&Token)->(){
    if let Some(token)=tokenizer.peek() {
        if token != expect_token {
            panic!("Error! expect number,found other");
        }
    }
    tokenizer.next();
}

pub fn consume<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:&Token)->bool {
    if let Some(token)=tokenizer.peek() {
        if token == expect_token {
            tokenizer.next();
            return true;
        }else{
            return false;
        }
    }
    return false;
}

