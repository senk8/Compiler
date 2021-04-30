use std::iter::Peekable;

use crate::lexer::Lexer;
use crate::parser::Parser;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::tokenize::OperatorKind::*;
use crate::types::tokenize::TokenKind::*;

use super::primary::primary;

/*
unary    = "+" primary
        |  "-" primary
        |  "*" primary
        |  "&" primary
        |  "sizeof" primary
        |  primary
*/
pub(super) fn unary(parser: &mut Parser, lexer: &mut Peekable<Lexer>) -> Result<Node, ParseError> {
    log::info!("Parsing is entered 'unary' !");

    if parser.choice(lexer, Opr(Add)) {
        primary(parser, lexer)
    } else if parser.choice(lexer, Opr(Sub)) {
        Ok(NdSub(Box::new(NdNum(0)), Box::new(primary(parser, lexer)?)))
    } else if parser.choice(lexer, Opr(Star)) {
        Ok(NdDeref(Box::new(primary(parser, lexer)?)))
    } else if parser.choice(lexer, Opr(Amp)) {
        Ok(NdRef(Box::new(primary(parser, lexer)?)))
    } else if parser.choice(lexer,Opr(Sizeof)) {
        Ok(NdSizeof(Box::new(primary(parser, lexer)?)))
    } else {
        primary(parser, lexer)
    }
}
