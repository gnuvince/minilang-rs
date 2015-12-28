use pos::Pos;
use token::TokenType;

#[derive(Debug)]
pub enum Error {
    GenericError,
    IllegalCharacter(char),

    UnexpectedToken(TokenType, TokenType)
}
