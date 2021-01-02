pub mod expr;
pub mod stmt;

use core::iter::Peekable;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::lexer::*;

use crate::types::token::TokenType::*;
use crate::types::token::*;

use crate::types::node::Node::*;
use crate::types::node::*;

use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LVar(pub usize, pub usize);

pub struct Parser<'a> {
    /* mutable field for symbol table */
    symbol_table: RefCell<HashMap<String, LVar>>,
    offset: usize,

    /* mutable field for tokenizer */
    tokenizer: RefCell<Peekable<Lexer<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Peekable<Lexer<'a>>) -> Parser<'a> {
        Parser {
            tokenizer: RefCell::new(lexer),
            symbol_table: RefCell::new(HashMap::new()),
            offset: 0,
        }
    }
    pub fn set_var(&mut self, name: String) -> () {
        self.offset = self.offset + 8;
        self.symbol_table
            .borrow_mut()
            .insert(name.clone(), LVar(name.len(), self.offset));
    }

    pub fn look_ahead(&self) -> Result<TokenType,ParseError> {
        self.tokenizer.borrow_mut().peek().cloned().ok_or(Eof)
    }

    pub fn next_token(&self) -> Result<TokenType,ParseError> {
        self.tokenizer.borrow_mut().next().ok_or(Eof)
    }

    pub fn parse(&mut self) -> Result<Vec<Node>,ParseError> {
        self.program()
    }
}

/*

impl<'a> Parser<'a> {
    /* consume methods require that if it unuse wrap value */

    fn expect_token(&mut self, expect: TokenKind) -> Result<TokenType,ParseError> {
        self.look_ahead().and_then(|tok|{
            match tok {
               Token(kind) if kind == expect => self.next_token(),
               Token(kind) => Err(UnexpectedToken),
               _ => Err(UnexpectedToken),
            }
        })
    }

    fn expect_keyword(&mut self, expect: KeywordKind) -> Result<TokenType,ParseError> {
        self.look_ahead().and_then(|tok|{
            match tok {
               Keyword(kind) if kind == expect => self.next_token(),
               Keyword(kind) => Err(UnexpectedKeyword),
               _ => Err(UnexpectedKeyword),
            }
        })
    }

    fn expect_num(&mut self) -> Result<usize,ParseError> {
        self.look_ahead().and_then(|tok|{
            match tok {
               Num(_) => self.next_token().map(|tok|match tok{
                   Num(n) => n,
                   _ => unreachable!(),
               }),
               _ => Err(ExpectedNumeric),
            }
        })
    }

    fn expect_id(&mut self) -> Result<String,ParseError> {
        self.look_ahead().and_then(|tok|{
            match tok {
               Id(_) => self.next_token().map(|tok|match tok{
                   Id(s) => s,
                   _ => unreachable!(),
               }),
               _ => Err(ExpectedIdentifier),
            }
        })
   }

    fn expect_delimitor(&mut self, expect: TokenKind) -> Result<TokenType,ParseError> {
        self.look_ahead().and_then(|tok|{
            match tok {
               Token(kind) if kind == expect => self.next_token(),
               Token(kind) => Err(UnexpectedDelimitor),
               _ => Err(UnclosedDelimitor),
            }
        })
   }
}
*/