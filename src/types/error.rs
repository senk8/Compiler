use super::annotation::Pos;
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

    #[error("expected an experession, but found other. : {0}\n{1}\n Suggestion : It may be missing some expression. Add some expression here. ")]
    MissingExpressionError(Pos,String),

    #[error("expected \";\" , but found other. : {0}\n{1}\n Suggestion : It may be missing \";\".  Add \";\" here. ")]
    MissingSemicolonError(Pos,String),

    #[error("Parsing process reached EOF. Your input may lack a delimitor. :{0}\n{1}")]
    EofError(Pos,String),

    #[error("Segmentation Fault :{0}\n{1}")]
    SegmentationFault(Pos,String),
}

