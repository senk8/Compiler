use super::token::Token;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedKeyword(Token),
    UnexpectedDelimitor(Token),
    UnclosedDelimitor(Token),
    ExpectedNumeric(Token),
    MissingExpression(Token),
    MissingSemicolon(Token),
    MissingDelimitor(Token),
    UndefinedSymbol(Token),
    SegmentationFault(Token),
    Eof,
}

/*
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected! :{0}\n{1}")]
    UnexpectedToken(Pos, String),

    #[error("Unexpected! :{0}\n{1}")]
    UnexpectedKeyword(Pos, String),

    #[error("Unexpected! :{0}\n{1}")]
    UnexpectedDelimitor(Pos, String),

    #[error("Unexpected! :{0}\n{1}")]
    UnclosedDelimitor(Pos, String),

    #[error("Unexpected! :{0}\n{1}")]
    ExpectedNumeric(Pos, String),

    #[error("expected an experession, but found other. : {0}\n{1}\n Suggestion : It may be missing some expression. Add some expression here. ")]
    MissingExpression(Pos, String),

    #[error("expected \";\" , but found other. : {0}\n{1}\n Suggestion : It may be missing \";\".  Add \";\" here. ")]
    MissingSemicolon(Pos, String),

    #[error("Parsing process reached EOF. Your input may lack a delimitor. :{0}\n{1}")]
    Eof(Pos, String),

    #[error("Segmentation Fault :{0}\n{1}")]
    SegmentationFault(Pos, String),
}
*/
