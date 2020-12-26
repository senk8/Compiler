#[derive(Debug,PartialEq)]
pub enum TokenType{
    Token(TokenKind),
    Keyword(Symbol),
    Num(usize),
    Ident(char)
}

#[derive(Debug,PartialEq)]
pub enum TokenKind{
    /* arithmetic operator */
    Plus,
    Minus,
    Mul,
    Div,

    /* rerational operator */
    Eq,
    Neq,
    Geq,
    Leq,
    Lt,
    Gt,

    /* others */
    Assign,

    /* delimitor */
    Lc,
    Rc,
    Semicolon,
}

#[derive(Debug,PartialEq)]
pub enum Symbol{
    If,
    While,
}
