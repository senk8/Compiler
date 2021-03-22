pub mod grammar;
pub mod parser_impl;

use crate::lexer::Lexer;
use crate::types::variable::LVar;

use core::iter::Peekable;
use std::collections::HashMap;

pub struct Parser {
    /* mutable field for symbol table */
    symbol_table: HashMap<String, LVar>,
    offset: usize,

    /* mutable field for tokenizer */
    //lexer: Peekable<Lexer<'a>>,
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
        let input = "2+1;";
        let mut lexer = Lexer::new(input.as_bytes()).peekable();
        let mut parser = Parser::new();

        let result = parser.expr(&mut lexer)?;

        dbg!(&result);

        assert_eq!(result, vec![
            node!(NdAdd,NdNum(2),NdNum(1))
        ]);

        Ok(())
    }


}