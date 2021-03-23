pub mod parser_impl;
use crate::types::variable::LVar;
use std::collections::HashMap;

pub struct Parser {
    /* mutable field for symbol table */
    symbol_table: HashMap<String, LVar>,
    offset: usize,
}

