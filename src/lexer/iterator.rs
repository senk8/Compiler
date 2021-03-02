use super::*;

use crate::types::token::DelimitorKind::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;
use crate::types::token::*;

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // Consumes as long as a '\n' or ' ' remains.
        loop {
            match *self.txt.get(self.pos)? {
                b' ' | b'\n' => self.pos += 1,
                _ => break,
            }
        }
        match self.txt[self.pos..] {
            [b'r', b'e', b't', b'u', b'r', b'n', b' ', ..] => self.lex_token(Key(Return), 6),
            [b'i', b'f', b'(', ..] => self.lex_token(Key(If), 2),
            [b'w', b'h', b'i', b'l', b'e', b'(', ..] => self.lex_token(Key(While), 5),
            [b'e', b'l', b's', b'e', b' ', ..] => self.lex_token(Key(Else), 4),
            [b'f', b'o', b'r', b'(', ..] => self.lex_token(Key(For), 3),
            [b'i', b'n', b't', ..] => self.lex_token(Key(Int), 3),
            [b'<', b'=', ..] => self.lex_token(Opr(Leq), 2),
            [b'>', b'=', ..] => self.lex_token(Opr(Geq), 2),
            [b'=', b'=', ..] => self.lex_token(Opr(Eq), 2),
            [b'!', b'=', ..] => self.lex_token(Opr(Neq), 2),
            [b'+', ..] => self.lex_token(Opr(Add), 1),
            [b'-', ..] => self.lex_token(Opr(Sub), 1),
            [b'*', ..] => self.lex_token(Opr(Star), 1),
            [b'/', ..] => self.lex_token(Opr(Div), 1),
            [b'&', ..] => self.lex_token(Opr(Amp), 1),
            [b'<', ..] => self.lex_token(Opr(Lt), 1),
            [b'>', ..] => self.lex_token(Opr(Gt), 1),
            [b'=', ..] => self.lex_token(Opr(Assign), 1),
            [b';', ..] => self.lex_token(Delim(Semicolon), 1),
            [b',', ..] => self.lex_token(Delim(Comma), 1),
            [b'(', ..] => self.lex_token(Delim(Lc), 1),
            [b')', ..] => self.lex_token(Delim(Rc), 1),
            [b'{', ..] => self.lex_token(Delim(LCurl), 1),
            [b'}', ..] => self.lex_token(Delim(RCurl), 1),
            [b'0'..=b'9', ..] => self.lex_num(),
            [b'a'..=b'z', ..] => self.lex_ident(),
            _ => panic!(self.error_at("unexpected token")),
        }
    }
}
