use crate::tokenizer::Token::*;

#[derive(Debug)]
pub enum Token{
    TkNum(usize),
    TkPlus,
    TkMinus
}


#[derive(Debug)]
pub struct Tokenizer<'a> {
    line: &'a str,
    cur: &'a str,
    pos: usize
}

// used for parse

impl<'a> Tokenizer<'a> {
    pub fn expect_num(&mut self) -> usize{
        let token = self.next().unwrap();
        match token {
           TkNum(n) => n,
           _ => panic!("Error! expect number,found other")
        }
    }

    pub fn expect(&mut self,op:char) {
        let token = self.next().unwrap();
        match token {
           TkPlus => if op!='+' { panic!("Error! expect number,found other")},
           TkMinus => if op!='-' { panic!("Error! expect number,found other")},
           _ => panic!("Error! expect number,found other")
        }
    }
    /*
    pub fn consume(&mut self,op:char)->bool {
        let token = self.next().unwrap();
        match token {
            TkPlus => if op=='+' { return true } else { return false},
            TkMinus => if op=='-' { return true } else { return false},
            _ => panic!("Error! expect number,found other")
        }
        token = token->next;
        return true;
    }
    */
}


// used for tokenize
impl<'a> Tokenizer<'a> { 
    pub fn new(line:&'a str)->Tokenizer<'a>{
        Tokenizer{ line:line , cur:line, pos:0 }
    }

    //文字列を数字である限り消費する。
    pub fn consume_num_greedy(&mut self)->&str{
        let first_non_num_idx = self.cur.find(|c| !char::is_numeric(c)).unwrap_or(self.cur.len());
        let (head,tail) = self.cur.split_at(first_non_num_idx);
        self.cur = tail;
        self.pos += first_non_num_idx;
        head
    }

    fn drop_head(&mut self)->(){
        self.cur = &self.cur[1..];
        self.pos+=1;
    }

    fn error_at(&self,message:&str)->String{
        let pos = self.pos ;
        let mut buf = format!("\n{}\n",&self.line);
        buf.push_str(&format!("{:>width$}","^",width = pos + 1));
        buf.push_str(&format!("\n{}",message));
        buf
    }
}


impl<'a> Iterator for Tokenizer<'a>{
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // triming a head of string
        self.cur = self.cur.trim_start();

        if self.cur.is_empty(){
            return None;
        }

        match self.cur.as_bytes()[0] {
            b'+' => {
                self.drop_head();
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
            _ => panic!(self.error_at("unexpected token"))
        }
    }
}