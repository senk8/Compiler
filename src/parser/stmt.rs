use super::*;

use crate::types::node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::TokenKind::*;

use crate::types::error::ParseError;

impl<'a> Parser<'a> {
    // program = stmt *

    // program = decl *
    pub(super) fn program(&mut self) -> Result<Vec<Node>, ParseError> {
        /*
        let mut trees = vec![];

        while let Some(_) = self.look_ahead() {
            trees.push(self.stmt()?);
        }
        */

        let mut trees = vec![];

        while let Some(_) = self.look_ahead() {
            trees.push(self.decl()?);
        }

        Ok(trees)
    }

    // decl = type ident ( ( type ident "," )* ) "{" stmt * "}"
    pub(super) fn decl(&mut self) -> Result<Node, ParseError> {
        /* 引数コンパイルしたら同時にローカル変数の定義を行う。*/

        self.expect(Key(Int))?;

        if let Some(Id(name)) = self.take_id() {
            self.expect(Delim(Lc))?;

            let mut args = vec![];
            if !self.choice(Delim(Rc)) {
                loop {
                    self.expect(Key(Int))?;

                    let var = match self.take_id() {
                        Some(Id(name)) => name,
                        _ => panic!("unexpect!"),
                    };

                    self.set_var(var);
                    args.push(NdLVar(self.offset));
                    if !self.choice(Delim(Comma)) {
                        self.expect(Delim(Rc))?;
                        break;
                    }
                }
            };

            self.expect(Delim(LCurl))?;

            let mut nodes = Vec::new();
            while let Ok(node) = self.stmt() {
                nodes.push(node);
            }

            self.expect(Delim(RCurl))?;

            Ok(NdDecl(name, args, Box::new(NdBlock(nodes))))
        } else {
            Err(UnexpectedToken(self.look_ahead().unwrap().1))
        }
    }

    /// stmt = expr ";"
    /// | "{" stmt* "}""
    /// | "return" expr ";"
    /// | "if" "(" expr ")" stmt ("else" stmt)?
    /// | "while" "(" expr ")" stmt
    /// | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    pub(super) fn stmt(&mut self) -> Result<Node, ParseError> {
        /* choice expr or return */

        if self.choice(Key(Return)) {
            let node = NdReturn(Box::new(self.expr()?));
            self.expect(Delim(Semicolon))?;
            Ok(node)
        } else if self.choice(Delim(LCurl)) {
            let mut nodes = Vec::new();
            while let Ok(node) = self.stmt() {
                nodes.push(node);
            }
            self.expect(Delim(RCurl))?;
            Ok(NdBlock(nodes))
        } else if self.choice(Key(If)) {
            self.expect(Delim(Lc))?;
            let first = self.expr()?;
            self.expect(Delim(Rc))?;
            let second = self.stmt()?;

            if self.choice(Key(Else)) {
                let third = self.stmt()?;
                Ok(NdIfElse(Box::new(first), Box::new(second), Box::new(third)))
            } else {
                Ok(NdIf(Box::new(first), Box::new(second)))
            }
        } else if self.choice(Key(While)) {
            self.expect(Delim(Lc))?;
            let first = self.expr()?;
            self.expect(Delim(Rc))?;
            let second = self.stmt()?;
            Ok(NdWhile(Box::new(first), Box::new(second)))
        } else if self.choice(Key(For)) {
            self.expect(Delim(Lc))?;
            let first = self.expr()?;
            self.expect(Delim(Semicolon))?;
            let second = self.expr()?;
            self.expect(Delim(Semicolon))?;
            let third = self.expr()?;
            self.expect(Delim(Rc))?;
            let fourth = self.stmt()?;
            Ok(NdFor(
                Box::new(first),
                Box::new(second),
                Box::new(third),
                Box::new(fourth),
            ))
        } else {
            let node = self.expr()?;
            self.expect(Delim(Semicolon))?;
            Ok(node)
        }
    }
}
