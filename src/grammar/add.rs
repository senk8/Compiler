use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;

use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::tokenize::OperatorKind::*;
use crate::types::tokenize::TokenKind::*;

use super::mul::mul;


// add    = mul ("+" mul | "-" mul)*
pub(super) fn add(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {

    log::info!("Parsing is entered 'add' !");

    /* MEMO: nodeがNdLVarの場合、その型を調べて＋１を別の値に置き換える。という風にできそう。 */
    let mut node = mul(parser,lexer)?;

    //if let NdLVar() = node

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