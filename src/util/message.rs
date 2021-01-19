use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

pub fn show_message(error:&ParseError,input:&[u8])->(){
    match error {
        UnexpectedToken(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = pos.1 + 1);
        },
        UnexpectedKeyword(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        UnexpectedDelimitor(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        UnclosedDelimitor(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        ExpectedNumeric(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("Unexpected! :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
        },
        MissingExpression(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("expected an experession, but found other. : {}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("Suggestion : It may be missing some expression. Add some expression here. ");
        },
        MissingSemicolon(pos) => {
            let code = &input[pos.0..pos.1];
            eprintln!("expected \";\" , but found other. :{}",pos);
            eprintln!("{}\n", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("Suggestion : It may be missing \";\".  Add \";\" here.");
        },
        Eof(pos) => {
            let code = input.last().unwrap();
            eprintln!("Parsing process reached EOF. Your input may lack a delimitor. :{}",pos);
            eprintln!("{}\n", *code as char);
            eprintln!("{}", "Suggestion: ");
        },
        SegmentationFault(pos) => {
            let code = input.last().unwrap();
            eprintln!("Segmentation Fault:{}",pos);
            eprintln!("{}\n", *code as char);
            eprintln!("{}", "Suggestion: ");
        }
    }
}