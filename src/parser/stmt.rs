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
        let mut trees = vec![];

        while let Some(_) = self.look_ahead() {
            trees.push(self.stmt()?);
        }

        Ok(trees)
    }

    /*
    // decl = ident ( (ident "," )* ) "{" stmt * "}"
    pub(super) fn decl(&mut self) -> Result<Vec<Node>, ParseError> {

        引数コンパイルしたら同時にローカル変数の定義を行う。

        match self.look_ahead().map(|tk|tk.0) {
            Some(Id(name)) =>  name,
        };

        self.expect_tk(Delim(Lc))?;

        let mut args = vec![];

        if let Some((Delim(Rc),_)) = self.look_ahead() {
            self.consume();
        }else{
            let ident = self.expect_ident()?;
            self.set_var(ident);

            args.push(NdLVar(self.offset.get()))

            while let Some((Delim(Comma),_))= self.look_ahead(){
                self.consume();

                let ident = self.expect_ident()?;
                self.set_var(ident);
                args.push(NdLVar(self.offset.get()));
            }

            self.expect_tk(Delim(Rc))?;
        }

        match self.look_ahead().map(|tok| tok.0) {
            // Parse "{" stmt* "}""
            Some(Delim(LCurl)) => {
                self.consume();
                let mut nodes = Vec::new();
                while let Ok(node) = self.stmt() {
                    nodes.push(node);
                }
                self.expect_tk(Delim(RCurl))?;
                Ok(NdBlock(nodes))
            }
            _ => Err()
        }
    }
    */
    
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

        /*
        match self.look_ahead().map(|tok| tok.0) {
            Some(Key(Else)) => {
                self.consume();
                let third = self.stmt()?;
                Ok(NdIfElse(Box::new(first), Box::new(second), Box::new(third)))
            }
            _ => Ok(NdIf(Box::new(first), Box::new(second))),
        }
        */
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

        /*
        match self.look_ahead().map(|tok| tok.0) {
            Some(Key(Return)) => {
                self.consume();
                let node = Ok(NdReturn(Box::new(self.expr()?)));
                self.expect(Delim(Semicolon))?;
                node
            }
            // Parse "{" stmt* "}""
            Some(Delim(LCurl)) => {
                self.consume();
                let mut nodes = Vec::new();
                while let Ok(node) = self.stmt() {
                    nodes.push(node);
                }
                self.expect(Delim(RCurl))?;
                Ok(NdBlock(nodes))
            }
            Some(Key(If)) => {
                self.consume();
                self.expect(Delim(Lc))?;
                let first = self.expr()?;
                self.expect(Delim(Rc))?;
                let second = self.stmt()?;

                /* ? */
                match self.look_ahead().map(|tok| tok.0) {
                    Some(Key(Else)) => {
                        self.consume();
                        let third = self.stmt()?;
                        Ok(NdIfElse(Box::new(first), Box::new(second), Box::new(third)))
                    }
                    _ => Ok(NdIf(Box::new(first), Box::new(second))),
                }
            }
            Some(Key(While)) => {
                self.consume();
                self.expect(Delim(Lc))?;
                let first = self.expr()?;
                self.expect(Delim(Rc))?;
                let second = self.stmt()?;
                Ok(NdWhile(Box::new(first), Box::new(second)))
            }
            Some(Key(For)) => {
                self.consume();
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
            }
            _ => {
                let node = self.expr()?;
                self.expect(Delim(Semicolon))?;
                Ok(node)
            }
        }
        */
        
    }
}
