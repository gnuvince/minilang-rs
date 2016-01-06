use error::Error;
use pos::Pos;
use token::{Token, TokenType};

pub struct Scanner {
    data: String,
    index: usize,
    start_pos: Pos,
    curr_pos: Pos,
}

impl Scanner {
    // Create a new scanner from a given program represented as a
    // String.
    pub fn new(data: String) -> Self {
        Scanner {
            data: data,
            index: 0,
            start_pos: Pos { line: 1, col: 1 },
            curr_pos: Pos { line: 1, col: 1 },
        }
    }

    // Internal function: return the character at the current index.
    fn peek(&self) -> char {
        match self.data.chars().nth(self.index) {
            Some(c) => c,
            None => '\x00',
        }
    }

    // Internal function: return the character at the current index
    // and increment the index by one.
    fn advance(&mut self) -> char {
        let c = self.peek();
        if !self.is_eof() {
            self.index += 1;
        }
        if c == '\n' {
            self.curr_pos.line += 1;
            self.curr_pos.col = 1;
        } else {
            self.curr_pos.col += 1
        }
        c
    }

    // Internal function: verify if the end of the program has been
    // reached.
    fn is_eof(&self) -> bool {
        self.index >= self.data.len()
    }



    pub fn next_token(&mut self) -> Result<Token, Error> {
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
            c if c.is_digit(10) => { self.scan_int_or_float() }
            c if is_id_start(c) => { self.scan_id_or_keyword() }
            c   => { Err(Error::IllegalCharacter(self.curr_pos, c)) }
        }
    }

    // Scan digits into an Int or Float token.
    fn scan_int_or_float(&mut self) -> Result<Token, Error> {
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
    fn scan_id_or_keyword(&mut self) -> Result<Token, Error> {
        let mut lexeme = String::new();
        while is_id_char(self.peek()) {
            lexeme.push(self.advance());
        }

        if lexeme == "if" { return Ok(self.empty_tok(TokenType::If)); }
        if lexeme == "then" { return Ok(self.empty_tok(TokenType::Then)); }
        if lexeme == "else" { return Ok(self.empty_tok(TokenType::Else)); }
        if lexeme == "end" { return Ok(self.empty_tok(TokenType::End)); }
        if lexeme == "while" { return Ok(self.empty_tok(TokenType::While)); }
        if lexeme == "do" { return Ok(self.empty_tok(TokenType::Do)); }
        if lexeme == "done" { return Ok(self.empty_tok(TokenType::Done)); }
        if lexeme == "read" { return Ok(self.empty_tok(TokenType::Read)); }
        if lexeme == "print" { return Ok(self.empty_tok(TokenType::Print)); }
        if lexeme == "var" { return Ok(self.empty_tok(TokenType::Var)); }
        if lexeme == "int" { return Ok(self.empty_tok(TokenType::TypeInt)); }
        if lexeme == "float" { return Ok(self.empty_tok(TokenType::TypeFloat)); }

        Ok(self.lexeme_tok(TokenType::Id, lexeme))
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
        let t = Token {
            typ: t,
            lexeme: None,
            pos: self.start_pos,
        };
        t
    }

    fn lexeme_tok(&self, t: TokenType, lexeme: String) -> Token {
        let t = Token {
            typ: t,
            lexeme: Some(lexeme),
            pos: self.start_pos,
        };
        t
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
