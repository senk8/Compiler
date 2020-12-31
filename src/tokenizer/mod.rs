pub mod iterator;
use crate::types::token::*;
use crate::types::token::TokenType::*;


#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Default,Hash)]
pub struct Tokenizer<'a> {
    /* it is only used by error_at */
    input: &'a [u8],

    /* Cursor */
    cur: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        let bytes = input.as_bytes();
        Tokenizer {
            input: bytes,
            cur: bytes,
            pos: 0,
        }
    }

    //文字列を数字である限り消費する。
    fn consume_num(&mut self) -> &[u8] {
        let first_non_num_idx = self
            .cur
            .iter()
            .position(|c|!c.is_ascii_digit())
            .unwrap_or(self.cur.len());
        let (head, tail) = self.cur.split_at(first_non_num_idx);
        self.cur = tail;
        self.pos += first_non_num_idx;
        return head;
    }

    //文字列をアルファベットである限り消費する。
    fn consume_ident(&mut self) -> &[u8] {
        let first_non_alpha_idx = self
            .cur
            .iter()
            .position(|c|!c.is_ascii_alphabetic())
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

    fn consume_and_tokenize(&mut self, kind:TokenKind,num : usize) -> Option<TokenType> 
    {
        self.consume_head(num);
        Some(Token(kind))
    }
}


impl<'a> Tokenizer<'a> {
    fn expect_non_id(&self,idx:usize)->bool{
        if let Some(c) = self.cur.get(idx) {
            if !(c.is_ascii_alphanumeric() || *c == b'_') {
                true
            }else{
                false
            }
        }else{
            false
        }
    }

    fn error_at(&self, description: &str) -> String {
        let pos = self.pos;
        let mut message = format!("\n{}\n", std::str::from_utf8(self.input).unwrap());
        message.push_str(&format!("{:>width$}", "^", width = pos + 1));
        message.push_str(&format!("\n{}", description));
        return message;
    }
}