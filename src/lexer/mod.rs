pub mod lexer_impl;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
pub struct Lexer<'a> {
    /* it is only used by error_at */
    txt: &'a [u8],

    /* Cursor */
    pos: usize,
}

