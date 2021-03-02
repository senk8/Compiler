use super::annotation::*;
pub type Token = (TokenKind, Pos);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum TokenKind {
    Opr(OperatorKind),
    Key(KeywordKind),
    Delim(DelimitorKind),
    Num(usize),
    Id(String),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum OperatorKind {
    /* arithmetic operator */
    Add,
    Sub,
    Star,
    Div,

    /* rerational operator */
    Eq,
    Neq,
    Geq,
    Leq,
    Lt,
    Gt,

    /* others */
    Amp,
    Assign,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum DelimitorKind {
    /* delimitor */
    Lc,
    Rc,
    Comma,
    Semicolon,
    LCurl,
    RCurl,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum KeywordKind {
    /* statement */
    If,
    Else,
    While,
    For,
    Return,
    Int,
}
