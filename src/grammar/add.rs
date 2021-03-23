use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;

use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;

use super::mul::mul;


// add    = mul ("+" mul | "-" mul)*
pub(super) fn add(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let mut node = mul(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Add)) {
            node = NdAdd(Box::new(node), Box::new(mul(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Sub)) {
            node = NdSub(Box::new(node), Box::new(mul(parser,lexer)?));
        } else {
            break Ok(node);
        }
    }
}