

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum TokenType {
    Token(TokenKind),
    Keyword(KeywordKind),
    Num(usize),
    Ident(String),
}


#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum TokenKind {
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


#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum KeywordKind {
    /* statement */
    If,
    While,

    /* statement */
    Return,
}

pub enum DelimitorKind{
    Lc,
    Rc,
    Semicolon
}

//pub trait Tokenizable{}
//impl Tokenizable for TokenKind{}
//impl Tokenizable for Symbol{}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Default,Hash)]
pub struct Span(usize,usize);

