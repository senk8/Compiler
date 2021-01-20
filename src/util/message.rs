use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

/* TODO エラーメッセージの表示を文ごとに行う */
pub fn show_message(error:&ParseError,input:&[u8])->(){
    match error {
        UnexpectedToken(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
        },
        UnexpectedKeyword(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
        },
        UnexpectedDelimitor(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
        },
        UnclosedDelimitor(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
        },
        ExpectedNumeric(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
        },
        MissingExpression(pos) => {
            let code = &input[pos.1..];
            eprintln!("expected an experession, but found other. : {}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
            eprintln!("Suggestion : It may be missing some expression. Add some expression here. ");
        },
        MissingSemicolon(pos) => {
            let code = &input[pos.1..];
            eprintln!("expected \";\" , but found other. :{}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
            eprintln!("Suggestion : It may be missing \";\".  Add \";\" here.");
        },
        MissingDelimitor(pos) => {
            let code = &input[pos.1..];
            eprintln!("expected \";\" , but found other. :{}",pos);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
            eprintln!("Suggestion : It may be missing \"(\".  Add \";\" here.");
        },
        Eof(pos) => {
            eprintln!("Parsing process reached EOF. Your input may lack a delimitor. :{}",pos);
            eprintln!("{}", String::from_utf8(input.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = input.len() + 1);
            eprintln!("{}", "Suggestion: ");
        },
        SegmentationFault(pos) => {
            let code = input.last().unwrap();
            eprintln!("Segmentation Fault:{}",pos);
            eprintln!("{}", *code as char);
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
            eprintln!("{}", "Suggestion: ");
        }
    }
}