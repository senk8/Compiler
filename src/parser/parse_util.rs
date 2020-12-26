use core::iter::Peekable;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::types::token::TokenType::*;
use crate::types::token::TokenKind::*;
use crate::types::token::*;

//TODO below functions are not appropritate because it is not correspond LA.
pub fn expect_num<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> Option<usize>{
    if let Some(Num(_)) = tokenizer.peek() {
        match tokenizer.next(){
            Some(Num(n)) => Some(n),
            _ => None,
        }
    }else{
        None
    }
}

pub fn expect_ident<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> Option<char>{
    if let Some(Ident(_)) = tokenizer.peek() {
        match tokenizer.next(){
            Some(Ident(c)) => Some(c),
            _ => None,
        }
    }else{
        None
    }
}

pub fn expect<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:TokenKind)->(){
    if let Some(Token(kind))=tokenizer.peek() {
        if *kind != expect_token {
            panic!("Error! expect number,found other");
        }
    }
    tokenizer.next();
}

pub fn consume<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:TokenKind)->bool {
    if let Some(Token(kind))=tokenizer.peek() {
        if *kind == expect_token {
            tokenizer.next();
            return true;
        }else{
            return false;
        }
    }
    return false;
}

