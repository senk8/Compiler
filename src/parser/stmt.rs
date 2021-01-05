use super::*;

use crate::types::node::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::TokenKind::*;

use crate::types::error::ParseError;
use crate::types::error::ParseErrorKind::*;

impl<'a> Parser<'a> {
    // program = stmt *
    pub(super) fn program(&self) -> Result<Vec<Node>,ParseError> {
        let mut trees = vec![];

        while let Some(_) = self.look_ahead() {
            trees.push(self.stmt()?);
        }

        Ok(trees)
    }

    /* stmt = expr ";" | "return" expr ";" */
    pub(super) fn stmt(&self) -> Result<Node,ParseError> {

        /* choice expr or return */
        let node = match self.look_ahead().map(|tok|tok.val){
            Some(Key(Return)) => {
                self.consume();
                Ok(NdReturn(Box::new(self.expr()?)))
            },
            Some(_) => self.expr(),
            None => self.raise_error(Eof,Pos(0,self.input.len()-1))
        }?;

        /* try consume ";" */
        if let Some(tok) = self.consume() {
            match tok.val{
                Delim(Semicolon) => Ok(node),
                _ => self.raise_error(UnexpectedToken,tok.pos),
            }
        }else{
            Err(self.make_error(LackSemicolon))
        }
    }
}
