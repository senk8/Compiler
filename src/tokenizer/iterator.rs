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

        if let Some(b"return") = ascii.get(0..6) {
            if let Some(c) = ascii.get(6) {
                if !(char::is_alphanumeric(*c as char) || *c == b'_') {
                        self.consume_head(6);
                        return Some(Keyword(Return));
                }
            }
        }

        if let Some(b"<=") = ascii.get(0..2){
            return self.consume_and_tokenize(Leq,2);
        }else if let Some(b">=") = ascii.get(0..2){
            return self.consume_and_tokenize(Geq,2);
        }else if let Some(b"==") = ascii.get(0..2){
            return self.consume_and_tokenize(Eq,2);
        }else if let Some(b"!=") = ascii.get(0..2){
            return self.consume_and_tokenize(Neq,2);
        }

        let prefix = *ascii.get(0)?;

        match prefix {
            b'+' => self.consume_and_tokenize(Plus,1),
            b'-' => self.consume_and_tokenize(Minus,1),
            b'*' => self.consume_and_tokenize(Mul,1),
            b'/' => self.consume_and_tokenize(Div,1),
            b'<' => self.consume_and_tokenize(Lt,1),
            b'>' => self.consume_and_tokenize(Gt,1),
            b'=' => self.consume_and_tokenize(Assign,1),
            b';' => self.consume_and_tokenize(Semicolon,1),
            cha  => {
                if b'0' <= cha && cha <= b'9' {
                    let head = self.consume_num();
                    return Some(Num(usize::from_str_radix(head, 10).unwrap()));
                } else if b'a' <= cha && cha <= b'z' {
                    let head = self.consume_ident();
                    return Some(Ident(String::from(head)));
                } else { panic!(self.error_at("unexpected token"));}
            }
        }
    }  

}
