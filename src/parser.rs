use scanner::{Token, TokenType};
use error::Error;

#[derive(Debug)]
enum Type {
    Int,
    Float,
}

#[derive(Debug)]
enum Decl {
    Decl(String, Type),
}

#[derive(Debug)]
enum Stmt {
    Read(String),
    Print(Expr),
    Assign(String, Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
}

#[derive(Debug)]
enum Expr {
    Id(String),
    Int(i64),
    Float(f64),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Program {
    decls: Vec<Decl>,
    stmts: Vec<Stmt>,
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
        match self.tokens[self.index] {
            Token::EmptyToken(ref t2) => t == *t2,
            Token::ValuedToken(ref t2, _) => t == *t2,
        }
    }

    fn eat(&mut self, t: TokenType) -> Result<(), Error> {
        let b = self.peek(t);
        if b {
            self.index += 1;
            Ok(())
        } else {
            Err(Error::GenericError)
        }
    }

    fn eat_lexeme(&mut self, t: TokenType) -> Result<String, Error> {
        let r =
            match self.tokens[self.index] {
                Token::EmptyToken(_) => Err(Error::GenericError),
                Token::ValuedToken(ref t2, ref lexeme) => {
                    if t == *t2 {
                        self.index += 1;
                        Ok(lexeme.clone())
                    } else {
                        Err(Error::GenericError)
                    }
                }
            };
        r
    }

    fn eat_type(&mut self) -> Result<Type, Error> {
        if self.peek(TokenType::TypeInt) {
            self.index += 1;
            Ok(Type::Int)
        } else if self.peek(TokenType::TypeFloat) {
            self.index += 1;
            Ok(Type::Float)
        } else {
            Err(Error::GenericError)
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let decls = try!(self.parse_decls());
        let stmts = try!(self.parse_stmts());
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
        try!(self.eat(TokenType::Var));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Colon));
        let ty = try!(self.eat_type());
        try!(self.eat(TokenType::Semicolon));
        Ok(Decl::Decl(id, ty))
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
            Err(Error::GenericError)
        }
    }

    fn parse_read(&mut self) -> Result<Stmt, Error> {
        try!(self.eat(TokenType::Read));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Read(id))
    }

    fn parse_print(&mut self) -> Result<Stmt, Error> {
        try!(self.eat(TokenType::Print));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Print(e))
    }

    fn parse_assign(&mut self) -> Result<Stmt, Error> {
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Equal));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Assign(id, e))
    }

    fn parse_if(&mut self) -> Result<Stmt, Error> {
        try!(self.eat(TokenType::If));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Then));
        let then_stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Else));
        let else_stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::End));
        Ok(Stmt::If(e, then_stmts, else_stmts))
    }

    fn parse_while(&mut self) -> Result<Stmt, Error> {
        try!(self.eat(TokenType::While));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Do));
        let stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Done));
        Ok(Stmt::While(e, stmts))
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        if self.peek(TokenType::Minus) {
            self.eat(TokenType::Minus);
            let e = try!(self.parse_expr());
            Ok(Expr::Negate(Box::new(e)))
        } else {
            let mut term = try!(self.parse_term());
            while self.next_is_add() {
                if self.peek(TokenType::Plus) {
                    self.eat(TokenType::Plus);
                    let t2 = try!(self.parse_term());
                    term = Expr::Add(Box::new(term), Box::new(t2));
                } else if self.peek(TokenType::Minus) {
                    self.eat(TokenType::Minus);
                    let t2 = try!(self.parse_term());
                    term = Expr::Sub(Box::new(term), Box::new(t2));
                } else {
                    return Err(Error::GenericError);
                }
            }
            Ok(term)
        }
    }

    fn parse_term(&mut self) -> Result<Expr, Error> {
        let mut fact = try!(self.parse_factor());
        while self.next_is_mul() {
            if self.peek(TokenType::Star) {
                self.eat(TokenType::Star);
                let f2 = try!(self.parse_factor());
                fact = Expr::Mul(Box::new(fact), Box::new(f2));
            } else if self.peek(TokenType::Slash) {
                self.eat(TokenType::Slash);
                let f2 = try!(self.parse_factor());
                fact = Expr::Div(Box::new(fact), Box::new(f2));
            } else {
                return Err(Error::GenericError);
            }
        }
        Ok(fact)
    }

    fn parse_factor(&mut self) -> Result<Expr, Error> {
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
        } else {
            Err(Error::GenericError)
        }
    }

    fn parse_int(&mut self) -> Result<Expr, Error> {
        let lexeme = try!(self.eat_lexeme(TokenType::Int));
        match lexeme.parse::<i64>() {
            Ok(n) => Ok(Expr::Int(n)),
            Err(_) => Err(Error::GenericError)
        }
    }

    fn parse_float(&mut self) -> Result<Expr, Error> {
        let lexeme = try!(self.eat_lexeme(TokenType::Float));
        match lexeme.parse::<f64>() {
            Ok(n) => Ok(Expr::Float(n)),
            Err(_) => Err(Error::GenericError)
        }
    }

    fn parse_id(&mut self) -> Result<Expr, Error> {
        let lexeme = try!(self.eat_lexeme(TokenType::Id));
        Ok(Expr::Id(lexeme))
    }

    fn parse_negate(&mut self) -> Result<Expr, Error> {
        try!(self.eat(TokenType::Minus));
        let e = try!(self.parse_expr());
        Ok(Expr::Negate(Box::new(e)))
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
