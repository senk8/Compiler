use std::iter::Peekable;

use crate::parser::Parser;
use crate::lexer::Lexer;

use crate::error_handler::parse_error::ParseError;
use crate::error_handler::parse_error::ParseError::*;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::TokenKind::*;



pub(super) fn decl(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {

    log::info!("Parsing is entered 'decl' !");

    /* 引数コンパイルしたら同時にローカル変数の定義を行う。*/

    let _ = parser.take_type(lexer).ok_or_else(||
        UnexpectedToken(lexer.next().unwrap())
    )?;

    if let Some(Id(name)) = parser.take_id(lexer) {
        parser.expect(lexer,Delim(Lparen))?;

        let mut args = vec![];
        if !parser.choice(lexer,Delim(Rparen)) {
            loop {

                if let Some(ty) = parser.take_type(lexer){

                    let token = parser.take_token(lexer).ok_or(Eof)?;

                    if let (Id(name), _) = token {
                        parser.set_var(name, ty);
                        args.push(NdLVar(parser.offset()));

                        if !parser.choice(lexer,Delim(Comma)) {
                            parser.expect(lexer,Delim(Rparen))?;
                            break;
                        }

                    } else {
                        return Err(UnexpectedToken(token));
                    }
                }else{
                    panic!("Expect Type fonund");
                }
            }
        };

        parser.expect(lexer,Delim(Lbrace))?;

        let mut nodes = Vec::new();

        while !parser.choice(lexer,Delim(Rbrace)) {
            nodes.push(super::stmt::stmt(parser,lexer)?);
        }

        Ok(NdDecl(name, args, Box::new(NdBlock(nodes))))
    } else {
        log::error!("error occured at 'decl' !");
        /* TODO: it will be unused look_ahead.unwrap() */ 
        Err(UnexpectedToken(parser.look_ahead(lexer).unwrap()))
    }
}
