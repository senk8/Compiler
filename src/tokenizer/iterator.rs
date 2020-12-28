use super::*;
use crate::types::token::*;
use crate::types::token::TokenKind::*;
use crate::types::token::TokenType::*;
use crate::types::token::Symbol::*;

impl<'a> Iterator for Tokenizer<'a>{
    type Item = TokenType;
    fn next(&mut self) -> Option<TokenType> {
        // triming a head of string
        self.cur = self.cur.trim_start();
        let ascii=self.cur.as_bytes();
        let head = ascii.get(0)?;

        if let Some(keyword) = ascii.get(0..6){
            match keyword {
                b"return" =>{
                    if let Some(c) = ascii.get(6){
                        if !(char::is_alphabetic(*c as char) && char::is_numeric(*c as char) && *c == b'_' ){
                            self.consume_head(6);
                            return Some(Keyword(Return));
                        }
                    }
                },
                _ => (),
            }
        }

        if let Some(prefix) = ascii.get(0..2){
            match prefix {
                b"<=" => {
                    self.consume_head(2);
                    return Some(Token(Lt));
                }, 
                b">=" => {
                    self.consume_head(2);
                    return Some(Token(Gt));
                }, 
                b"==" => {
                    self.consume_head(2);
                    return Some(Token(Eq));
                }, 
                b"!=" => {
                    self.consume_head(2);
                    return Some(Token(Neq));
                },
                _ => (),
            }
        }

        match head {
            b'+' => {
                self.consume_head(1);
                Some(Token(Plus))
            },
            b'-' => {
                self.consume_head(1);
                Some(Token(Minus))
            },
            b'*' => {
                self.consume_head(1);
                Some(Token(Mul))
            },
            b'/' => {
                self.consume_head(1);
                Some(Token(Div))
            },
            b'<' => {
                self.consume_head(1);
                Some(Token(Lt))
            }, 
            b'>' => {
                self.consume_head(1);
                Some(Token(Gt))
            }, 
            b'=' => {
                self.consume_head(1);
                Some(Token(Assign))
            }, 
            b';' => {
                self.consume_head(1);
                Some(Token(Semicolon))
            }, 
            c =>{
                if b'0' <= *c && *c <= b'9'{
                   let head = self.consume_num();
                   Some(Num(usize::from_str_radix(head,10).unwrap()))
                }else if b'a' <= *c && *c <= b'z'{
                   let head = self.consume_ident();
                   Some(Ident(String::from(head)))
                }else{
                   panic!(self.error_at("unexpected token"));
                }
           },
        }
    }
}