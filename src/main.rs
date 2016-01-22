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
use std::env;

#[derive(PartialEq, Eq)]
enum Command {
    Scan,
    Parse,
    Typecheck,
    Codegen,
}

fn compile(cmd: Command) -> Result<(), Error> {
    let mut data = String::new();
    let _ = stdin().read_to_string(&mut data);
    let mut scanner = Scanner::new(&data);

    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let tok = try!(scanner.next_token());
        let is_eof = tok.typ == TokenType::Eof;
        tokens.push(tok);
        if is_eof {
            break;
        }
    }

    if cmd == Command::Scan {
        for tok in tokens {
            println!("{:?}", tok);
        }
        return Ok(());
    }

    let mut parser = Parser::new(tokens);
    let program = try!(parser.parse_program());

    if cmd == Command::Parse {
        println!("{:#?}", program);
        return Ok(());
    }

    let mut typechecker = TypeChecker::new();
    try!(typechecker.tc_program(&program));

    if cmd == Command::Typecheck {
        println!("SYMBOL TABLE");
        println!("============");
        for (id, ty) in &typechecker.symtable {
            println!("{}: {:?}", id, ty);
        }

        println!("");

        println!("EXPRESSION TABLE");
        println!("================");
        for (expr, ty) in &typechecker.expr_table {
            println!("{:?}: {:?}", expr, ty);
        }
        return Ok(())
    }

    cgen::codegen(&program, &typechecker.symtable, &typechecker.expr_table);

    Ok(())
}

fn usage() {
    match env::current_exe() {
        Ok(path) => { println!("Usage: {} <scan | parse | typecheck | codegen>", path.display()); }
        Err(_) => { println!("Usage: minic <scan | parse | typecheck | codegen>"); }
    }
}

fn main() {
    match env::args().nth(1) {
        Some(cmd) => {
            let res =
                if cmd == "scan" {
                    compile(Command::Scan)
                } else if cmd == "parse" {
                    compile(Command::Parse)
                } else if cmd == "typecheck" {
                    compile(Command::Typecheck)
                } else if cmd == "codegen" {
                    compile(Command::Codegen)
                } else {
                    Err(Error::UsageError)
                };
            match res {
                Ok(()) => (),
                Err(Error::UsageError) => usage(),
                Err(e) => { println!("Error: {}", e); }
            }

        }
        None => usage()
    }
}
