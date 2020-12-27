pub mod expr;
pub mod stmt;
pub mod parse_util;

pub struct Parser<'a>{
    symbol_map : RefCell<HashMap<String,LVar>>,
    tokenizer : RefCell<Peekable<Tokenizer<'a>>>
}

pub struct SymbolMap{
    symbol_map : 
    last :
}