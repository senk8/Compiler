use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::tokenize::OperatorKind::*;
use crate::types::tokenize::TokenKind::*;

use super::add::add;

pub(super) fn relational(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {

    log::info!("Parsing is entered 'relational' !");

    let mut node = add(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Lt)) {
            node = NdLt(Box::new(node), Box::new(add(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Leq)) {
            node = NdLeq(Box::new(node), Box::new(add(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Gt)) {
            node = NdLt(Box::new(add(parser,lexer)?), Box::new(node));
        } else if parser.choice(lexer,Opr(Geq)) {
            node = NdLeq(Box::new(add(parser,lexer)?), Box::new(node));
        } else {
            break Ok(node);
        }
    }
}