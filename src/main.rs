extern crate clap;
use clap::{App, SubCommand};

mod error;
mod pos;
mod token;
mod scanner;
mod types;
mod ast;
mod parser;
mod typecheck;
// mod cgen;

use token::{Token, TokenType};
use error::Error;
use scanner::Scanner;
use parser::Parser;
use typecheck::TypeChecker;

use std::io::{Read, stdin};
use std::process;


struct CompileManager;

enum CompileAction {
    Scan,
    DisplayTokens,
    Parse,
    DisplayAst,
    Typecheck,
    TypeTables,
}

impl CompileManager {
    fn error(&self, err: Error) -> ! {
        println!("{}", err);
        process::exit(1);
    }

    fn perform_action(&self, action: CompileAction) {
        match action {
            CompileAction::Scan => { self.scan(false).unwrap_or_else(|e| self.error(e)) }
            CompileAction::DisplayTokens => { self.scan(true).unwrap_or_else(|e| self.error(e)) }
            CompileAction::Parse => { self.parse(false).unwrap_or_else(|e| self.error(e)) }
            CompileAction::DisplayAst => { self.parse(true).unwrap_or_else(|e| self.error(e)) }
            CompileAction::Typecheck => { self.typecheck(false).unwrap_or_else(|e| self.error(e)) }
            CompileAction::TypeTables => { self.typecheck(true).unwrap_or_else(|e| self.error(e)) }
        }
    }

    fn get_tokens(&self) -> Result<Vec<Token>, Error> {
        let mut stdin = stdin();
        let mut buf = String::new();
        let _ = stdin.read_to_string(&mut buf);
        let mut scanner = Scanner::new(&buf);

        let mut tokens = Vec::new();
        loop {
            let tok = try!(scanner.next_token());
            let is_eof = tok.typ == TokenType::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    // TODO(vfoley): don't build token vector if `display_tokens == false`.
    fn scan(&self, display_tokens: bool) -> Result<(), Error> {
        let tokens = try!(self.get_tokens());
        if display_tokens {
            for tok in tokens.iter() {
                println!("{:?}", tok);
            }
        }
        Ok(())
    }


    fn parse(&self, display_ast: bool) -> Result<(), Error> {
        let tokens = try!(self.get_tokens());
        let mut parser = Parser::new(tokens);
        let ast = try!(parser.parse_program());
        if display_ast {
            println!("{:#?}", ast);
        }
        Ok(())
    }

    fn typecheck(&self, display_tables: bool) -> Result<(), Error> {
        let tokens = try!(self.get_tokens());
        let mut parser = Parser::new(tokens);
        let ast = try!(parser.parse_program());
        let mut tc = TypeChecker::new();
        try!(tc.tc_program(&ast));
        if display_tables {
            println!("SYMBOL TABLE");
            println!("{:#?}", tc.symtable);
            println!("EXPRESSION TABLE");
            println!("{:#?}", tc.expr_table);
        }
        Ok(())
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

        .subcommand(SubCommand::with_name("typetables")
                    .about("Typecheck a program and print its typed AST"))

        .subcommand(SubCommand::with_name("mips")
                    .about("Generate MIPS code for a program"))



        .get_matches();

    let cm = CompileManager;
    match compiler_match.subcommand_name() {
        Some("scan") => { cm.perform_action(CompileAction::Scan) }
        Some("tokens") => { cm.perform_action(CompileAction::DisplayTokens) }
        Some("parse") => { cm.perform_action(CompileAction::Parse) }
        Some("ast") => { cm.perform_action(CompileAction::DisplayAst) }
        Some("typecheck") => { cm.perform_action(CompileAction::Typecheck) }
        Some("typetables") => { cm.perform_action(CompileAction::TypeTables) }
        Some(_) => {}
        None => {
            println!("{}", compiler_match.usage());
        }
    }
}
