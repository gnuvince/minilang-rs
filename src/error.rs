use std::fmt;
use std::fmt::Display;
use std::result;

use pos::Pos;
use token::{Token, TokenType};
use types::Type;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GenericError,
    UsageError,

    // Scanner errors
    IllegalCharacter(Pos, char),

    // Parser errors
    UnexpectedToken(Token, Vec<TokenType>), // Token contains position
    InvalidIntLiteral(Pos, String),
    InvalidFloatLiteral(Pos, String),

    // Typechecking errors
    UnexpectedType { pos: Pos, expected: Type, actual: Type },
    DuplicateVariable(Pos, String),
    UndeclaredVariable(Pos, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::GenericError => { write!(f, "Generic Error") }
            Error::UsageError => { write!(f, "Usage Error") }

            Error::IllegalCharacter(pos, c) => {
                write!(f, "{}: Illegal character: '{}'", pos, c)
            }

            Error::UnexpectedToken(ref tok, ref choices) => {
                let _ = write!(f, "{}: Unexpected token. Found: {}. Expected: ", tok.pos, tok);
                let mut not_first = false;
                for choice in choices {
                    if not_first {
                        let _ = write!(f, ", ");
                    }
                    not_first = true;
                    let _ = write!(f, "{}", choice);
                }
                write!(f, "")
            }
            Error::InvalidIntLiteral(pos, ref s) =>
                write!(f, "{}: Invalid integer literal: '{}'", pos, s),
            Error::InvalidFloatLiteral(pos, ref s) =>
                write!(f, "{}: Invalid float literal: '{}'", pos, s),

            Error::UnexpectedType { pos, expected, actual } =>
                write!(f, "{}: Unexpected type. Found: {}. Expected: {}.", pos, actual, expected),
            Error::DuplicateVariable(pos, ref id) =>
                write!(f, "{}: Duplicate variable declaration: {}", pos, id),
            Error::UndeclaredVariable(pos, ref id) =>
                write!(f, "{}: Undeclared variable: {}", pos, id),
        }
    }
}
