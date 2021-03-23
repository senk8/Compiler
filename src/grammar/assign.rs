use std::iter::Peekable;

use crate::Parser;
use crate::Lexer;

use crate::types::error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;

use super::equality::equality;

//assign = equality ( "=" assign )?
pub(super) fn assign(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let node = equality(parser,lexer)?;

    if parser.choice(lexer,Opr(Assign)) {
        Ok(NdAssign(Box::new(node), Box::new(assign(parser,lexer)?)))
    } else {
        Ok(node)
    }
}