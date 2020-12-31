use super::*;
use crate::types::token::Symbol::*;
use crate::types::token::TokenKind::*;
use crate::types::token::TokenType::*;
use crate::types::token::*;

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenType;
    fn next(&mut self) -> Option<TokenType> {
        // triming a head of string

        self.cur = self.cur.trim_start();
        let ascii = self.cur.as_bytes();

        match ascii {
            _ if ascii.len() == 0 => None,
            [b'r',b'e',b't',b'u',b'r',b'n',..] if self.expect_non_id(ascii,6) => {
                self.consume_head(6);
                Some(Keyword(Return))
            }
            [b'<',b'=',..] => self.consume_and_tokenize(Leq,2),
            [b'>',b'=',..] => self.consume_and_tokenize(Geq,2),
            [b'=',b'=',..] => self.consume_and_tokenize(Eq,2),
            [b'!',b'=',..] => self.consume_and_tokenize(Neq,2),
            [b'+',..] => self.consume_and_tokenize(Plus,1),
            [b'-',..] => self.consume_and_tokenize(Minus,1),
            [b'*',..] => self.consume_and_tokenize(Mul,1),
            [b'/',..] => self.consume_and_tokenize(Div,1),
            [b'<',..] => self.consume_and_tokenize(Lt,1),
            [b'>',..] => self.consume_and_tokenize(Gt,1),
            [b'=',..] => self.consume_and_tokenize(Assign,1),
            [b';',..] => self.consume_and_tokenize(Semicolon,1),
            [b'0'..=b'9',..] => {
                let head = self.consume_num();
                Some(Num(usize::from_str_radix(head, 10).unwrap()))
            },
            [b'a'..=b'z',..] => {
                let head = self.consume_ident();
                Some(Ident(String::from(head)))
            },
            _ => panic!(self.error_at("unexpected token")),
        }
    }  

}
