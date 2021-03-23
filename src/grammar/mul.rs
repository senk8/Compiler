use std::iter::Peekable;

use crate::Parser;
use crate::Lexer;

use crate::types::error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;

use super::unary::unary;

// mul     = unary ("*" unary | "/" unary)*
pub(super) fn mul(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let mut node = unary(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Star)) {
            node = NdMul(Box::new(node), Box::new(unary(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Div)) {
            node = NdDiv(Box::new(node), Box::new(unary(parser,lexer)?));
        } else {
            break Ok(node);
        }
    }
}