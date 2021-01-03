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

        while let Ok(_) = self.look_ahead() {
            trees.push(self.stmt()?);
        }

        Ok(trees)
    }

    /* stmt = expr ";" | "return" expr ";" */
    pub(super) fn stmt(&self) -> Result<Node,ParseError> {

        let node = self.look_ahead().and_then(|tok|
            match tok.val{
                Key(Return) => {
                    self.next_token()?;
                    Ok(NdReturn(Box::new(self.expr()?)))
                },
                _ => self.expr()
            }
        )?;

        let next = self.look_ahead()?;

        match next.val {
            Delim(Semicolon) => {
                self.next_token()?;
                Ok(node)
            },
            _ => self.raise_error(LackSemicolon,next.pos),
        }
    }
}
