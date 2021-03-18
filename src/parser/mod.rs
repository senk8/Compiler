pub mod grammar;
pub mod parser_impl;

use crate::lexer::Lexer;
use crate::types::variable::LVar;

use core::iter::Peekable;
use std::collections::HashMap;

pub struct Parser<'a> {
    /* mutable field for symbol table */
    symbol_table: HashMap<String, LVar>,
    offset: usize,

    /* mutable field for tokenizer */
    lexer: Peekable<Lexer<'a>>,
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::types::node::Node::*;
    use anyhow::Result;

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
    fn test_parse_arithmetic() -> Result<()> {
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
    fn test_parse_relatinonal() -> Result<()> {
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
    fn test_parse_variable() -> Result<()> {
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
    fn test_parse_keyword() -> Result<()> {
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
    fn test_parse_block() -> Result<()> {
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
