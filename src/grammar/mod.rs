mod add;
mod assign;
mod decl;
mod entry;
mod equality;
mod expr;
mod mul;
mod primary;
mod relational;
mod stmt;
mod unary;

use crate::lexer::Lexer;
use crate::parser::Parser;
use std::iter::Peekable;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;

pub fn parse(parser: &mut Parser, lexer: &mut Peekable<Lexer>) -> Result<Vec<Node>, ParseError> {
    entry::program(parser, lexer)
}
