mod error;
mod pos;
mod token;
mod scanner;
mod types;
mod parser;
mod typecheck;

use token::{Token, TokenType};
use error::Error;
use scanner::Scanner;
use parser::Parser;
use typecheck::TypeChecker;
use std::io::{Read, stdin};

fn compile() -> Result<(), Error> {
    let mut data: String = String::new();
    let _ = stdin().read_to_string(&mut data);
    let mut scanner = Scanner::new(data);

    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let tok = try!(scanner.next_token());
        let is_eof = tok.typ == TokenType::Eof;
        tokens.push(tok);
        if is_eof {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let program = try!(parser.parse_program());

    let mut typechecker = TypeChecker::new();
    try!(typechecker.tc_program(&program));

    Ok(())
}

fn main() {
    match compile() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e)
    }
}
