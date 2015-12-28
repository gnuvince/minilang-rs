use pos::Pos;
use token::{Token, TokenType};

#[derive(Debug)]
pub enum Error {
    GenericError,
    IllegalCharacter(Pos, char),

    UnexpectedToken(Token, TokenType)
}
