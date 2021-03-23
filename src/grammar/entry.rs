use std::iter::Peekable;
use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;

pub(super) fn program(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Vec<Node>, ParseError> {
    let mut trees = vec![];

    while let Some(_) = parser.look_ahead(lexer) {
        trees.push(super::decl::decl(parser,lexer)?);
        parser.reset_vars();
    }

    Ok(trees)
}