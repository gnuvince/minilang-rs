mod error;
mod scanner;
mod pos;
mod parser;

use scanner::{Scanner, Token, TokenType};
use parser::Parser;
use std::io::{Read, stdin};

fn main() {
    let mut data: String = String::new();
    let _ = stdin().read_to_string(&mut data);
    let mut scanner = Scanner::new(data);

    let mut tokens: Vec<Token> = Vec::new();

    loop {
        match scanner.next_token() {
            Ok(Token::EmptyToken(TokenType::Eof)) => { tokens.push(Token::EmptyToken(TokenType::Eof)); break; }
            Ok(tok) => { tokens.push(tok); }
            Err(e) => { println!("Error: {:?}", e); return; }
        }
    }

    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();
    println!("{:?}", program);
}
