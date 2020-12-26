#[derive(Debug,PartialEq)]
pub enum TokenKind{
    Num(usize),
    Ident(u8),
    Plus,
    Minus,
    Mul,
    Div,
    Lc,
    Rc,
    Eq,
    Neq,
    Geq,
    Leq,
    Lt,
    Gt,
    Semicolon,
}