use super::annotation::*;

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

