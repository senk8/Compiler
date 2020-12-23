use self::Token::*;

#[derive(Debug,PartialEq)]
pub enum Token{
    TkNum(usize),
    TkPlus,
    TkMinus,
    TkMul,
    TkDiv,
    TkLc,
    TkRc,
    TkEq,
    TkNeq,
    TkGeq,
    TkLeq,
    TkLt,
    TkGt,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    source: &'a str,//構文解析中にエラーを報告するためだけに使う
    cur: &'a str,
    pos: usize
}

impl<'a> Tokenizer<'a> { 
    pub fn new(source:&'a str)->Tokenizer<'a>{
        Tokenizer{ source:source , cur:source, pos:0 }
    }

    //文字列を数字である限り消費する。
    pub fn consume_num_greedy(&mut self)->&str{
        let first_non_num_idx = self.cur.find(|c| !char::is_numeric(c)).unwrap_or(self.cur.len());
        let (head,tail) = self.cur.split_at(first_non_num_idx);
        self.cur = tail;
        self.pos += first_non_num_idx;
        return head;
    }

    //文字列の最初を取り除く
    fn consume_head(&mut self,index:usize)->(){
        let (head,tail) = self.cur.split_at(index);
        self.cur = tail;
        self.pos+=1;
    }

    //字句解析中のエラーを報告する。
    fn error_at(&self,description:&str)->String{
        let pos = self.pos ;
        let mut message = format!("\n{}\n",&self.source);
        message.push_str(&format!("{:>width$}","^",width = pos + 1));
        message.push_str(&format!("\n{}",description));
        return message;
    }
}


impl<'a> Iterator for Tokenizer<'a>{
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // triming a head of string
        self.cur = self.cur.trim_start();
        let ascii=self.cur.as_bytes();
        let head = ascii.get(0)?;

        if let Some(head) = ascii.get(0..2){
            match head {
                b"<=" => {
                    self.consume_head(2);
                    return Some(TkLt);
                }, 
                b">=" => {
                    self.consume_head(2);
                    return Some(TkGt);
                }, 
                b"==" => {
                    self.consume_head(2);
                    return Some(TkEq);
                }, 
                b"!=" => {
                    self.consume_head(2);
                    return Some(TkNeq);
                },
                _ => (),
            }
        }

        match head {
            b'+' => {
                self.consume_head(1);
                Some(TkPlus)
            },
            b'-' => {
                self.consume_head(1);
                Some(TkMinus)
            },
            b'*' => {
                self.consume_head(1);
                Some(TkMul)
            },
            b'/' => {
                self.consume_head(1);
                Some(TkDiv)
            },
            b'<' => {
                self.consume_head(1);
                Some(TkLt)
            }, 
            b'>' => {
                self.consume_head(1);
                Some(TkGt)
            }, 
            b'0'|b'1'|b'2'|b'3'|b'4'|b'5'|b'6'|b'7'|b'8'|b'9' =>{
                let head = self.consume_num_greedy();
                Some(TkNum(usize::from_str_radix(head,10).unwrap()))
            },
            _ => panic!(self.error_at("unexpected token"))
        }
    }
}