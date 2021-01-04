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

use crate::types::error::ParseError;
use crate::types::error::ParseError::*;


#[derive(Debug, PartialEq, Clone)]
pub struct LVar(pub usize, pub usize);

pub struct Parser<'a> {
    /* mutable field for symbol table */
    symbol_table: RefCell<HashMap<String, LVar>>,
    offset: Cell<usize>,

    /* mutable field for tokenizer */
    lexer: RefCell<Peekable<Lexer<'a>>>,
    input: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let input = lexer.get_txt();
        let ll_1_lexer = lexer.peekable();

        Parser {
            lexer: RefCell::new(ll_1_lexer),
            symbol_table: RefCell::new(HashMap::new()),
            offset: Cell::new(0),
            input: input
        }
    }
    pub fn set_var(&self, name: String) -> () {
        self.offset.set(self.offset.get() + 8);
        self.symbol_table
            .borrow_mut()
            .insert(name.clone(), LVar(name.len(), self.offset.get()));
    }

    pub fn look_ahead(&self) -> Result<Token,ParseError> {
        self.lexer.borrow_mut().peek().cloned().ok_or(Eof(Pos(0,0)," ".to_owned()))
    }

    pub fn next_token(&self) -> Result<Token,ParseError> {
        self.lexer.borrow_mut().next().ok_or(Eof(Pos(0,0)," ".to_owned()))
    }

    pub fn parse(&self) -> Result<Vec<Node>,ParseError> {
        self.program()
    }

    pub fn raise_error(&self,pos:Pos)->Result<Node,ParseError> {
        let string_input = std::str::from_utf8(self.input).map(|s|String::from(s)).unwrap();
        Err(Eof(pos,string_input))
    }
    
    fn new_opr(&self,lhs:Node,rhs:Node)->Result<Node,ParseError>{
        use crate::types::token::TokenKind::*;
        use crate::types::token::OperatorKind::*;

        let x = Ok(match self.next_token()?.val {
            Opr(Add) => NdAdd(Box::new(lhs), Box::new(rhs)),
            Opr(Sub) => NdSub(Box::new(lhs), Box::new(rhs)),
            Opr(Mul) => NdMul(Box::new(lhs), Box::new(rhs)),
            Opr(Div) => NdDiv(Box::new(lhs), Box::new(rhs)),
            Opr(Assign) => NdAssign(Box::new(lhs), Box::new(rhs)),
            Opr(Lt) => NdLt(Box::new(lhs), Box::new(rhs)),
            Opr(Gt) => NdLt(Box::new(lhs), Box::new(rhs)),
            Opr(Leq) => NdLeq(Box::new(lhs), Box::new(rhs)),
            Opr(Geq) => NdLeq(Box::new(lhs), Box::new(rhs)),
            Opr(Eq) => NdEq(Box::new(lhs), Box::new(rhs)),
            Opr(Neq) => NdNeq(Box::new(lhs), Box::new(rhs)),
            x => panic!("{:?}",x),
        });
        println!("{:?}",x);
        x
    }
}

mod tests{
    use super::*;

    #[allow(dead_code)]
    macro_rules! new_node {
        ($f:ident,$lhs:expr,$rhs:expr) => {
            $f(Box::new($lhs),Box::new($rhs))
        };
    }

    #[test]
    fn test_parse_arithmetic()->Result<(),ParseError>{

        let input = "2+1;2-1;2*1;2/1;2+3*3/3-1;";
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        println!("{:?}",new_node!(NdAdd,NdNum(2),NdNum(1)));

        let answer = vec![

            new_node!(NdAdd,NdNum(2),NdNum(1)),
            new_node!(NdSub,NdNum(2),NdNum(1)),
            new_node!(NdMul,NdNum(2),NdNum(1)),
            new_node!(NdDiv,NdNum(2),NdNum(1)),

            new_node!(NdSub,
                new_node!(NdAdd,
                    NdNum(2),
                    new_node!(NdDiv,
                        new_node!(NdMul,
                            NdNum(3),
                            NdNum(3)
                        ),
                        NdNum(3)
                    )
                ),
                NdNum(1)
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
        let lexer = Lexer::new(input);
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
        let lexer = Lexer::new(input);
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
        let lexer = Lexer::new(input);
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
