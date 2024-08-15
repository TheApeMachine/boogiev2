use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod logging;

use lexer::{Lexer};
use parser::{Parser};
use logging::{log_info, log_warning, log_error};

fn main() {
    // Get the command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Ensure that a filename was provided.
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    // Read the source code from the file.
    let filename = &args[1];
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
            process::exit(1);
        }
    };

    // Lexing: Convert the source code into tokens.
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    // Parsing: Convert tokens into an abstract syntax tree (AST).
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Output the parsed statements.
    for statement in ast {
        println!("{:?}", statement);
    }
}

