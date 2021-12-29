use crate::lexer::Lexer;
use crate::parser::Parser;
use std::iter::Peekable;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;

pub(super) fn program(
    parser: &mut Parser,
    lexer: &mut Peekable<Lexer>,
) -> Result<Vec<Node>, ParseError> {
    log::info!("Parsing is entered 'program' !");
    let mut trees = vec![];

    while parser.look_ahead(lexer).is_some() {
        trees.push(super::decl::decl(parser, lexer)?);
        parser.reset_table();
    }

    Ok(trees)
}
