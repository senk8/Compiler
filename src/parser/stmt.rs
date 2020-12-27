use core::iter::Peekable;

use crate::tokenizer::tokenizer::Tokenizer;
use crate::types::token::TokenKind::*;
use crate::types::node::*;

use super::parse_util::*;
use super::expr::expr;

// program = stmt *
pub fn program<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Vec<Node>{
    let mut trees = vec![];

    while let Some(_) = tokenizer.peek(){
        trees.push(stmt(tokenizer));
    }

    trees
}

/* stmt = expr ";" | "return" expr ";" */
pub fn stmt<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{

    /*
    let node = if consume_keywoed(Return) {
        NdReturn(expr(tokenizer));
    }else{
        expr(tokenizer);
    };
    */

    let node = expr(tokenizer);
    expect(tokenizer,Semicolon);
    node
}
