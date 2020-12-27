/*
use core::iter::Peekable;
use crate::types::variable::{LVar,SYMBOL_MAP};
use crate::types::token::TokenKind::*;
use crate::types::node::*;
use crate::types::node::Node::*;
use super::parse_util::*;

//expr = assign
pub fn expr<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    assign(tokenizer)
}

//assign = equality ( "=" assign )?
pub fn assign<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = equality(tokenizer);
    if consume(tokenizer,Assign) {
        node = NdAssign(Box::new(node),Box::new(assign(tokenizer)));
    }
    node
}

// equality = relational ("==" relational | "!=" relational)*
pub fn equality<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = relational(tokenizer);

    loop{
        if consume(tokenizer,Eq){
            node = NdEq(Box::new(node),Box::new(relational(tokenizer)));
        }else if consume(tokenizer,Neq){
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
        if consume(tokenizer,Lt){
            node = NdLt(Box::new(node),Box::new(add(tokenizer)));
        }else if consume(tokenizer,Leq){
            node = NdLeq(Box::new(node),Box::new(add(tokenizer)));
        }else if consume(tokenizer,Gt){
            node = NdLt(Box::new(add(tokenizer)),Box::new(node));
        }else if consume(tokenizer,Geq){
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
        if consume(tokenizer,Plus){
            node = NdAdd(Box::new(node),Box::new(mul(tokenizer)));
        }else if consume(tokenizer,Minus) {
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
        if consume(tokenizer,Mul){
            node = NdMul(Box::new(node),Box::new(unary(tokenizer)));
        }else if consume(tokenizer,Div) {
            node = NdDiv(Box::new(node),Box::new(unary(tokenizer)));
        }else{
            break node;
        }
    }
}

// This function represent following grammar.
// primary = num | ident | "(" expr ")"*
pub fn primary<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    if consume(tokenizer,Rc) {
        let node = expr(tokenizer);
        expect(tokenizer,Lc);
        return node;
    }

    let node = if let Some(name) = take_ident(tokenizer){

        let result = SYMBOL_MAP.with(|sm|{
            sm.borrow().get(&name).map(|x|x.clone())
        });

        let node = if let Some(lvar)= result {
            NdLVar(lvar.1)
        }else{
            let offset = usize::from(name.chars().next().unwrap() as u8-b'a');
            let lvar=LVar(name.len(),(offset+1)*8);

            SYMBOL_MAP.with(|sm|{
                sm.borrow_mut().insert(name,lvar.clone());
            });

            NdLVar((offset+1)*8)
        };

        return node;
    };

    NdNum(take_num(tokenizer).expect("Error! expect number,found other"))
}

// This function represents following grammar.
// unary    = ("+" | "-")?  primary
pub fn unary<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    if consume(tokenizer,Plus){
        return primary(tokenizer);
    }

    if consume(tokenizer,Minus){
        return NdSub(Box::new(NdNum(0)),Box::new(primary(tokenizer)))
    }

    return primary(tokenizer);
}
*/