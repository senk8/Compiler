use core::iter::Peekable;

use crate::tokenizer::tokenizer::Tokenizer;
use crate::types::token::TokenKind::*;
use crate::types::node::*;

use super::parse_util::*;
use super::expr::expr;

// program = stmt * 
pub fn program<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Vec<Node>{
    let mut trees = vec![];

    loop {
        if let Some(_) = tokenizer.peek() {
            trees.push(stmt(tokenizer));
        }else{
            break trees;
        }
    }
}

//stmt = expr ";"
pub fn stmt<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let node = expr(tokenizer);
    expect(tokenizer,Semicolon);
    node
}
