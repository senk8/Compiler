use super::Lexer;

use std::str::from_utf8;

use crate::types::tokenize::DelimitorKind::*;
use crate::types::tokenize::KeywordKind::*;
use crate::types::tokenize::OperatorKind::*;
use crate::types::tokenize::TokenKind::*;
use crate::types::tokenize::TypeKind::*;
use crate::types::tokenize::*;

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
            .map(|s| s.parse::<usize>())
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
            .map(String::from)
            .unwrap());

        Some((val, pos))
    }

    fn error_at(&self, description: &str) -> String {
        let pos = self.pos;
        let mut message = format!("\n{}\n", from_utf8(self.txt).unwrap());
        message.push_str(&format!("{:>width$}", "^", width = pos + 1));
        message.push_str(&format!("\n{}", description));
        message
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // Consumes as long as a '\n' or ' ' remains.
        while let b' ' | b'\n' = *self.txt.get(self.pos)? {
            self.pos += 1
        }
        match self.txt[self.pos..] {
            [b's', b'i', b'z', b'e', b'o', b'f', ..] => self.lex_token(Opr(Sizeof), 6),
            [b'r', b'e', b't', b'u', b'r', b'n', b' ', ..] => self.lex_token(Key(Return), 6),
            [b'i', b'f', b'(', ..] => self.lex_token(Key(If), 2),
            [b'w', b'h', b'i', b'l', b'e', b'(', ..] => self.lex_token(Key(While), 5),
            [b'e', b'l', b's', b'e', b' ', ..] => self.lex_token(Key(Else), 4),
            [b'f', b'o', b'r', b'(', ..] => self.lex_token(Key(For), 3),
            [b'i', b'n', b't', ..] => self.lex_token(Type(Int), 3),
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
            [b'(', ..] => self.lex_token(Delim(Lparen), 1),
            [b')', ..] => self.lex_token(Delim(Rparen), 1),
            [b'{', ..] => self.lex_token(Delim(Lbrace), 1),
            [b'}', ..] => self.lex_token(Delim(Rbrace), 1),
            [b'[', ..] => self.lex_token(Delim(Lbracket), 1),
            [b']', ..] => self.lex_token(Delim(Rbracket), 1),
            [b'0'..=b'9', ..] => self.lex_num(),
            [b'a'..=b'z', ..] => self.lex_ident(),
            _ => panic!("{}", self.error_at("unexpected token")),
        }
    }
}
