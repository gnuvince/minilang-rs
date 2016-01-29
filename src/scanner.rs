use error::{Result, Error};
use pos::Pos;
use token::{Token, TokenType};

use std::str::Chars;
use std::iter::Peekable;

pub struct Scanner<'a> {
    data: Peekable<Chars<'a>>,
    start_pos: Pos,
    curr_pos: Pos,
}

impl<'a> Scanner<'a> {
    // Create a new scanner from a given program represented as a
    // String.
    pub fn new<'b>(data: &'b str) -> Scanner<'b> {
        Scanner {
            data: data.chars().peekable(),
            start_pos: Pos { line: 1, col: 1 },
            curr_pos: Pos { line: 1, col: 1 },
        }
    }

    // Internal function: return the character at the current index.
    fn peek(&mut self) -> char {
        self.data.peek().map(|&c|c).unwrap_or('\x00')
    }

    // Internal function: return the character at the current index
    // and increment the index by one.
    fn advance(&mut self) -> char {
        let c = self.peek();
        if c == '\n' {
            self.curr_pos.line += 1;
            self.curr_pos.col = 1;
        } else {
            self.curr_pos.col += 1
        }
        let _ = self.data.next();
        c
    }

    // Internal function: verify if the end of the program has been
    // reached.
    fn is_eof(&mut self) -> bool {
        !self.data.peek().is_some()
    }



    pub fn next_token(&mut self) -> Result<Token> {
        // Discard blanks and comments.
        self.skip_comments_and_whitespace();

        // Set start_pos (starting position of the next token) to the
        // value of curr_pos (current position in the text stream).
        self.start_pos = self.curr_pos;

        // Return Eof if the end of the file has been reached.
        if self.is_eof() {
            return Ok(self.empty_tok(TokenType::Eof));
        }

        // Scanning dispatch.
        match self.peek() {
            '+' => { Ok(self.single_char_tok(TokenType::Plus)) }
            '-' => { Ok(self.single_char_tok(TokenType::Minus)) }
            '*' => { Ok(self.single_char_tok(TokenType::Star)) }
            '/' => { Ok(self.single_char_tok(TokenType::Slash)) }
            '=' => { Ok(self.single_char_tok(TokenType::Equal)) }
            '(' => { Ok(self.single_char_tok(TokenType::LParen)) }
            ')' => { Ok(self.single_char_tok(TokenType::RParen)) }
            ':' => { Ok(self.single_char_tok(TokenType::Colon)) }
            ';' => { Ok(self.single_char_tok(TokenType::Semicolon)) }
            ',' => { Ok(self.single_char_tok(TokenType::Comma)) }
            c if c.is_digit(10) => { self.scan_int_or_float() }
            c if is_id_start(c) => { self.scan_id_or_keyword() }
            c => { Err(Error::IllegalCharacter(self.curr_pos, c)) }
        }
    }

    // Scan digits into an Int or Float token.
    fn scan_int_or_float(&mut self) -> Result<Token> {
        let mut val = String::new();
        while self.peek().is_digit(10) {
            val.push(self.advance());
        }

        if self.peek() != '.' {
            return Ok(self.lexeme_tok(TokenType::Int, val));
        }

        val.push(self.advance()); // Add decimal point.

        while self.peek().is_digit(10) {
            val.push(self.advance());
        }

        Ok(self.lexeme_tok(TokenType::Float, val))
    }

    // Scan alpha-numeric characters into an Id or a keyword token.
    fn scan_id_or_keyword(&mut self) -> Result<Token> {
        let mut lexeme = String::new();
        while is_id_char(self.peek()) {
            lexeme.push(self.advance());
        }

        let token_type = match &*lexeme {
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "else" => TokenType::Else,
            "end" => TokenType::End,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "done" => TokenType::Done,
            "read" => TokenType::Read,
            "print" => TokenType::Print,
            "var" => TokenType::Var,
            "int" => TokenType::TypeInt,
            "float" => TokenType::TypeFloat,
            "void" => TokenType::TypeVoid,
            "function" => TokenType::Function,
            "return" => TokenType::Return,
            _ => TokenType::Id,
        };

        let token = if token_type == TokenType::Id {
            self.lexeme_tok(token_type, lexeme)
        } else {
            self.empty_tok(token_type)
        };

        Ok(token)
    }


    fn skip_comments_and_whitespace(&mut self) {
        loop {
            self.skip_whitespace();
            if self.peek() == '#' {
                self.skip_comment();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        while self.peek() != '\n' {
            self.advance();
        }
    }

    fn empty_tok(&self, t: TokenType) -> Token {
        Token {
            typ: t,
            lexeme: None,
            pos: self.start_pos,
        }
    }

    fn lexeme_tok(&self, t: TokenType, lexeme: String) -> Token {
        Token {
            typ: t,
            lexeme: Some(lexeme),
            pos: self.start_pos,
        }
    }

    fn single_char_tok(&mut self, t: TokenType) -> Token {
        let t = self.empty_tok(t);
        self.advance();
        t
    }
}

fn is_id_start(c: char) -> bool {
    (c >= 'a' && c <= 'z') ||
    c == '_' ||
    (c >= 'A' && c <= 'Z')
}


fn is_id_char(c: char) -> bool {
    (c >= 'a' && c <= 'z') ||
    c == '_' ||
    c.is_digit(10) ||
    (c >= 'A' && c <= 'Z')
}
