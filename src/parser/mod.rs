pub mod grammar;
pub mod parser_impl;

use crate::lexer::Lexer;
use crate::types::variable::LVar;

use core::iter::Peekable;
use std::collections::HashMap;

pub struct Parser<'a> {
    /* mutable field for symbol table */
    symbol_table: HashMap<String, LVar>,
    offset: usize,

    /* mutable field for tokenizer */
    lexer: Peekable<Lexer<'a>>,
}