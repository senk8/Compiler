use super::Parser;

use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::types::tokenize::TokenKind::*;
use crate::types::tokenize::TypeKind;
use crate::types::tokenize::*;

use crate::error_handler::parse_error::ParseError;
use crate::error_handler::parse_error::ParseError::*;

use crate::types::parse::LVar;
use crate::types::parse::TypeInfo;

use core::iter::Peekable;

impl Parser {
    pub fn new() -> Parser {
        Parser {
            symbol_table: HashMap::new(),
            offset: 0,
        }
    }
    pub fn set_var(&mut self, name: String, type_: TypeInfo) -> () {
        self.offset += Self::size_of_type(&type_);
        self.symbol_table
            .insert(name.clone(), LVar(self.offset, type_));
    }

    pub fn find_var(&self, name: String) -> Option<LVar> {
        self.symbol_table.get(&name).cloned()
    }

    pub fn reset_table(&mut self) -> () {
        self.symbol_table.clear();
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn look_ahead(&mut self, lexer: &mut Peekable<Lexer>) -> Option<Token> {
        //TODO : check it out. Whether we implement Deref trait for Token.
        lexer.peek().cloned()
    }

    pub fn expect(
        &mut self,
        lexer: &mut Peekable<Lexer>,
        kind: TokenKind,
    ) -> Result<(), ParseError> {
        self.look_ahead(lexer).ok_or(Eof).and_then(|tk| {
            if tk.0 == kind {
                lexer.next();
                Ok(())
            } else {
                match tk.0 {
                    Type(TypeKind::Int) => Err(UnexpectedToken(tk)),
                    _ => Err(UnexpectedDelimitor(tk)),
                }
            }
        })
    }

    pub fn take_id(&mut self, lexer: &mut Peekable<Lexer>) -> Option<TokenKind> {
        match self.look_ahead(lexer).map(|tk| tk.0) {
            Some(Id(_)) => lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    fn take_type_helper(&mut self, lexer: &mut Peekable<Lexer>) -> Option<TokenKind> {
        match self.look_ahead(lexer).map(|tk| tk.0) {
            Some(Type(_)) => lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    //typeident = type '*' *
    pub fn take_type(&mut self, lexer: &mut Peekable<Lexer>) -> Option<TypeInfo> {
        if let Some(Type(t)) = self.take_type_helper(lexer) {
            let mut type_info = match t {
                TypeKind::Int => TypeInfo::Int,
            };

            while self.choice(lexer, Opr(OperatorKind::Star)) {
                type_info = TypeInfo::Pointer(Box::new(type_info));
            }

            Some(type_info)
        } else {
            None
        }
    }

    pub fn take_num(&mut self, lexer: &mut Peekable<Lexer>) -> Option<TokenKind> {
        match self.look_ahead(lexer).map(|tk| tk.0) {
            Some(Num(_)) => lexer.next().map(|tk| tk.0),
            _ => None,
        }
    }

    pub fn take_token(&mut self, lexer: &mut Peekable<Lexer>) -> Option<Token> {
        match self.look_ahead(lexer).map(|tk| tk.0) {
            Some(_) => lexer.next(),
            _ => None,
        }
    }

    pub fn choice(&mut self, lexer: &mut Peekable<Lexer>, kind: TokenKind) -> bool {
        match self.look_ahead(lexer).map(|tk| tk.0) {
            Some(k) if k == kind => {
                lexer.next();
                true
            }
            _ => false,
        }
    }
}

impl Parser {
    fn size_of_type(type_info: &TypeInfo) -> usize {
        match type_info {
            TypeInfo::Int => 8,
            TypeInfo::Pointer(_) => 8,
            TypeInfo::Array(type_,size_) => size_ * Self::size_of_type(&*type_),
        }
    }
}