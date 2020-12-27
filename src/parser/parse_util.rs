use core::iter::Peekable;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::types::token::TokenType::*;
use crate::types::token::TokenKind::*;
use crate::types::token::*;

//TODO below functions are not appropritate because it is not correspond LA.
pub fn take_num<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> Option<usize>{
    if let Some(Num(_)) = tokenizer.peek() {
        match tokenizer.next(){
            Some(Num(n)) => Some(n),
            _ => None,
        }
    }else{
        None
    }
}

pub fn take_ident<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> Option<String>{
    if let Some(Ident(_)) = tokenizer.peek() {
        match tokenizer.next(){
            Some(Ident(s)) => Some(s),
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

/*
pub fn is_alnum<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->bool {
    ('a' <= c && c <= 'z') ||
    ('A' <= c && c <= 'Z') ||
    ('0' <= c && c <= '9') ||
    (c == '_')
}

if prefix = "return" && is_alnum(prefix[6])) {
    Keyword(Return)

}
*/