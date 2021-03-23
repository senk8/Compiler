mod entry;
mod decl;
mod stmt;
mod equality;
mod assign;
mod add;
mod mul;
mod primary;
mod unary;
mod relational;
mod expr;

use std::iter::Peekable;
use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;

pub fn parse(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Vec<Node>,ParseError> {
    entry::program(parser,lexer)
}


