use super::annotation::Pos;
use std::fmt;
use thiserror::Error;

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash)]
pub enum ParseErrorKind {
   UnexpectedToken,
   UnexpectedKeyword,
   UnexpectedDelimitor,
   UnclosedDelimitor,
   ExpectedNumeric,
   ExpectedIdentifier,
   LackSemicolon,
   LackExpr,
   Eof,
}


impl fmt::Display for Pos{
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"line:{} column:{}",self.0,self.1)
    }
}

//#[derive(Error,Debug)]
#[derive(Error,Debug)]
pub enum ParseError{
    #[error("Unexpected! :{0}\n{1}")]
    UnexpectedTokenError(Pos,String),

    #[error("Unexpected! :{0}\n{1}")]
    UnexpectedKeywordError(Pos,String),

    #[error("Unexpected! :{0}\n{1}")]
    UnexpectedDelimitorError(Pos,String),

    #[error("Unexpected! :{0}\n{1}")]
    UnclosedDelimitorError(Pos,String),

    #[error("Unexpected! :{0}\n{1}")]
    ExpectedNumericError(Pos,String),

    #[error("Unexpected! :{0}\n{1}")]
    LackExprError(Pos,String),

    #[error("Lack Some Semicolon !. Your input lack a delimitor. : {0}\n{1}")]
    LackSemicolonError(Pos,String),

    #[error("Parsing process reached EOF. Your input may lack a delimitor. :{0}\n{1}")]
    EofError(Pos,String),

    #[error("Segmentation Fault :{0}\n{1}")]
    SegmentationFault(Pos,String),
}

