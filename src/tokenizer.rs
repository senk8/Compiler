use crate::tokenizer::Token::*;

#[derive(Debug)]
pub enum Token{
    TkNum(usize),
    TkPlus,
    TkMinus
}

impl Token{
    pub fn expect_num(&self) -> usize{
        match self {
           TkNum(n) => *n,
           _ => panic!("Error! expect number,found other")
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    line: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(line:&'a str)->Tokenizer<'a>{
        Tokenizer{ line:line }
    }

    //文字列を数字である限り消費する。
    pub fn consume_num_greedy(&mut self)->&str{
        let first_non_num_idx = self.line.find(|c| !char::is_numeric(c)).unwrap_or(self.line.len());
        let (head,tail) = self.line.split_at(first_non_num_idx);
        self.line = tail;
        head
    }

    fn drop_head(&mut self)->(){
        self.line = &self.line[1..];
    }
}


impl<'a> Iterator for Tokenizer<'a>{
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // triming a head of string
        self.line = self.line.trim_start();

        if self.line.is_empty(){
            return None;
        }

        match self.line.as_bytes()[0] {
            b'+' => {
                self.line = &self.line[1..];
                Some(TkPlus)
            },
            b'-' => {
                self.drop_head();
                Some(TkMinus)
            },
            b'0'|b'1'|b'2'|b'3'|b'4'|b'5'|b'6'|b'7'|b'8'|b'9' =>{
                let head = self.consume_num_greedy();
                Some(TkNum(usize::from_str_radix(head,10).unwrap()))
            },
            _ => panic!("unexpected token")
        }
    }
}