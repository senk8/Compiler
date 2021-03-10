pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod types;


use std::fs::File;
use std::io::prelude::*;

use clap::{App, Arg, ArgGroup};

use lexer::Lexer;
use parser::Parser;
use interpreter::gen_instruction::gen_inst_x86_64;

use types::error::ParseError;
use types::error::ParseError::*;

use anyhow::Result;
use anyhow::Context;

/* 懸念点

    1. Blockとstmtが同じ
    2. rspを16の倍数にしていない(スタックに退避する方法でなんとかなる)

*/

fn main() -> Result<()> {
    let app = App::new("Compiler")
        .version("1.0,0")
        .author("SenK")
        .about("C Complier implementation for Rust")
        .arg(Arg::from_usage("-c --compile <SOURCE> 'source_string'").required(false))
        .arg(Arg::from_usage("<SOURCE_FILE> 'source_file'").required(false))
        .group(ArgGroup::with_name("input").args(&["compile", "SOURCE_FILE"]));

    let matches = app.get_matches();
    let mut buf = String::new();

    /* input processing section */

    let input = if let Some(path) = matches.value_of("SOURCE_FILE") {
        let mut f = File::open(path).expect("file not found");
        f.read_to_string(&mut buf)
            .expect("something went wrong reading the file");

        buf.as_bytes()
    } else if let Some(source) = matches.value_of("compile") {
        source.as_bytes()
    } else {
        unimplemented!();
    };

    /* tokenize and parse */

    /*TODO ParseErrorをひとつにするかどうか */

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(asts) => {
            gen_inst_x86_64(asts, "out.s")?;
            Ok(())
        }
        Err(kind) => {
            print_error(&kind, input);
            Err(kind)?
        }
    }
}


/* TODO エラーメッセージの表示を文ごとに行う */
fn print_error(error: &ParseError, input: &[u8]) -> () {
    match error {
        UnexpectedToken(tk) => {
            let code = &input[tk.1.0..tk.1.1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
        }
        UnexpectedKeyword(tk) => {
            let code = &input[tk.1.0..tk.1.1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
        }
        UnexpectedDelimitor(tk) => {
            let code = &input[tk.1.0..tk.1.1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
        }
        UnclosedDelimitor(tk) => {
            let code = &input[tk.1.0..tk.1.1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
        }
        ExpectedNumeric(tk) => {
            let code = &input[tk.1.0..tk.1.1];
            eprintln!("Unexpected! :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
        }
        MissingExpression(tk) => {
            let code = &input[tk.1.1..];
            eprintln!("expected an experession, but found other. : {}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
            eprintln!("Suggestion : It may be missing some expression. Add some expression here. ");
        }
        MissingSemicolon(tk) => {
            let code = &input[tk.1.1..];
            eprintln!("expected \";\" , but found other. :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
            eprintln!("Suggestion : It may be missing \";\".  Add \";\" here.");
        }
        MissingDelimitor(tk) => {
            let code = &input[tk.1.1..];
            eprintln!("expected \";\" , but found other. :{}", tk.1);
            eprintln!("{}", String::from_utf8(code.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
            eprintln!("Suggestion : It may be missing \"(\".  Add \";\" here.");
        }
        UndefinedSymbol(tk) => {
            let code = input.last().unwrap();
            eprintln!("This variable is Undefined:{}", tk.1);
            eprintln!("{}", *code as char);
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
            eprintln!("{}", "Suggestion: ");
        }
        SegmentationFault(tk) => {
            let code = input.last().unwrap();
            eprintln!("Segmentation Fault:{}", tk.1);
            eprintln!("{}", *code as char);
            eprintln!("{:>width$}", "^", width = tk.1.1 + 1);
            eprintln!("{}", "Suggestion: ");
        }
        Eof => {
            eprintln!(
                "Parsing process reached EOF. Your input may lack a delimitor. :{}",
                input.len()
            );
            eprintln!("{}", String::from_utf8(input.to_vec()).unwrap());
            eprintln!("{:>width$}", "^", width = input.len() + 1);
            eprintln!("{}", "Suggestion: ");
        }
    }
}

fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}


mod tests {
    #[cfg(test)]
    fn type_of<T>(_: T) -> String {
        let a = std::any::type_name::<T>();
        return a.to_string();
    }
}
