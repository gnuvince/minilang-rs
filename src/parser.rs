use std::mem;
use std::hash::{Hash, Hasher};

use token::{Token, TokenType};
use pos::Pos;
use error::Error;
use types::Type;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Float(f64);

impl Hash for Float {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        unsafe {
            let h: i64 = mem::transmute(*self);
            state.write_i64(h);
            state.finish();
        }
    }
}

impl Eq for Float {

}

#[derive(Debug)]
pub struct Decl {
    pub pos: Pos,
    pub id: String,
    pub ty: Type,
}

#[derive(Debug)]
pub enum Stmt {
    Read { pos: Pos, id: String },
    Print { pos: Pos, expr: Expr },
    Assign { pos: Pos, id: String, expr: Expr },
    If { pos: Pos, expr: Expr, then_stmts: Vec<Stmt>, else_stmts: Vec<Stmt> },
    While { pos: Pos, expr: Expr, stmts: Vec<Stmt> },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expr {
    Id { pos: Pos, id: String },
    Int { pos: Pos, value: i64 },
    Float { pos: Pos, value: Float },
    Negate { pos: Pos, expr: Box<Expr> },
    Add { pos: Pos, expr1: Box<Expr>, expr2: Box<Expr> },
    Sub { pos: Pos, expr1: Box<Expr>, expr2: Box<Expr> },
    Mul { pos: Pos, expr1: Box<Expr>, expr2: Box<Expr> },
    Div { pos: Pos, expr1: Box<Expr>, expr2: Box<Expr> },
}

#[derive(Debug)]
pub struct Program {
    pub decls: Vec<Decl>,
    pub stmts: Vec<Stmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            index: 0,
        }
    }

    fn peek(&self, t: TokenType) -> bool {
        let token = &self.tokens[self.index];
        token.typ == t
    }

    fn curr_token(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn token_pos(&self) -> Pos {
        self.tokens[self.index].pos
    }

    fn eat(&mut self, t: TokenType) -> Result<(), Error> {
        let b = self.peek(t);
        if b {
            self.index += 1;
            Ok(())
        } else {
            Err(Error::UnexpectedToken(self.curr_token(), vec![t]))
        }
    }

    fn eat_lexeme(&mut self, t: TokenType) -> Result<String, Error> {
        let token = &self.tokens[self.index];
        if token.typ == t {
            match token.lexeme {
                Some(ref lexeme) => {
                    self.index += 1;
                    Ok(lexeme.clone())
                }
                None => Err(Error::UnexpectedToken(self.curr_token(), vec![t]))
            }
        } else {
            Err(Error::UnexpectedToken(self.curr_token(), vec![t]))
        }
    }

    fn eat_type(&mut self) -> Result<Type, Error> {
        if self.peek(TokenType::TypeInt) {
            self.index += 1;
            Ok(Type::Int)
        } else if self.peek(TokenType::TypeFloat) {
            self.index += 1;
            Ok(Type::Float)
        } else {
            Err(Error::UnexpectedToken(self.curr_token(), vec![TokenType::TypeInt, TokenType::TypeFloat]))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let decls = try!(self.parse_decls());
        let stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Eof));

        Ok(Program {
            decls: decls,
            stmts: stmts,
        })
    }

    fn parse_decls(&mut self) -> Result<Vec<Decl>, Error> {
        let mut decls: Vec<Decl> = Vec::new();
        while self.peek(TokenType::Var) {
            let decl = try!(self.parse_decl());
            decls.push(decl);
        }
        Ok(decls)
    }

    fn parse_decl(&mut self) -> Result<Decl, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Var));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Colon));
        let ty = try!(self.eat_type());
        try!(self.eat(TokenType::Semicolon));
        Ok(Decl { pos: pos, id: id, ty: ty })
    }


    fn parse_stmts(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while self.is_stmt_start() {
            let stmt = try!(self.parse_stmt());
            stmts.push(stmt);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, Error> {
        if self.peek(TokenType::Read) {
            self.parse_read()
        } else if self.peek(TokenType::Print) {
            self.parse_print()
        } else if self.peek(TokenType::Id) {
            self.parse_assign()
        } else if self.peek(TokenType::If) {
            self.parse_if()
        } else if self.peek(TokenType::While) {
            self.parse_while()
        } else {
            Err(Error::UnexpectedToken(
                self.curr_token(),
                vec![TokenType::Read, TokenType::Print,
                     TokenType::Id, TokenType::If, TokenType::While]))
        }
    }

    fn parse_read(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Read));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Read { pos: pos, id: id })
    }

    fn parse_print(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Print));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Print { pos: pos, expr: e })
    }

    fn parse_assign(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Equal));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Assign { pos: pos, id: id, expr: e })
    }

    fn parse_if(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::If));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Then));
        let then_stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Else));
        let else_stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::End));
        Ok(Stmt::If {
            pos: pos,
            expr: e,
            then_stmts: then_stmts,
            else_stmts: else_stmts,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::While));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Do));
        let stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Done));
        Ok(Stmt::While {
            pos: pos,
            expr: e,
            stmts: stmts,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let mut term = try!(self.parse_term());
        while self.next_is_add() {
            if self.peek(TokenType::Plus) {
                self.eat(TokenType::Plus);
                let t2 = try!(self.parse_term());
                term = Expr::Add {
                    pos: pos,
                    expr1: Box::new(term),
                    expr2: Box::new(t2)
                };
            } else if self.peek(TokenType::Minus) {
                self.eat(TokenType::Minus);
                let t2 = try!(self.parse_term());
                term = Expr::Sub {
                    pos: pos,
                    expr1: Box::new(term),
                    expr2: Box::new(t2)
                };
            } else {
                return Err(Error::UnexpectedToken(
                    self.curr_token(),
                    vec![TokenType::Plus, TokenType::Minus]));
            }
        }
        Ok(term)
    }

    fn parse_term(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let mut fact = try!(self.parse_factor());
        while self.next_is_mul() {
            if self.peek(TokenType::Star) {
                self.eat(TokenType::Star);
                let f2 = try!(self.parse_factor());
                fact = Expr::Mul {
                    pos: pos,
                    expr1: Box::new(fact),
                    expr2: Box::new(f2),
                };
            } else if self.peek(TokenType::Slash) {
                self.eat(TokenType::Slash);
                let f2 = try!(self.parse_factor());
                fact = Expr::Div {
                    pos: pos,
                    expr1: Box::new(fact),
                    expr2: Box::new(f2),
                };
            } else {
                return Err(Error::UnexpectedToken(
                    self.curr_token(),
                    vec![TokenType::Star, TokenType::Slash]));
            }
        }
        Ok(fact)
    }

    fn parse_factor(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        if self.peek(TokenType::Int) {
            self.parse_int()
        } else if self.peek(TokenType::Float) {
            self.parse_float()
        } else if self.peek(TokenType::Id) {
            self.parse_id()
        } else if self.peek(TokenType::LParen) {
            try!(self.eat(TokenType::LParen));
            let e = try!(self.parse_expr());
            try!(self.eat(TokenType::RParen));
            Ok(e)
        } else if self.peek(TokenType::Minus) {
            try!(self.eat(TokenType::Minus));
            let e = try!(self.parse_expr());
            Ok(Expr::Negate { pos: pos, expr: Box::new(e) })
        } else {
            Err(Error::UnexpectedToken(
                self.curr_token(),
                vec![TokenType::Int, TokenType::Float, TokenType::Id,
                     TokenType::Minus, TokenType::LParen]))
        }
    }

    fn parse_int(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Int));
        match lexeme.parse::<i64>() {
            Ok(n) => Ok(Expr::Int { pos: pos, value: n }),
            Err(_) => Err(Error::InvalidIntLiteral(pos, lexeme))
        }
    }

    fn parse_float(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Float));
        match lexeme.parse::<f64>() {
            Ok(n) => Ok(Expr::Float { pos: pos, value: Float(n) }),
            Err(_) => Err(Error::InvalidFloatLiteral(pos, lexeme))
        }
    }

    fn parse_id(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Id));
        Ok(Expr::Id { pos: pos, id: lexeme })
    }

    fn parse_negate(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Minus));
        let e = try!(self.parse_expr());
        Ok(Expr::Negate { pos: pos, expr: Box::new(e) })
    }

    fn is_stmt_start(&self) -> bool {
        self.peek(TokenType::Id) ||
            self.peek(TokenType::If) ||
            self.peek(TokenType::While) ||
            self.peek(TokenType::Read) ||
            self.peek(TokenType::Print)
    }

    fn next_is_add(&self) -> bool {
        self.peek(TokenType::Plus) || self.peek(TokenType::Minus)
    }

    fn next_is_mul(&self) -> bool {
        self.peek(TokenType::Star) || self.peek(TokenType::Slash)
    }
}
