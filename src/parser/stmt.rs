use super::*;

use crate::types::node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::TokenKind::*;

use crate::types::error::ParseError;
use crate::types::error::ParseErrorKind::*;

/*
macro_rules! choice {
    ($x:expr,$($p:pat),*) => {
        {
            match x {
                $(
                    $pat =>
                )*
            }
        }
    };
}
*/

impl<'a> Parser<'a> {
    // program = stmt *
    pub(super) fn program(&self) -> Result<Vec<Node>, ParseError> {
        let mut trees = vec![];

        while let Some(_) = self.look_ahead() {
            trees.push(self.stmt()?);
        }

        Ok(trees)
    }

    /// stmt = expr ";"
    /// | "return" expr ";"
    /// | "if" "(" expr ")" stmt ("else" stmt)?
    /// | "while" "(" expr ")" stmt
    /// | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    pub(super) fn stmt(&self) -> Result<Node, ParseError> {
        /* choice expr or return */
        let node = match self.look_ahead().map(|tok| tok.0) {
            Some(Key(Return)) => {
                self.consume();
                Ok(NdReturn(Box::new(self.expr()?)))
            }
            /*
            Some(Key(If)) => {
                self.consume()
                    .expect(Delim(Lc))
                    .expr()
                    .expect(Rc)
                    .stmt()
                    .option()

                /* ? */
                let node = self.expr()?;
                self.look_ahead().ok_or(self.make_error(Eof)).and_then(|tok|match tok.val{
                        Delim(Lc) => Ok(node),
                        _ => self.raise_error(UnclosedDelimitor,tok.pos),
                }
            },*/
            _ => self.expr(),
        }?;

        /* try consume ";" */
        self.expect_tk(Delim(Semicolon))?;

        Ok(node)
    }
}
