pub mod lexer;
pub mod parser;
pub mod semantic_analyzer;
pub mod types;
pub mod util;

use lexer::*;
use parser::*;
use semantic_analyzer::assemble::gen_inst_x86_64;
use types::error::ParseError;
use util::message::show_message;

use std::fs::File;
use std::io::prelude::*;

use clap::{App, Arg, ArgGroup};

//use anyhow::{bail, ensure, Context, Result};

fn main() -> Result<(), ParseError> {
    let app = App::new("Compiler")
        .version("1.0,0")
        .author("SenK")
        .about("C Complier implementation for Rust")
        .arg(Arg::from_usage("-c --compile <SOURCE> 'source_string'").required(false))
        .arg(Arg::from_usage("<FILE> 'source_file'").required(false))
        .group(ArgGroup::with_name("input").args(&["compile", "FILE"]));

    let matches = app.get_matches();
    let mut buf = String::new();

    /* input processing section */

    let input = if let Some(path) = &matches.value_of("FILE") {
        let mut f = File::open(path).expect("file not found");
        f.read_to_string(&mut buf)
            .expect("something went wrong reading the file");

        buf.as_bytes()
    } else if let Some(source) = &matches.value_of("compile") {
        source.as_bytes()
    } else {
        unimplemented!();
    };

    /* tokenize and parse */

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let asts = parser.parse().map_err(|error| {
        show_message(&error, input);
        error
    })?;

    gen_inst_x86_64(asts);

    return Ok(());
}

mod tests {
    #[cfg(test)]
    fn type_of<T>(_: T) -> String {
        let a = std::any::type_name::<T>();
        return a.to_string();
    }
}
