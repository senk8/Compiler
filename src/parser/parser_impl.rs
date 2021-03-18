use super::Parser;

use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::types::node::Node;
use crate::types::token::TokenKind::*;
use crate::types::token::TypeKind::*;
use crate::types::token::*;

use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

use crate::types::variable::LVar;
use crate::types::variable::VarAnnot;



impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let ll_1_lexer = lexer.peekable();

        Parser {
            lexer: ll_1_lexer,
            symbol_table: HashMap::new(),
            offset: 0,
        }
    }
    pub fn set_var(&mut self, name: String, ty: VarAnnot) -> () {
        self.offset += 8;
        self.symbol_table
            .insert(name.clone(), LVar(self.offset, ty));
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ParseError> {
        self.program()
    }

    pub fn look_ahead(&mut self) -> Option<Token> {
        //TODO : check it out. Whether we implement Deref trait for Token.
        self.lexer.peek().cloned()
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        self.look_ahead()
            .ok_or(Eof)
            .and_then(|tk| {
                if tk.0 == kind {
                    self.lexer.next();
                    Ok(())
                } else {
                    match tk.0 {
                        Type(Int) => Err(UnexpectedToken(tk)),
                        _ => Err(UnexpectedDelimitor(tk)),
                    }
                }
            })
    }

    pub(super) fn take_id(&mut self) -> Option<TokenKind> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(Id(_)) => self.lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    fn take_type_helper(&mut self) -> Option<TokenKind> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(Type(_)) => self.lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    //typeident = type '*' *
    pub(super) fn take_type(&mut self)->Option<VarAnnot> {
        if let Some(Type(t)) = self.take_type_helper(){

            let mut ty = VarAnnot { ty: t, ptr: None };

            while self.choice(Opr(OperatorKind::Star)) {
                ty = VarAnnot {
                    ty: Pointer,
                    ptr: Some(Box::new(ty)),
                };
            }

            Some(ty)
        }else{
            None
        }
    }

    pub(super) fn take_num(&mut self) -> Option<TokenKind> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(Num(_)) => self.lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    pub(super) fn take_token(&mut self) -> Option<Token> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(_) => self.lexer.next(),
            _ => None,
        }
    }

    pub(super) fn choice(&mut self, kind: TokenKind) -> bool {
        match self.look_ahead().map(|tk| tk.0) {
            Some(k) if k == kind => {
                self.lexer.next();
                true
            }
            _ => false,
        }
    }

}

