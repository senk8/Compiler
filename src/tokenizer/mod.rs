pub mod iterator;
use crate::types::token::*;
use crate::types::token::TokenType::*;

/*
#[derive(Debug)]
pub struct Tokenizer<'a> {
    /* it is only used by error_at */
    input: &'a [u8],
    //source: &'a str,

    /* Cursor */
    cur: &'a [u8],
    //cur: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        let byte_input = input.as_bytes();
        Tokenizer {
            input: byte_input,
            //source: input,
            cur: byte_input,
            //cur: input,
            pos: 0,
        }
    }
 */

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Default,Hash)]
pub struct Tokenizer<'a> {
    /* it is only used by error_at */
    source: &'a str,

    /* Cursor */
    cur: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            source: source,
            cur: source,
            pos: 0,
        }
    }

    //文字列を数字である限り消費する。
    pub fn consume_num(&mut self) -> &str {
        let first_non_num_idx = self
            .cur
            .find(|c| !char::is_numeric(c))
            .unwrap_or(self.cur.len());
        let (head, tail) = self.cur.split_at(first_non_num_idx);
        self.cur = tail;
        self.pos += first_non_num_idx;
        return head;
    }

    //文字列をアルファベットである限り消費する。
    pub fn consume_ident(&mut self) -> &str {
        let first_non_alpha_idx = self
            .cur
            .find(|c| !char::is_alphabetic(c))
            .unwrap_or(self.cur.len());
        let (head, tail) = self.cur.split_at(first_non_alpha_idx);
        self.cur = tail;
        self.pos += first_non_alpha_idx;
        return head;
    }

    //文字列の最初を取り除く
    fn consume_head(&mut self, index: usize) -> () {
        let (_, tail) = self.cur.split_at(index);
        self.cur = tail;
        self.pos += 1;
    }

    //字句解析中のエラーを報告する。
    fn error_at(&self, description: &str) -> String {
        let pos = self.pos;
        let mut message = format!("\n{}\n", &self.source);
        message.push_str(&format!("{:>width$}", "^", width = pos + 1));
        message.push_str(&format!("\n{}", description));
        return message;
    }

    fn consume_and_tokenize(&mut self, kind:TokenKind,num : usize) -> Option<TokenType> {
        self.consume_head(num);
        Some(Token(kind))
    }
}
