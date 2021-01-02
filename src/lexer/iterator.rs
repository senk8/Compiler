use super::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::TokenKind::*;
use crate::types::token::TokenType::*;
use crate::types::token::*;


impl<'a> Iterator for Lexer<'a> {
    type Item = TokenType;
    fn next(&mut self) -> Option<TokenType> {

        if b' ' == *self.txt.get(self.pos)? {
            self.pos+=1;
        }

        match self.txt[self.pos..] {
            [b'r',b'e',b't',b'u',b'r',b'n',b' ',..] => self.lex_keyword(Return,6),
            [b'<',b'=',..] => self.lex_token(Leq,2),
            [b'>',b'=',..] => self.lex_token(Geq,2),
            [b'=',b'=',..] => self.lex_token(Eq,2),
            [b'!',b'=',..] => self.lex_token(Neq,2),
            [b'+',..] => self.lex_token(Plus,1),
            [b'-',..] => self.lex_token(Minus,1),
            [b'*',..] => self.lex_token(Mul,1),
            [b'/',..] => self.lex_token(Div,1),
            [b'<',..] => self.lex_token(Lt,1),
            [b'>',..] => self.lex_token(Gt,1),
            [b'=',..] => self.lex_token(Assign,1),
            [b';',..] => self.lex_token(Semicolon,1),
            [b'(',..] => self.lex_token(Lc,1),
            [b')',..] => self.lex_token(Rc,1),
            [b'0'..=b'9',..] => self.lex_num(),
            [b'a'..=b'z',..] => self.lex_ident(),
            _ => panic!(self.error_at("unexpected token")),
        }
    }  

}
