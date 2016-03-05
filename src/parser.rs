use token::{Token, TokenType};
use ast::*;
use pos::Pos;
use error::Error;
use types::Type;


pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    curr_id: u64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            index: 0,
            curr_id: 0,
        }
    }

    fn next_id(&mut self) -> u64 {
        let x = self.curr_id;
        self.curr_id += 1;
        x
    }

    fn peek(&self) -> TokenType {
        self.tokens[self.index].typ
    }

    fn curr_token(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn token_pos(&self) -> Pos {
        self.tokens[self.index].pos
    }

    fn eat(&mut self, t: TokenType) -> Result<(), Error> {
        if self.peek() == t {
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

    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let decls = try!(self.parse_decls());
        let stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Eof));

        Ok(Program {
            decls: decls,
            stmts: stmts,
        })
    }

    fn parse_type(&mut self) -> Result<Type, Error> {
        match self.peek() {
            TokenType::TypeInt => {
                let _ = try!(self.eat(TokenType::TypeInt));
                Ok(Type::Int)
            }
            TokenType::TypeFloat => {
                let _ = try!(self.eat(TokenType::TypeFloat));
                Ok(Type::Float)
            }
            TokenType::TypeString => {
                let _ = try!(self.eat(TokenType::TypeString));
                Ok(Type::String)
            }
            _ => {
                Err(Error::UnexpectedToken(self.curr_token(),
                                           vec![TokenType::TypeInt,
                                                TokenType::TypeFloat,
                                                TokenType::TypeString
                                           ]))
            }
        }
    }

    fn parse_decls(&mut self) -> Result<Vec<Decl>, Error> {
        let mut decls: Vec<Decl> = Vec::new();
        while self.peek() == TokenType::Var {
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
        let ty = try!(self.parse_type());
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
        match self.peek() {
            TokenType::Read => { self.parse_read() }
            TokenType::Print => { self.parse_print() }
            TokenType::Id => { self.parse_assign() }
            TokenType::If => { self.parse_if() }
            TokenType::While => { self.parse_while() }
            _ => {
                Err(Error::UnexpectedToken(
                    self.curr_token(),
                    vec![TokenType::Read, TokenType::Print,
                         TokenType::Id, TokenType::If, TokenType::While]))
            }
        }
    }

    fn parse_read(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Read));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Read(StmtRead { pos: pos, id: id }))
    }

    fn parse_print(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Print));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Print(StmtPrint { pos: pos, expr: e }))
    }

    fn parse_assign(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Equal));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Assign(StmtAssign { pos: pos, id: id, expr: e }))
    }

    fn parse_if(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::If));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Then));
        let then_stmts = try!(self.parse_stmts());

        let else_stmts =
            if self.peek() == TokenType::Else {
                try!(self.eat(TokenType::Else));
                try!(self.parse_stmts())
            } else {
                vec![]
            };

        try!(self.eat(TokenType::EndIf));
        Ok(Stmt::If(StmtIf {
            pos: pos,
            expr: e,
            then_stmts: then_stmts,
            else_stmts: else_stmts,
        }))
    }

    fn parse_while(&mut self) -> Result<Stmt, Error> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::While));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Do));
        let stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Done));
        Ok(Stmt::While(StmtWhile {
            pos: pos,
            expr: e,
            stmts: stmts,
        }))
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let mut term = try!(self.parse_term());
        while self.next_is_add() {
            let tok = self.peek();
            let op =
                match tok {
                    TokenType::Plus => Binop::Add,
                    TokenType::Minus => Binop::Sub,
                    _ => {
                        return Err(Error::UnexpectedToken(
                            self.curr_token(),
                            vec![TokenType::Plus, TokenType::Minus]));
                    }
                };
            try!(self.eat(tok));
            let t2 = try!(self.parse_term());
            term = Expr {
                pos: pos,
                node_id: self.next_id(),
                expr: Expr_::Binop(ExprBinop {
                    op: op,
                    expr1: Box::new(term),
                    expr2: Box::new(t2)
                })
            };
        }
        Ok(term)
    }

    fn parse_term(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let mut fact = try!(self.parse_factor());
        while self.next_is_mul() {
            let tok = self.peek();
            let op =
                match tok {
                    TokenType::Star => Binop::Mul,
                    TokenType::Slash => Binop::Div,
                    _ => {
                        return Err(Error::UnexpectedToken(
                            self.curr_token(),
                            vec![TokenType::Star, TokenType::Slash]));
                    }
                };
            try!(self.eat(tok));
            let f2 = try!(self.parse_factor());
            fact = Expr {
                pos: pos,
                node_id: self.next_id(),
                expr: Expr_::Binop(ExprBinop {
                    op: op,
                    expr1: Box::new(fact),
                    expr2: Box::new(f2),
                })
            };
        }
        Ok(fact)
    }

    fn parse_factor(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        match self.peek() {
            TokenType::Int => { self.parse_int() }
            TokenType::Float => { self.parse_float() }
            TokenType::String => { self.parse_string() }
            TokenType::Id => { self.parse_id() }
            TokenType::LParen => {
                try!(self.eat(TokenType::LParen));
                let e = try!(self.parse_expr());
                try!(self.eat(TokenType::RParen));
                Ok(e)
            }
            TokenType::Minus => {
                try!(self.eat(TokenType::Minus));
                let e = try!(self.parse_expr());
                Ok(Expr {
                    pos: pos,
                    node_id: self.next_id(),
                    expr: Expr_::Negate(ExprNegate {
                        expr: Box::new(e)
                    })
                })
            }
            _ => {
                Err(Error::UnexpectedToken(
                    self.curr_token(),
                    vec![TokenType::Int, TokenType::Float, TokenType::Id,
                         TokenType::Minus, TokenType::LParen]))
            }
        }
    }

    fn parse_int(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Int));
        match lexeme.parse::<i64>() {
            Ok(n) => Ok(Expr {
                pos: pos,
                node_id: self.next_id(),
                expr: Expr_::Int(ExprInt {
                    value: n
                })
            }),
            Err(_) => Err(Error::InvalidIntLiteral(pos, lexeme))
        }
    }

    fn parse_float(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Float));
        match lexeme.parse::<f64>() {
            Ok(n) => Ok(Expr {
                pos: pos,
                node_id: self.next_id(),
                expr: Expr_::Float(ExprFloat {
                    value: n
                })
            }),
            Err(_) => Err(Error::InvalidFloatLiteral(pos, lexeme))
        }
    }

    fn parse_string(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::String));
        Ok(Expr {
            pos: pos,
            node_id: self.next_id(),
            expr: Expr_::String(ExprString {
                value: lexeme
            })
        })
    }

    fn parse_id(&mut self) -> Result<Expr, Error> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Id));
        Ok(Expr {
            pos: pos,
            node_id: self.next_id(),
            expr: Expr_::Id(ExprId {
                id: lexeme
            })
        })
    }

    fn is_stmt_start(&self) -> bool {
        self.peek() == TokenType::Id
            || self.peek() == TokenType::If
            || self.peek() == TokenType::While
            || self.peek() == TokenType::Read
            || self.peek() == TokenType::Print
    }

    fn next_is_add(&self) -> bool {
        self.peek() == TokenType::Plus || self.peek() == TokenType::Minus
    }

    fn next_is_mul(&self) -> bool {
        self.peek() == TokenType::Star || self.peek() == TokenType::Slash
    }
}
