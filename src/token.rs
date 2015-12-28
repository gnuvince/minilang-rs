use ::pos::Pos;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Values
    Int,
    Float,
    Id,

    // Punctuation and operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    Colon,
    Semicolon,

    // Keywords
    If,
    Then,
    Else,
    End,
    While,
    Do,
    Done,
    Read,
    Print,
    Var,
    TypeInt,
    TypeFloat,

    // Others
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub lexeme: Option<String>,
}
