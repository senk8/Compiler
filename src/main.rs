pub mod error_handler;
pub mod grammar;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod types;

use std::fs::File;
use std::io::prelude::*;

use anyhow::Result;
use clap::{App, Arg, ArgGroup};

fn main() -> Result<()> {
    //LOG_LEVEL: error > warn > info > debug > trace
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    log::trace!("start");

    let app = App::new("Compiler")
        .version("1.0,0")
        .author("SenK")
        .about("C Complier implementation for Rust")
        .arg(Arg::from_usage("-c --compile <SOURCE> 'source_string'").required(false))
        .arg(Arg::from_usage("<SOURCE_FILE> 'source_file'").required(false))
        .group(ArgGroup::with_name("input").args(&["compile", "SOURCE_FILE"]));

    let matches = app.get_matches();
    let mut buf = String::new();

    log::trace!("args parse phase");

    /* input processing section */

    let input = if let Some(path) = matches.value_of("SOURCE_FILE") {
        log::debug!("input from a file");
        let mut f = File::open(path).expect("file not found");
        f.read_to_string(&mut buf)
            .expect("something went wrong reading the file");

        buf.as_bytes()
    } else if let Some(source) = matches.value_of("compile") {
        log::debug!("input from a command file");
        source.as_bytes()
    } else {
        unimplemented!();
    };

    /* tokenize and parse */

    /*TODO ParseErrorをひとつにするかどうか */

    let mut lexer = lexer::Lexer::new(input).peekable();
    let mut parser = parser::Parser::new();

    log::trace!("start parsing");

    let result = grammar::parse(&mut parser, &mut lexer);

    match result {
        Ok(asts) => {
            interpreter::gen_instruction::gen_inst_x86_64(asts, "out.s")?;
            log::trace!("end");
            Ok(())
        }
        Err(kind) => {
            error_handler::print::print_error(&kind, input);
            log::error!("error occured!");
            Err(kind)?
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    fn type_of<T>(_: T) -> String {
        let a = std::any::type_name::<T>();
        return a.to_string();
    }

    #[test]
    fn test_compiler() -> anyhow::Result<()> {
        use super::lexer::Lexer;
        use super::parser::Parser;
        use std::fs::File;
        use std::io::prelude::*;

        std::env::set_var("RUST_LOG", "trace");
        env_logger::init();

        log::trace!("start");

        let mut buf = String::new();

        log::trace!("args parse phase");

        let input = {
            let mut f = File::open("foo.c").expect("file not found");
            f.read_to_string(&mut buf)
                .expect("something went wrong reading the file");

            buf.as_bytes()
        };

        let mut lexer = Lexer::new(input).peekable();
        let mut parser = Parser::new();

        log::trace!("start parsing");

        match super::grammar::parse(&mut parser, &mut lexer) {
            Ok(asts) => {
                crate::interpreter::gen_instruction::gen_inst_x86_64(asts, "out.s")?;
                log::trace!("end");
                Ok(())
            }
            Err(kind) => {
                crate::error_handler::print::print_error(&kind, input);
                Err(kind)?
            }
        }
    }
}
