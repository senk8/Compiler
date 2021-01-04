use super::annotation::*;
use std::fmt;
use thiserror::Error;

/*
pub type ParseError = Annot<ParseErrorKind>;

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum TokenizeError {
   InvalidChar,
   UnexpectedChar
}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum ParseErrorKind {
   UnexpectedToken,
   UnexpectedKeyword,
   UnexpectedDelimitor,
   UnclosedDelimitor,
   ExpectedNumeric,
   ExpectedIdentifier,
   LackSemicolon,
   Eof,
}
*/

impl fmt::Display for Pos{
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"line:{} column:{}",self.0,self.1)
    }
}

#[derive(Error,Debug)]
pub enum ParseError{
    #[error("Unexpected! :{0} {1}")]
    UnexpectedToken(Pos,String),

    #[error("Unexpected! :{0} {1}")]
    UnexpectedKeyword(Pos,String),

    #[error("Unexpected! :{0} {1}")]
    UnexpectedDelimitor(Pos,String),

    #[error("Unexpected! :{0} {1}")]
    UnclosedDelimitor(Pos,String),

    #[error("Unexpected! :{0} {1}")]
    ExpectedNumeric(Pos,String),

    #[error("Unexpected! :{0} {1}")]
    ExpectedIdentifier(Pos,String),

    #[error("Lack Some Semicolon !. Your input lack a delimitor. : {0}\n{1}")]
    LackSemicolon(Pos,String),

    #[error("Parsing process reached EOF. Your input may lack a delimitor. :{0}\n{1}")]
    Eof(Pos,String),
}

