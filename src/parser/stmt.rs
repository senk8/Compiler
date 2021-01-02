use super::*;
use crate::types::node::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::TokenKind::*;
use crate::types::error::ParseError;

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

        self.look_ahead().and_then(|tok|
            match tok{
                Keyword(Return) => {
                    self.next_token()?;
                    Ok(NdReturn(Box::new(self.expr()?)))
                },
                _ => self.expr()
            }
        ).and_then(|node|
            match self.look_ahead()? {
                Token(Semicolon) => {
                    self.next_token()?;
                    Ok(node)
                },
                _ => Err(LackSemicolon),
            }
        )
    }
}
