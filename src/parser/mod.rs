pub mod expr;
pub mod stmt;

use std::collections::HashMap;
use std::cell::RefCell;
use core::iter::Peekable;

use crate::tokenizer::*;

use crate::types::token::*;
use crate::types::token::TokenType::*;
use crate::types::token::TokenKind::*;

use crate::types::node::*;
use crate::types::node::Node::*;

#[derive(Debug,PartialEq,Clone)]
pub struct LVar(pub usize,pub usize);

pub struct Parser<'a>{
    /* mutable field for symbol table */
    symbol_table : RefCell<HashMap<String,LVar>>,
    offset : usize,

    /* mutable field for tokenizer */
    tokenizer : RefCell<Peekable<Tokenizer<'a>>>,
}


impl<'a> Parser<'a>{
    pub fn new(tokenizer:Peekable<Tokenizer<'a>>)->Parser<'a>{
        Parser { tokenizer:RefCell::new(tokenizer),
                symbol_table:RefCell::new(HashMap::new()),
                offset:0
            }
    }
    pub fn set_var(&mut self,name:String)->(){
        self.offset = self.offset +  8 ;
        self.symbol_table.borrow_mut().insert(name.clone(),LVar(name.len(),self.offset));
    }

    pub fn look_ahead(&mut self)->Option<TokenType>{
        self.tokenizer.borrow_mut().peek().cloned()
    }

    pub fn next_token(&mut self)->Option<TokenType>{
        self.tokenizer.borrow_mut().next()
    }

    pub fn parse(&mut self)->Vec<Node>{
        self.program()
    }
}


impl<'a> Parser<'a>{

    fn take_num(&mut self) -> Option<usize>{
        if let Some(Num(_)) = self.look_ahead() {
            match self.next_token() {
                Some(Num(n)) => Some(n),
                _ => None,
            }
        }else{
            None
        }
    }

    fn take_ident(&mut self) -> Option<String>{
        if let Some(Ident(_)) = self.look_ahead() {
            match self.next_token(){
                Some(Ident(s)) => Some(s),
                _ => None,
            }
        }else{
            None
        }
    }

    fn expect(&mut self,expect_token:TokenKind)->(){
        if let Some(Token(kind))=self.look_ahead() {
            if kind != expect_token {
                panic!("Error! expect number,found other");
            }
        }
        self.next_token();
    }

    fn consume(&mut self,expect_token:TokenKind)->bool {
        if let Some(Token(kind))=self.look_ahead(){
            if kind == expect_token {
                self.next_token();
                return true;
            }else{
                return false;
            }
        }
        return false;
    }
}