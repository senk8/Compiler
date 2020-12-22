use core::iter::Peekable;

use crate::tokenizer::Tokenizer;
use crate::tokenizer::Token::*;

use super::parse_util::*;
use super::node::Node;
use super::node::Node::*;



pub fn expr<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    equality(tokenizer)
}

// equality = relational ("==" relational | "!=" relational)*
pub fn equality<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = relational(tokenizer);

    loop{
        if consume(tokenizer,&TkEq){
            node = NdEq(Box::new(node),Box::new(relational(tokenizer)));
        }else if consume(tokenizer,&TkNeq){
            node = NdNeq(Box::new(node),Box::new(relational(tokenizer)));
        }else{
            break node;
        }
    }
}

//relational = add ("<" add | "<=" add | ">" add| ">=" add) *
pub fn relational<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = add(tokenizer);

    loop{
        if consume(tokenizer,&TkLt){
            node = NdLt(Box::new(node),Box::new(add(tokenizer)));
        }else if consume(tokenizer,&TkLeq){
            node = NdLeq(Box::new(node),Box::new(add(tokenizer)));
        }else if consume(tokenizer,&TkGt){
            node = NdLt(Box::new(add(tokenizer)),Box::new(node));
        }else if consume(tokenizer,&TkGeq){
            node = NdLeq(Box::new(add(tokenizer)),Box::new(node));
        }else{
            break node;
        }
    }
}

// This function represent following grammar.
// add    = mul ("+" mul | "-" mul)*
pub fn add<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
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

// This function represent following grammar.
// mul     = unary ("*" unary | "/" unary)*
pub fn mul<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = unary(tokenizer);

    loop {
        if consume(tokenizer,&TkMul){
            node = NdMul(Box::new(node),Box::new(unary(tokenizer)));
        }else if consume(tokenizer,&TkDiv) {
            node = NdDiv(Box::new(node),Box::new(unary(tokenizer)));
        }else{
            break node;
        }
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

// This function represents following grammar.
// unary    = ("+" | "-")?  primary
pub fn unary<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    if consume(tokenizer,&TkPlus){
        return primary(tokenizer);
    }
    if consume(tokenizer,&TkMinus){
        return NdSub(Box::new(NdNum(0)),Box::new(primary(tokenizer)))
    }

    return primary(tokenizer);
}


