#[derive(Debug,PartialEq)]

pub enum TokenKind{
    Num(usize),
    Ident(char),
    Plus,
    Minus,
    Mul,
    Div,
    Lc,
    Rc,
    Assign,
    Eq,
    Neq,
    Geq,
    Leq,
    Lt,
    Gt,
    Semicolon,
}

/*
pub enum Token{
    Keyword(TokenKind),
    Keyword(TokenKind),
    Operator(TokenKind),
}
*/