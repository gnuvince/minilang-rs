mod error;
mod pos;
mod token;
mod scanner;
mod types;
mod parser;

use token::{Token, TokenType};
use scanner::Scanner;
use parser::Parser;
use std::io::{Read, stdin};

fn main() {
    let mut data: String = String::new();
    let _ = stdin().read_to_string(&mut data);
    let mut scanner = Scanner::new(data);

    let mut tokens: Vec<Token> = Vec::new();

    loop {
        match scanner.next_token() {
            Ok(tok) => {
                let is_eof = tok.typ == TokenType::Eof;
                tokens.push(tok);
                if is_eof {
                    break;
                }
            }
            Err(e) => { println!("Error: {:?}", e); return; }
        }
    }

    for tok in tokens.iter() {
        println!("{:?}", tok);
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();
    println!("{:?}", program);
}
