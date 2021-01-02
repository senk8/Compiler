pub mod iterator;
use crate::types::token::*;
use crate::types::token::TokenType::*;
use crate::types::error::TokenizeError;
use std::str::from_utf8;

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Default,Hash)]
pub struct Lexer<'a> {
    /* it is only used by error_at */
    txt: &'a [u8],

    /* Cursor */
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let txt = input.as_bytes();
        let pos = 0;
        Lexer {txt,pos}
    }

    //文字列の最初を取り除く
    fn consume(&mut self, n: usize) -> Option<()>{
        if self.pos + n <= self.txt.len() {
            self.pos += n;
            Some(())
        }else{
            None
        }
    }
}

impl<'a> Lexer<'a> {

    fn lex_token(&mut self, kind:TokenKind,len : usize) -> Option<TokenType> {
        self.consume(len)?;
        Some(Token(kind))
    }

    fn lex_keyword(&mut self,keyword:KeywordKind,len: usize) -> Option<TokenType>{
        self.consume(len)?;
        Some(Keyword(keyword))
    }

    //文字列を数字である限り消費する。
    fn lex_num(&mut self) -> Option<TokenType> {
        let begin = self.pos;

        while self.pos < self.txt.len() && self.txt[self.pos].is_ascii_digit() {
            self.pos+=1;
        }

        /* TODO : TokenizeError isn't used  */
        Some(Num(
        from_utf8(&self.txt[begin..self.pos])
        .map(|s|usize::from_str_radix(s,10))
        .unwrap()
        .unwrap()
        ))
    }

    //文字列をアルファベットである限り消費する。
    fn lex_ident(&mut self) -> Option<TokenType>  {
        let begin = self.pos;

        while self.pos < self.txt.len() && self.txt[self.pos].is_ascii_alphabetic() {
            self.pos+=1;
        }

        /* TODO : TokenizeError isn't used  */
        Some(Id(
        from_utf8(&self.txt[begin..self.pos])
        .map(|s|String::from(s))
        .unwrap()
        ))
    }
}


impl<'a> Lexer<'a> {

    fn error_at(&self, description: &str) -> String {
        let pos = self.pos;
        let mut message = format!("\n{}\n", from_utf8(self.txt).unwrap());
        message.push_str(&format!("{:>width$}", "^", width = pos + 1));
        message.push_str(&format!("\n{}", description));
        return message;
    }

    /*
    fn expect_non_idx(&self,idx:usize)->bool{
        match self.txt.get(idx) {
            Some(c) if !(c.is_ascii_alphanumeric() || *c == b'_') => true,
            _ => false,
        }
    }
    */
}