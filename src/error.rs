use pos::Pos;
use token::{Token, TokenType};
use types::Type;

#[derive(Debug)]
pub enum Error {
    GenericError,

    // Scanner errors
    IllegalCharacter(Pos, char),

    // Parser errors
    UnexpectedToken(Token, Vec<TokenType>), // Token contains position
    InvalidIntLiteral(Pos, String),
    InvalidFloatLiteral(Pos, String),

    // Typechecking errors
    UnexpectedType(Pos, Type, Type),
    UndeclaredVariable(Pos, String),
}
