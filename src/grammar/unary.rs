use std::iter::Peekable;

use crate::Parser;
use crate::Lexer;

use crate::types::error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;

use super::primary::primary;

/*
unary    = "+" primary
        |  "-" primary
        |  "*" primary
        |  "&" primary
        |  "sizeof" primary
        |  primary
*/
pub(super) fn unary(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    if parser.choice(lexer,Opr(Add)) {
        primary(parser,lexer)
    } else if parser.choice(lexer,Opr(Sub)) {
        Ok(NdSub(Box::new(NdNum(0)), Box::new(primary(parser,lexer)?)))
    } else if parser.choice(lexer,Opr(Star)) {
        Ok(NdDeref(Box::new(primary(parser,lexer)?)))
    } else if parser.choice(lexer,Opr(Amp)) {
        Ok(NdRef(Box::new(primary(parser,lexer)?)))
    /*
    } else if self.choice(Opr(Sizeof)) {
        let node = self.primary()?;

        match node {
            NdNum(_) => NdNum(4),
            Nd(_) => NdNum(4),
        }

        Ok(NdSizeof())
    */
    } else {
        primary(parser,lexer)
    }
}