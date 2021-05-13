use std::iter::Peekable;

use crate::lexer::Lexer;
use crate::parser::Parser;

use crate::error_handler::parse_error::ParseError;
use crate::error_handler::parse_error::ParseError::*;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::parse::TypeInfo;
use crate::types::tokenize::DelimitorKind::*;
use crate::types::tokenize::TokenKind::*;

use super::assign::assign;

//expr = assign | type ident ([ add ])?
pub(super) fn expr(parser: &mut Parser, lexer: &mut Peekable<Lexer>) -> Result<Node, ParseError> {
    log::info!("Parsing is entered 'expr' !");
    if let Some(mut type_) = parser.take_type(lexer) {
        let token = parser.take_token(lexer).ok_or(Eof)?;

        if parser.choice(lexer, Delim(Lbracket)) {
            if let Some(Num(size_)) = parser.take_num(lexer) {
                parser.expect(lexer, Delim(Rbracket))?;
                type_ = TypeInfo::Array(Box::new(type_),size_);
            } else {
                log::error!("error occured at expr!");
                return Err(ExpectedNumeric(token));
            }
        };

        if let (Id(name), _) = token {
            parser.set_var(name, type_);
            Ok(NdVdecl(parser.offset()))
        } else {
            log::error!("error occured at expr!");
            Err(UnexpectedToken(token))
        }
    } else {
        assign(parser, lexer)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use anyhow::Result;

    #[allow(unused_macros)]
    macro_rules! node {
        ($f:ident,$lhs:expr,$rhs:expr) => {
            $f(Box::new($lhs), Box::new($rhs))
        };
        ($f:ident,$lhs:expr) => {
            $f(Box::new($lhs))
        };
    }

    #[test]
    fn test_parse_arithmetic() -> Result<()> {
        let inputs = ["2+1", "2-1", "2*1", "2/1"];

        let answers = [
            node!(NdAdd, NdNum(2), NdNum(1)),
            node!(NdSub, NdNum(2), NdNum(1)),
            node!(NdMul, NdNum(2), NdNum(1)),
            node!(NdDiv, NdNum(2), NdNum(1)),
        ];

        for (input, answer) in inputs.iter().zip(answers.iter()) {
            let mut lexer = Lexer::new(input.as_bytes()).peekable();
            let mut parser = Parser::new();

            let result = expr(&mut parser, &mut lexer)?;
            dbg!(&result);

            assert_eq!(result, *answer);
        }

        Ok(())
    }
}
