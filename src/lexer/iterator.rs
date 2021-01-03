use super::*;

use crate::types::token::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::TokenKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::DelimitorKind::*;


impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {

        if b' ' == *self.txt.get(self.pos)? {
            self.pos+=1;
        }

        match self.txt[self.pos..] {
            [b'r',b'e',b't',b'u',b'r',b'n',b' ',..] => self.lex_token(Key(Return),6),
            [b'<',b'=',..] => self.lex_token(Opr(Leq),2),
            [b'>',b'=',..] => self.lex_token(Opr(Geq),2),
            [b'=',b'=',..] => self.lex_token(Opr(Eq),2),
            [b'!',b'=',..] => self.lex_token(Opr(Neq),2),
            [b'+',..] => self.lex_token(Opr(Add),1),
            [b'-',..] => self.lex_token(Opr(Sub),1),
            [b'*',..] => self.lex_token(Opr(Mul),1),
            [b'/',..] => self.lex_token(Opr(Div),1),
            [b'<',..] => self.lex_token(Opr(Lt),1),
            [b'>',..] => self.lex_token(Opr(Gt),1),
            [b'=',..] => self.lex_token(Opr(Assign),1),
            [b';',..] => self.lex_token(Delim(Semicolon),1),
            [b'(',..] => self.lex_token(Delim(Lc),1),
            [b')',..] => self.lex_token(Delim(Rc),1),
            [b'0'..=b'9',..] => self.lex_num(),
            [b'a'..=b'z',..] => self.lex_ident(),
            _ => panic!(self.error_at("unexpected token")),
        }
    }  

}
