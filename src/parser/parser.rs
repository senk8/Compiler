use crate::lexer::*;
use core::iter::Peekable;
use std::collections::HashMap;

use crate::types::node::Node;
use crate::types::token::TokenKind::*;
use crate::types::token::TypeKind::*;
use crate::types::token::*;

use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

use crate::types::types::LVar;
use crate::types::types::VarAnnot;

pub struct Parser<'a> {
    /* mutable field for symbol table */
    pub(super) symbol_table: HashMap<String, LVar>,
    pub(super) offset: usize,

    /* mutable field for tokenizer */
    pub(super) lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let ll_1_lexer = lexer.peekable();

        Parser {
            lexer: ll_1_lexer,
            symbol_table: HashMap::new(),
            offset: 0,
        }
    }
    pub fn set_var(&mut self, name: String, ty: VarAnnot) -> () {
        self.offset += 8;
        self.symbol_table
            .insert(name.clone(), LVar(self.offset, ty));
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ParseError> {
        self.program()
    }

    pub fn look_ahead(&mut self) -> Option<Token> {
        //TODO : check it out. Whether we implement Deref trait for Token.
        self.lexer.peek().cloned()
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        self.look_ahead()
            .ok_or(Eof(Default::default()))
            .and_then(|tk| {
                if tk.0 == kind {
                    self.lexer.next();
                    Ok(())
                } else {
                    match tk.0 {
                        Type(Int) => Err(UnexpectedToken(tk.1)),
                        _ => Err(UnexpectedDelimitor(tk.1)),
                    }
                }
            })
    }

    pub(super) fn take_id(&mut self) -> Option<TokenKind> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(Id(_)) => self.lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    pub(super) fn take_type(&mut self) -> Option<TokenKind> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(Key(_)) => self.lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    pub(super) fn take_num(&mut self) -> Option<TokenKind> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(Num(_)) => self.lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    pub(super) fn take_token(&mut self) -> Option<Token> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(_) => self.lexer.next(),
            _ => None,
        }
    }

    pub(super) fn choice(&mut self, kind: TokenKind) -> bool {
        match self.look_ahead().map(|tk| tk.0) {
            Some(k) if k == kind => {
                self.lexer.next();
                true
            }
            _ => false,
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::types::node::Node::*;

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
        let mut lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);

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
        let mut lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);

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
        let mut lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);

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
        let mut lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);

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

    #[test]
    fn test_parse_block() -> Result<(), ParseError> {
        let input = "{return 2;}{2==2;}";
        let mut lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);

        let result = parser.parse()?;

        let answer = vec![
            NdBlock(vec![node!(NdReturn, NdNum(2))]),
            NdBlock(vec![node!(NdEq, NdNum(2), NdNum(2))]),
        ];

        for (tree, ans) in result.into_iter().zip(answer.into_iter()) {
            assert_eq!(tree, ans);
        }

        Ok(())
    }
}
