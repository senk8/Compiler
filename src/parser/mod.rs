pub mod expr;
pub mod stmt;

use core::iter::Peekable;
use std::cell::{RefCell,Cell};
use std::collections::HashMap;

use crate::lexer::*;

use crate::types::token::*;

use crate::types::node::Node::*;
use crate::types::node::*;

use crate::types::annotation::Pos;

use crate::types::error::{ParseError,ParseErrorKind};
use crate::types::error::ParseErrorKind::*;


#[derive(Debug, PartialEq, Clone)]
pub struct LVar(pub usize, pub usize);

pub struct Parser<'a> {
    /* mutable field for symbol table */
    symbol_table: RefCell<HashMap<String, LVar>>,
    offset: Cell<usize>,

    /* mutable field for tokenizer */
    tokenizer: RefCell<Peekable<Lexer<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Peekable<Lexer<'a>>) -> Parser<'a> {
        Parser {
            tokenizer: RefCell::new(lexer),
            symbol_table: RefCell::new(HashMap::new()),
            offset: Cell::new(0),
        }
    }
    pub fn set_var(&self, name: String) -> () {
        self.offset.set(self.offset.get() + 8);
        self.symbol_table
            .borrow_mut()
            .insert(name.clone(), LVar(name.len(), self.offset.get()));
    }

    pub fn look_ahead(&self) -> Result<Token,ParseError> {
        self.tokenizer.borrow_mut().peek().cloned().ok_or(ParseError{val:Eof,pos:Pos(0,0)})
    }

    pub fn next_token(&self) -> Result<Token,ParseError> {
        self.tokenizer.borrow_mut().next().ok_or(ParseError{val:Eof,pos:Pos(0,0)})
    }

    pub fn parse(&self) -> Result<Vec<Node>,ParseError> {
        self.program()
    }

    pub fn raise_error(&self,val:ParseErrorKind,pos:Pos)->Result<Node,ParseError> {
        Err(ParseError{val,pos})
    }
}

mod tests{
    use super::*;

    #[test]
    fn test_parse_arithmetic()->Result<(),ParseError>{
        let input = "2+1;2-1;2*1;2/1;2+3*3/3-1;";
        let lexer = Lexer::new(input).peekable();
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            NdAdd(Box::new(NdNum(2)),Box::new(NdNum(1))),
            NdSub(Box::new(NdNum(2)),Box::new(NdNum(1))),
            NdMul(Box::new(NdNum(2)),Box::new(NdNum(1))),
            NdDiv(Box::new(NdNum(2)),Box::new(NdNum(1))),
            NdSub(
                Box::new(NdAdd(
                    Box::new(NdNum(2)),
                    Box::new(NdDiv(
                        Box::new(NdMul(Box::new(NdNum(3)),Box::new(NdNum(3)))),
                        Box::new(NdNum(3))
                    ))
                )),
                Box::new(NdNum(1))
            )
        ];

        for (tree,ans) in result.into_iter().zip(answer.into_iter()){
            assert_eq!(tree,ans);
        }

        Ok(())
    }

    #[test]
    fn test_parse_relatinonal()->Result<(),ParseError>{
        let input = "2<3;2>3;2<=3;2>=3;2==3;2!=3;";
        let lexer = Lexer::new(input).peekable();
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            NdLt(Box::new(NdNum(2)),Box::new(NdNum(3))),
            NdLt(Box::new(NdNum(3)),Box::new(NdNum(2))),
            NdLeq(Box::new(NdNum(2)),Box::new(NdNum(3))),
            NdLeq(Box::new(NdNum(3)),Box::new(NdNum(2))),
            NdEq(Box::new(NdNum(2)),Box::new(NdNum(3))),
            NdNeq(Box::new(NdNum(2)),Box::new(NdNum(3))),
            /*
            NdEq(
                Box::new(NdLeq(Box::new(NdNum(2)),
                            Box::new(NdNum(3)))
                ),
                Box::new(NdLeq(Box::new(NdNum(2)),
                            Box::new(NdNum(3))),
                )
            )
            */
        ];

        for (tree,ans) in result.into_iter().zip(answer.into_iter()){
            assert_eq!(tree,ans);
        }

        Ok(())
    }

    #[test]
    fn test_parse_variable()->Result<(),ParseError>{
        let input = "a=2;b=3;a*b;";
        let lexer = Lexer::new(input).peekable();
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            NdAssign(Box::new(NdLVar(8)),Box::new(NdNum(2))),
            NdAssign(Box::new(NdLVar(16)),Box::new(NdNum(3))),
            NdMul(Box::new(NdLVar(8)),Box::new(NdLVar(16))),
        ];

        for (tree,ans) in result.into_iter().zip(answer.into_iter()){
            assert_eq!(tree,ans);
        }

        Ok(())
    }

    #[test]
    fn test_parse_keyword()->Result<(),ParseError>{
        let input = "return 2*2;return 2==2;";
        let lexer = Lexer::new(input).peekable();
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            NdReturn(Box::new(NdMul(Box::new(NdNum(2)),Box::new(NdNum(2))))),
            NdReturn(Box::new(NdEq(Box::new(NdNum(2)),Box::new(NdNum(2)))))
        ];

        for (tree,ans) in result.into_iter().zip(answer.into_iter()){
            assert_eq!(tree,ans);
        }

        Ok(())
    }
}
