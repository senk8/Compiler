
use crate::tokenizer::*;
use crate::tokenizer::Tokenizer;
use crate::tokenizer::Token::*;
use core::iter::Peekable;
use crate::parser::Node::*;

#[derive(Debug)]
pub enum Node{
    NdAdd(Box<Node>,Box<Node>),
    NdSub(Box<Node>,Box<Node>),
    NdMul(Box<Node>,Box<Node>),
    NdDiv(Box<Node>,Box<Node>),
    NdNum(usize) 
}

//TODO below functions are not appropritate because it is not correspond LA.

pub fn expect_num<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> usize{
    //TODO TKNumでないとifの中に入れないのだが、中でパターンマッチしないとだめか？
    if let TkNum(_) = tokenizer.peek().unwrap() {
        match tokenizer.next().unwrap(){
            TkNum(n) => n,
            _ => panic!("Error! expect number,found other")
        }
    }else{
        panic!("Error! expect number,found other");
    }
}

pub fn expect<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:&Token)->(){
    let token = tokenizer.peek().unwrap();

    if token != expect_token {
        panic!("Error! expect number,found other");
    }

    tokenizer.next();
}

pub fn consume<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:&Token)->bool {
    let token = tokenizer.peek().unwrap();

    if token != expect_token {
        false
    }else{
        tokenizer.next();
        true
    }
}

// This function represent following grammar.
// primary = num | "(" expr ")"*
pub fn primary<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    if consume(tokenizer,&TkRc) {
        let node = expr(tokenizer);
        expect(tokenizer,&TkLc);
        return node;
    }
    NdNum(expect_num(tokenizer))
}

// This function represent following grammar.
// mul     = primary ("*" primary | "/" primary)*
pub fn mul<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = primary(tokenizer);

    loop {
        if consume(tokenizer,&TkMul){
            node = NdMul(Box::new(node),Box::new(primary(tokenizer)));
        }else if consume(tokenizer,&TkDiv) {
            node = NdDiv(Box::new(node),Box::new(primary(tokenizer)));
        }else{
            break node;
        }
    }
}

// This function represent following grammar.
// expr    = mul ("+" mul | "-" mul)*
pub fn expr<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = mul(tokenizer);

    loop {
        if consume(tokenizer,&TkPlus){
            node = NdAdd(Box::new(node),Box::new(mul(tokenizer)));
        }else if consume(tokenizer,&TkMinus) {
            node = NdSub(Box::new(node),Box::new(mul(tokenizer)));
        }else{
            break node;
        }
    }
}

