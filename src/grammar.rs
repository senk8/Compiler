
use crate::tokenizer::Tokenizer;
use crate::tokenizer::Token::*;
use core::iter::Peekable;
use crate::parser::*;
use crate::grammar::Node::*;

#[derive(Debug)]
pub enum Node{
    NdAdd(Box<Node>,Box<Node>),
    NdSub(Box<Node>,Box<Node>),
    NdMul(Box<Node>,Box<Node>),
    NdDiv(Box<Node>,Box<Node>),
    NdNum(usize) 
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

