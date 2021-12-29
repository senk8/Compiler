use super::parse_error::ParseError;
use super::parse_error::ParseError::*;

/* TODO エラーメッセージの表示を文ごとに行う */
pub fn print_error(error: &ParseError, input: &[u8]) {
    match error {
        UnexpectedToken(tk) => {
            let code = &input[tk.1 .0..tk.1 .1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
        }
        UnexpectedKeyword(tk) => {
            let code = &input[tk.1 .0..tk.1 .1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
        }
        UnexpectedDelimitor(tk) => {
            let code = &input[tk.1 .0..tk.1 .1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
        }
        UnclosedDelimitor(tk) => {
            let code = &input[tk.1 .0..tk.1 .1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
        }
        ExpectedNumeric(tk) => {
            let code = &input[tk.1 .0..tk.1 .1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
        }
        MissingExpression(tk) => {
            let code = &input[tk.1 .1..];
            eprintln!("expected an experession, but found other. : {}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
            eprintln!("Suggestion : It may be missing some expression. Add some expression here. ");
        }
        MissingSemicolon(tk) => {
            let code = &input[tk.1 .1..];
            eprintln!("expected \";\" , but found other. :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
            eprintln!("Suggestion : It may be missing \";\".  Add \";\" here.");
        }
        MissingDelimitor(tk) => {
            let code = &input[tk.1 .1..];
            eprintln!("expected \";\" , but found other. :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
            eprintln!("Suggestion : It may be missing \"(\".  Add \";\" here.");
        }
        UndefinedSymbol(tk) => {
            let code = input.last().unwrap();
            eprintln!("This variable is Undefined:{}", tk.1);
            eprintln!("{}", *code as char);
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
            eprintln!("Suggestion: ");
        }
        SegmentationFault(tk) => {
            let code = input.last().unwrap();
            eprintln!("Segmentation Fault:{}", tk.1);
            eprintln!("{}", *code as char);
            eprintln!("{:>width$}", "^", width = tk.1 .1 + 1);
            eprintln!("Suggestion: ");
        }
        Eof => {
            eprintln!(
                "Parsing process reached EOF. Your input may lack a delimitor. :{}",
                input.len()
            );
            eprintln!("{}", String::from_utf8(input.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = input.len() + 1);
            eprintln!("Suggestion: ");
        }
    }
}
