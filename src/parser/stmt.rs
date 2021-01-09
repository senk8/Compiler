use super::*;

use crate::types::node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::TokenKind::*;

use crate::types::error::ParseError;

macro_rules! raise {
    ($error:ident,$pos:expr,$input:expr) => {{
        let pos = Pos($pos - 1, $pos);
        let mut message = std::str::from_utf8($input)
            .map(|s| String::from(s))
            .unwrap();
        message.push_str(&format!("\n{:>width$}\n", "^", width = pos.0 + 1));
        $error(pos, message)
    }};
}

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
        match self.look_ahead().map(|tok| tok.0) {
            Some(Key(Return)) => {
                self.consume();
                let node = Ok(NdReturn(Box::new(self.expr()?)));
                self.expect_tk(Delim(Semicolon))?;
                node
            }
            Some(Key(If)) => {
                self.consume();
                self.expect_tk(Delim(Lc))?;
                let first = self.expr()?;
                self.expect_tk(Delim(Rc))?;
                let second = self.stmt()?;

                /* ? */
                match self.look_ahead().map(|tok|tok.0){
                    Some(Key(Else)) => {
                        self.consume();
                        let third = self.stmt()?;
                        Ok(NdIfElse(Box::new(first),Box::new(second),Box::new(third)))
                    },
                    _ => Ok(NdIf(Box::new(first),Box::new(second))),
                }
            },
            Some(Key(While)) => {
                self.consume();
                self.expect_tk(Delim(Lc))?;
                let first = self.expr()?;
                self.expect_tk(Delim(Rc))?;
                let second = self.stmt()?;
                Ok(NdWhile(Box::new(first),Box::new(second)))
            },
            Some(Key(For)) => {
                self.consume();
                self.expect_tk(Delim(Lc))?;
                let first = self.expr()?;
                self.expect_tk(Delim(Semicolon))?;
                let second = self.expr()?;
                self.expect_tk(Delim(Semicolon))?;
                let third = self.expr()?;
                self.expect_tk(Delim(Rc))?;
                let fourth = self.stmt()?;
                Ok(NdFor(Box::new(first),Box::new(second),Box::new(third),Box::new(fourth)))
            },
            _ => {
                let node = self.expr();
                self.expect_tk(Delim(Semicolon))?;
                node
            }
        }

        /* try consume ";" */

    }
}
