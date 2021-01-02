#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash,Copy)]
pub enum TokenizeError {
   InvalidChar,
   UnexpectedChar
}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash,Copy)]
pub enum ParseError {
   UnexpectedToken,
   UnexpectedKeyword,
   UnexpectedDelimitor,
   UnclosedDelimitor,
   ExpectedNumeric,
   ExpectedIdentifier,
   LackSemicolon,
   Eof,
}

