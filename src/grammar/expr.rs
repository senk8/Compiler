use std::iter::Peekable;

use crate::Parser;
use crate::Lexer;

use crate::types::error::ParseError;
use crate::types::error::ParseError::*;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::TokenKind::*;


use super::assign::assign;


//expr = assign | type ident
pub(super) fn expr(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    if let Some(ty) = parser.take_type(lexer){
        let token = parser.take_token(lexer).ok_or(Eof)?;

        if let (Id(name), _) = token {
            parser.set_var(name, ty);
            Ok(NdVdecl(parser.offset()))
        } else {
            Err(UnexpectedToken(token))
        }
    } else {
        assign(parser,lexer)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
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

        let result = expr(&mut parser,&mut lexer)?;

        dbg!(&result);

        assert_eq!(result, 
            node!(NdAdd,NdNum(2),NdNum(1))
        );

        Ok(())
    }


}