use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::tokenize::OperatorKind::*;
use crate::types::tokenize::TokenKind::*;

use super::unary::unary;

// mul     = unary ("*" unary | "/" unary)*
pub(super) fn mul(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    log::info!("Parsing is entered 'mul' !");
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