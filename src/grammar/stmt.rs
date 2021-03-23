use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::TokenKind::*;

use super::expr::expr;

/// stmt = expr ";"
/// | "{" stmt* "}""
/// | "return" expr ";"
/// | "if" "(" expr ")" stmt ("else" stmt)?
/// | "while" "(" expr ")" stmt
/// | "for" "(" expr? ";" expr? ";" expr? ")" stmt
pub(super) fn stmt(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    /* choice expr or return */

    if parser.choice(lexer,Key(Return)) {
        let node = NdReturn(Box::new(expr(parser,lexer)?));
        parser.expect(lexer,Delim(Semicolon))?;
        Ok(node)
    } else if parser.choice(lexer,Delim(Lbrace)) {
        let mut nodes = Vec::new();
        while !parser.choice(lexer,Delim(Rbrace)) {
            nodes.push(stmt(parser,lexer)?);
        }
        Ok(NdBlock(nodes))
    } else if parser.choice(lexer,Key(If)) {
        parser.expect(lexer,Delim(Lparen))?;
        let first = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Rparen))?;
        let second = stmt(parser,lexer)?;

        if parser.choice(lexer,Key(Else)) {
            let third = stmt(parser,lexer)?;
            Ok(NdIfElse(Box::new(first), Box::new(second), Box::new(third)))
        } else {
            Ok(NdIf(Box::new(first), Box::new(second)))
        }
    } else if parser.choice(lexer,Key(While)) {
        parser.expect(lexer,Delim(Lparen))?;
        let first = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Rparen))?;
        let second = stmt(parser,lexer)?;
        Ok(NdWhile(Box::new(first), Box::new(second)))
    } else if parser.choice(lexer,Key(For)) {
        parser.expect(lexer,Delim(Lparen))?;
        let first = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Semicolon))?;
        let second = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Semicolon))?;
        let third = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Rparen))?;
        let fourth = stmt(parser,lexer)?;
        Ok(NdFor(
            Box::new(first),
            Box::new(second),
            Box::new(third),
            Box::new(fourth),
        ))
    } else {
        let node = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Semicolon))?;
        Ok(node)
    }
}