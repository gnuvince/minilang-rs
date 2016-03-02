use std::fmt;

use pos::Pos;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Values
    Int,
    Float,
    String,
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
    EndIf,
    While,
    Do,
    Done,
    Read,
    Print,
    Var,
    TypeInt,
    TypeFloat,
    TypeString,

    // Others
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Int => write!(f, "integer"),
            TokenType::Float => write!(f, "float"),
            TokenType::String => write!(f, "string"),
            TokenType::Id => write!(f, "identifier"),
            TokenType::Plus => write!(f, "'+'"),
            TokenType::Minus => write!(f, "'-'"),
            TokenType::Star => write!(f, "'*'"),
            TokenType::Slash => write!(f, "'/'"),
            TokenType::Equal => write!(f, "'='"),
            TokenType::LParen => write!(f, "'('"),
            TokenType::RParen => write!(f, "')'"),
            TokenType::Colon => write!(f, "':'"),
            TokenType::Semicolon => write!(f, "';'"),
            TokenType::If => write!(f, "'if'"),
            TokenType::Then => write!(f, "'then'"),
            TokenType::Else => write!(f, "'else'"),
            TokenType::EndIf => write!(f, "'endif'"),
            TokenType::While => write!(f, "'while'"),
            TokenType::Do => write!(f, "'do'"),
            TokenType::Done => write!(f, "'done'"),
            TokenType::Read => write!(f, "'read'"),
            TokenType::Print => write!(f, "'print'"),
            TokenType::Var => write!(f, "'var'"),
            TokenType::TypeInt => write!(f, "'int'"),
            TokenType::TypeFloat => write!(f, "'float'"),
            TokenType::TypeString => write!(f, "'string'"),
            TokenType::Eof => write!(f, "<eof>"),
        }
    }
}

#[derive(Clone, Debug)]

pub struct Token {
    pub typ: TokenType,
    pub lexeme: Option<String>,
    pub pos: Pos,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.lexeme {
            Some(ref s) => write!(f, "{} ({})", self.typ, s),
            None => write!(f, "{}", self.typ)
        }
    }
}
