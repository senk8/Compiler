use std::fmt;

pub type Token = (TokenKind, Pos);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum TokenKind {
    Opr(OperatorKind),
    Key(KeywordKind),
    Type(TypeKind),
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

    Sizeof,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum DelimitorKind {
    /* delimitor */
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Semicolon,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum KeywordKind {
    /* statement */
    If,
    Else,
    While,
    For,
    Return,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum TypeKind {
    Int,
    Pointer,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct Pos(pub usize, pub usize);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line:{} Column:{}", self.0 + 1, self.1 + 1)
    }
}