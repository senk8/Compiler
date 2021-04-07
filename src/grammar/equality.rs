use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;

use super::relational::relational;


// equality = relational ("==" relational | "!=" relational)*
pub(super) fn equality(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {

    log::info!("Parsing is entered 'equality' !");

    let mut node = relational(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Eq)) {
            node = NdEq(Box::new(node), Box::new(relational(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Neq)) {
            node = NdNeq(Box::new(node), Box::new(relational(parser,lexer)?));
        } else {
            break Ok(node);
        }
    }
}
