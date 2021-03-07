pub mod iterator;

use std::str::from_utf8;

use crate::types::types::*;
use crate::types::token::TokenKind::*;
use crate::types::token::*;

//use crate::types::error::TokenizeError;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
pub struct Lexer<'a> {
    /* it is only used by error_at */
    txt: &'a [u8],

    /* Cursor */
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Lexer<'a> {
        let txt = input;
        let pos = 0;
        Lexer { txt, pos }
    }

    pub fn cur(&self) -> Pos {
        Pos(1, self.pos)
    }
}

impl<'a> Lexer<'a> {
    //文字列の最初を取り除く
    fn consume(&mut self, n: usize) -> Option<()> {
        if self.pos + n <= self.txt.len() {
            self.pos += n;
            Some(())
        } else {
            None
        }
    }

    fn lex_token(&mut self, val: TokenKind, len: usize) -> Option<Token> {
        let pos = Pos(1, self.pos);
        self.consume(len)?;
        Some((val, pos))
    }

    //文字列を数字である限り消費する。
    fn lex_num(&mut self) -> Option<Token> {
        let begin = self.pos;

        while self.pos < self.txt.len() && self.txt[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let pos = Pos(begin, self.pos);

        /* TODO : TokenizeError isn't used  */
        let val = Num(from_utf8(&self.txt[begin..self.pos])
            .map(|s| usize::from_str_radix(s, 10))
            .unwrap()
            .unwrap());

        Some((val, pos))
    }

    //文字列をアルファベットである限り消費する。
    fn lex_ident(&mut self) -> Option<Token> {
        let begin = self.pos;

        while self.pos < self.txt.len() && self.txt[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }

        let pos = Pos(begin, self.pos);

        /* TODO : TokenizeError isn't used  */
        let val = Id(from_utf8(&self.txt[begin..self.pos])
            .map(|s| String::from(s))
            .unwrap());

        Some((val, pos))
    }

    fn error_at(&self, description: &str) -> String {
        let pos = self.pos;
        let mut message = format!("\n{}\n", from_utf8(self.txt).unwrap());
        message.push_str(&format!("{:>width$}", "^", width = pos + 1));
        message.push_str(&format!("\n{}", description));
        return message;
    }
}
