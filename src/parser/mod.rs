pub mod expr;
pub mod stmt;

use core::iter::Peekable;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::lexer::*;

use crate::types::token::*;

use crate::types::node::Node::*;
use crate::types::node::*;

use crate::types::annotation::Pos;

use crate::types::error::ParseError::*;
use crate::types::error::{ParseError, ParseErrorKind};

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
            input: input,
        }
    }
    pub fn set_var(&self, name: String) -> () {
        self.offset.set(self.offset.get() + 8);
        self.symbol_table
            .borrow_mut()
            .insert(name.clone(), LVar(name.len(), self.offset.get()));
    }

    pub fn look_ahead(&self) -> Option<Token> {
        self.lexer.borrow_mut().peek().cloned()
    }

    pub fn consume(&self) -> Option<Token> {
        self.lexer.borrow_mut().next()
    }

    pub fn parse(&self) -> Result<Vec<Node>, ParseError> {
        self.program()
    }

    pub fn raise_error(&self, kind: ParseErrorKind, pos: Pos) -> Result<Node, ParseError> {
        use crate::types::error::ParseErrorKind::*;

        let begin = pos.0;
        let mut message = std::str::from_utf8(self.input)
            .map(|s| String::from(s))
            .unwrap();
        message.push_str(&format!("\n{:>width$}\n", "^", width = begin + 1));

        Err(match kind {
            UnexpectedToken => UnexpectedTokenError(pos, message),
            UnclosedDelimitor => UnclosedDelimitorError(pos, message),
            UnexpectedKeyword => UnexpectedKeywordError(pos, message),
            UnexpectedDelimitor => UnexpectedDelimitorError(pos, message),
            Eof => EofError(pos, message),
            LackSemicolon => MissingExpressionError(pos, message),
            LackExpr => MissingSemicolonError(pos, message),
            _ => SegmentationFault(pos, message),
        })
    }

    pub fn make_error(&self, kind: ParseErrorKind) -> ParseError {
        use crate::types::error::ParseErrorKind::*;
        let pos = Pos(self.input.len(), self.input.len());
        let mut message = std::str::from_utf8(self.input)
            .map(|s| String::from(s))
            .unwrap();
        message.push_str(&format!("\n{:>width$}\n", "^", width = pos.0 + 1));

        match kind {
            UnexpectedToken => UnexpectedTokenError(pos, message),
            UnclosedDelimitor => UnclosedDelimitorError(pos, message),
            UnexpectedKeyword => UnexpectedKeywordError(pos, message),
            UnexpectedDelimitor => UnexpectedDelimitorError(pos, message),
            Eof => EofError(pos, message),
            LackSemicolon => MissingSemicolonError(pos, message),
            LackExpr => MissingExpressionError(pos, message),
            _ => SegmentationFault(pos, message),
        }
    }

    pub(super) fn expect_tk(&self, kind: TokenKind) -> Result<(), ParseError> {
        use crate::types::error::ParseErrorKind::*;
        self.look_ahead()
            .ok_or(self.make_error(Eof))
            .and_then(|tok| {
                if tok.0 == kind {
                    self.consume();
                    Ok(())
                } else {
                    Err(self.make_error(UnexpectedToken))
                }
            })
    }

    /*
    fn new_opr(&self,lhs:Node,rhs:Node)->Result<Node,ParseError>{
        use crate::types::token::TokenKind::*;
        use crate::types::token::OperatorKind::*;

        Ok(match self.consume().unwrap().val {
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
            _ => unreachable!(),
        })

    }
    */
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! node {
        ($f:ident,$lhs:expr,$rhs:expr) => {
            $f(Box::new($lhs), Box::new($rhs))
        };
        ($f:ident,$lhs:expr) => {
            $f(Box::new($lhs))
        };
    }

    #[test]
    fn test_parse_arithmetic() -> Result<(), ParseError> {
        let input = "2+1;2-1;2*1;2/1;2+3*3/3-1;";
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        println!("{:?}", node!(NdAdd, NdNum(2), NdNum(1)));
        println!("{:?}", node!(NdAdd, NdNum(2), NdNum(1)));

        let answer = vec![
            node!(NdAdd, NdNum(2), NdNum(1)),
            node!(NdSub, NdNum(2), NdNum(1)),
            node!(NdMul, NdNum(2), NdNum(1)),
            node!(NdDiv, NdNum(2), NdNum(1)),
            node!(
                NdSub,
                node!(
                    NdAdd,
                    NdNum(2),
                    node!(NdDiv, node!(NdMul, NdNum(3), NdNum(3)), NdNum(3))
                ),
                NdNum(1)
            ),
        ];

        for (tree, ans) in result.into_iter().zip(answer.into_iter()) {
            assert_eq!(tree, ans);
        }

        Ok(())
    }

    #[test]
    fn test_parse_relatinonal() -> Result<(), ParseError> {
        let input = "2<3;2>3;2<=3;2>=3;2==3;2!=3;";
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            node!(NdLt, NdNum(2), NdNum(3)),
            node!(NdLt, NdNum(3), NdNum(2)),
            node!(NdLeq, NdNum(2), NdNum(3)),
            node!(NdLeq, NdNum(3), NdNum(2)),
            node!(NdEq, NdNum(2), NdNum(3)),
            node!(NdNeq, NdNum(2), NdNum(3)),
        ];

        for (tree, ans) in result.into_iter().zip(answer.into_iter()) {
            assert_eq!(tree, ans);
        }

        Ok(())
    }

    #[test]
    fn test_parse_variable() -> Result<(), ParseError> {
        let input = "a=2;b=3;a*b;";
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            node!(NdAssign, NdLVar(8), NdNum(2)),
            node!(NdAssign, NdLVar(16), NdNum(3)),
            node!(NdMul, NdLVar(8), NdLVar(16)),
        ];

        for (tree, ans) in result.into_iter().zip(answer.into_iter()) {
            assert_eq!(tree, ans);
        }

        Ok(())
    }

    #[test]
    fn test_parse_keyword() -> Result<(), ParseError> {
        let input = "return 2*2;return 2==2;";
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            node!(NdReturn, node!(NdMul, NdNum(2), NdNum(2))),
            node!(NdReturn, node!(NdEq, NdNum(2), NdNum(2))),
        ];

        for (tree, ans) in result.into_iter().zip(answer.into_iter()) {
            assert_eq!(tree, ans);
        }

        Ok(())
    }
}
