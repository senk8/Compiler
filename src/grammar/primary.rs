use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::error_handler::parse_error::ParseError::*;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::TokenKind::*;

use super::expr::expr;

// primary = num | ident | "(" expr ")" | ident ( "(" argument ")" )?
// argument = (expr ( "," expr )* ) ?
pub(super) fn primary(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {

    log::info!("Parsing is entered 'primary' !");

    if let Some(Num(n)) = parser.take_num(lexer) {
        Ok(NdNum(n))
    } else if let Some(Id(name)) = parser.take_id(lexer) {
        if parser.choice(lexer,Delim(Lparen)) {
            let mut args = vec![];

            /* exprにマッチすることを先読みできないので、")"がないかどうかを選択肢にしている。 */
            if !parser.choice(lexer,Delim(Rparen)) {
                args.push(expr(parser,lexer)?);
                loop {
                    if parser.choice(lexer,Delim(Comma)) {
                        args.push(expr(parser,lexer)?);
                    } else {
                        break;
                    };
                }
                parser.expect(lexer,Delim(Rparen))?;
            }

            Ok(NdCall(name.to_string(), args))
        } else {
            let result = parser.find_var(name);

            if let Some(lvar) = result {
                Ok(NdLVar(lvar.0))
            } else {
                log::error!("error occured at 'primary'!");
                Err(UndefinedSymbol(lexer.next().unwrap()))
            }
        }
    } else if parser.choice(lexer,Delim(Lparen)) {
        let node = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Rparen))?;
        Ok(node)
    } else {
        log::error!("error occured at 'primary'!");
        Err(UnexpectedToken(lexer.next().unwrap()))
    }
}
