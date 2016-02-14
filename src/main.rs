extern crate clap;
use clap::{App, Arg, SubCommand};

mod error;
mod pos;
mod token;
mod scanner;
mod types;
mod ast;
mod parser;
mod typecheck;
mod cgen;

use token::{Token, TokenType};
use error::Error;
use scanner::Scanner;
use parser::Parser;
use typecheck::TypeChecker;

use std::io::{Read, stdin};
use std::process;


fn scan(display_tokens: bool) {
    let mut stdin = stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf);
    let mut scanner = Scanner::new(&buf);

    loop {
        match scanner.next_token() {
            Err(err) => {
                println!("{:?}", err);
                process::exit(1)
            }
            Ok(Token { typ: TokenType::Eof, .. }) => {}
            Ok(tok) => {
                if display_tokens {
                    println!("{:?}", tok);
                }
            }
        }
    }
}

fn parse(display_ast: bool) {
    let mut stdin = stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf);
    let mut scanner = Scanner::new(&buf);

    let mut tokens = Vec::new();
    loop {
        match scanner.next_token() {
            Ok(tok) => {
                let is_eof = tok.typ == TokenType::Eof;
                tokens.push(tok);
                if is_eof {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                process::exit(1);
            }
        }
    }

    let mut parser = Parser::new(tokens);

    match parser.parse_program() {
        Ok(ast) => {
            if display_ast {
                println!("{:#?}", ast);
            }
        }
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    }
}

fn main() {
    let compiler_match = App::new("Minilang compiler")
        .version("0.1")
        .author("Vincent Foley <vfoley@gmail.com>")
        .subcommand(SubCommand::with_name("scan")
                    .about("Scan a program; return 0 if valid, 1 otherwise"))

        .subcommand(SubCommand::with_name("tokens")
                    .about("Scan a program and print its tokens one per line"))

        .subcommand(SubCommand::with_name("parse")
                    .about("Parse a program; return 0 if valid, 1 otherwise"))

        .subcommand(SubCommand::with_name("ast")
                    .about("Parse a program and print its AST"))

        .subcommand(SubCommand::with_name("typecheck")
                    .about("Typecheck a program; return 0 if valid, 1 otherwise"))

        .subcommand(SubCommand::with_name("typed-ast")
                    .about("Typecheck a program and print its typed AST"))

        .subcommand(SubCommand::with_name("mips")
                    .about("Generate MIPS code for a program"))



        .get_matches();

    match compiler_match.subcommand_name() {
        Some("scan") => { scan(false) }
        Some("tokens") => { scan(true) }
        Some("parse") => { parse(false) }
        Some("ast") => { parse(true) }
        Some(_) => {}
        None => {
            println!("{}", compiler_match.usage());
        }
    }
}
