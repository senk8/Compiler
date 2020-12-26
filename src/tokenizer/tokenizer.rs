use crate::types::token::*;
use crate::types::token::TokenKind::*;

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
    pub fn consume_num(&mut self)->&str{
        let first_non_num_idx = self.cur.find(|c| !char::is_numeric(c)).unwrap_or(self.cur.len());
        let (head,tail) = self.cur.split_at(first_non_num_idx);
        self.cur = tail;
        self.pos += first_non_num_idx;
        return head;
    }

    //文字列の最初を取り除く
    fn consume_head(&mut self,index:usize)->(){
        let (_,tail) = self.cur.split_at(index);
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
    type Item = TokenKind;
    fn next(&mut self) -> Option<TokenKind> {
        // triming a head of string
        self.cur = self.cur.trim_start();
        let ascii=self.cur.as_bytes();
        let head = ascii.get(0)?;

        if let Some(head) = ascii.get(0..2){
            match head {
                b"<=" => {
                    self.consume_head(2);
                    return Some(Lt);
                }, 
                b">=" => {
                    self.consume_head(2);
                    return Some(Gt);
                }, 
                b"==" => {
                    self.consume_head(2);
                    return Some(Eq);
                }, 
                b"!=" => {
                    self.consume_head(2);
                    return Some(Neq);
                },
                _ => (),
            }
        }

        match head {
            b'+' => {
                self.consume_head(1);
                Some(Plus)
            },
            b'-' => {
                self.consume_head(1);
                Some(Minus)
            },
            b'*' => {
                self.consume_head(1);
                Some(Mul)
            },
            b'/' => {
                self.consume_head(1);
                Some(Div)
            },
            b'<' => {
                self.consume_head(1);
                Some(Lt)
            }, 
            b'>' => {
                self.consume_head(1);
                Some(Gt)
            }, 
            b'=' => {
                self.consume_head(1);
                Some(Assign)
            }, 
            b';' => {
                self.consume_head(1);
                Some(Semicolon)
            }, 
            c =>{
                if b'0' <= *c && *c <= b'9'{
                   let head = self.consume_num();
                   Some(Num(usize::from_str_radix(head,10).unwrap()))
                }else if b'a' <= *c && *c <= b'z'{
                   self.consume_head(1);
                   Some(Ident(*c as char))
                }else{
                    panic!(self.error_at("unexpected token"));
                }
           },
        }
    }
}