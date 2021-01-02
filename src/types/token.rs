

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum TokenType {
    Token(TokenKind),
    Keyword(KeywordKind),
    Num(usize),
    Id(String),
}

/*
pub struct Wrap<T> {
    val:T

}
*/

//type Token = Wrap<TokenType>


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

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Default,Hash)]
pub struct Span(usize,usize);

//pub trait Tokenizable{}
//impl Tokenizable for TokenKind{}
//impl Tokenizable for Symbol{}

/* 
#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum Token {
    Operator(BinOp),
    Delimitor(DelimitorKind),
    Keyword(KeywordKind),
    Num(usize),
    Ident(String),
}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum DelimitorKind{
    Lc,
    Rc,
    Semicolon
}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum KeywordKind {
    /* statement */
    If,
    While,

    /* statement */
    Return,
}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum BinOp {
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
}
*/