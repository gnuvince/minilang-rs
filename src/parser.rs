use token::{Token, TokenType};
use ast::*;
use pos::Pos;
use error::{Result, Error};
use types::Type;


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

    fn eat(&mut self, t: TokenType) -> Result<()> {
        let b = self.peek(t);
        if b {
            self.index += 1;
            Ok(())
        } else {
            Err(Error::UnexpectedToken(self.curr_token(), vec![t]))
        }
    }

    fn eat_lexeme(&mut self, t: TokenType) -> Result<String> {
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

    fn eat_type(&mut self) -> Result<Type> {
        if self.peek(TokenType::TypeInt) {
            self.index += 1;
            Ok(Type::Int)
        } else if self.peek(TokenType::TypeFloat) {
            self.index += 1;
            Ok(Type::Float)
        } else if self.peek(TokenType::TypeVoid) {
            self.index += 1;
            Ok(Type::Void)
        } else {
            Err(Error::UnexpectedToken(self.curr_token(), vec![TokenType::TypeInt, TokenType::TypeFloat]))
        }
    }

    /*
     * program = { decl } .
     */
    pub fn parse_program(&mut self) -> Result<Program> {
        let decls = try!(self.parse_decls());
        try!(self.eat(TokenType::Eof));

        Ok(Program {
            decls: decls,
        })
    }

    fn parse_decls(&mut self) -> Result<Vec<Decl>> {
        let mut decls: Vec<Decl> = Vec::new();
        while self.peek(TokenType::Var) || self.peek(TokenType::Function) {
            let decl = try!(self.parse_decl());
            decls.push(decl);
        }
        Ok(decls)
    }

    /*
     * decl = var_decl
     *      | fun_decl
     */
    fn parse_decl(&mut self) -> Result<Decl> {
        if self.peek(TokenType::Var) {
            let vd = try!(self.parse_var_decl());
            Ok(Decl::Var(vd))
        } else if self.peek(TokenType::Function) {
            let fd = try!(self.parse_func_decl());
            Ok(Decl::Fun(fd))
        } else {
            Err(Error::UnexpectedToken(
                self.curr_token(),
                vec![TokenType::Var, TokenType::Function]))
        }
    }

    fn parse_var_decls(&mut self) -> Result<Vec<VarDecl>> {
        let mut vds = Vec::new();
        while self.peek(TokenType::Var) {
            let vd = try!(self.parse_var_decl());
            vds.push(vd);
        }
        Ok(vds)
    }

    /*
     * var_decl = "var" id ":" type ";"
     */
    fn parse_var_decl(&mut self) -> Result<VarDecl> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Var));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Colon));
        let ty = try!(self.eat_type());
        try!(self.eat(TokenType::Semicolon));
        Ok(VarDecl { pos: pos, id: id, ty: ty })
    }

    /*
     * fun_decl   = "function" id "(" param_list ")" ":" type { stmt } "end"
     */
    fn parse_func_decl(&mut self) -> Result<FunDecl> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Function));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::LParen));
        let params = try!(self.parse_params());
        try!(self.eat(TokenType::RParen));
        try!(self.eat(TokenType::Colon));
        let ty = try!(self.eat_type());
        let decls = try!(self.parse_var_decls());
        let stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::End));

        Ok(FunDecl {
            pos: pos,
            id: id,
            params: params,
            ty: ty,
            decls: decls,
            stmts: stmts,
        })
    }


    /*
     * params = ε
     *        | id ":" type
     *        | id ":" type "," params
     */
    fn parse_params(&mut self) -> Result<Vec<(String, Type)>> {
        let mut params = Vec::new();
        while self.peek(TokenType::Id) {
            let id = try!(self.eat_lexeme(TokenType::Id));
            try!(self.eat(TokenType::Colon));
            let ty = try!(self.eat_type());
            params.push((id, ty));

            if self.peek(TokenType::Comma) {
                try!(self.eat(TokenType::Comma));
            } else {
                break;
            }
        }
        Ok(params)
    }


    fn parse_stmts(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while self.is_stmt_start() {
            let stmt = try!(self.parse_stmt());
            stmts.push(stmt);
        }
        Ok(stmts)
    }

    /*
     * stmt = read_stmt
     *      | print_stmt
     *      | assign_stmt
     *      | if_stmt
     *      | while_stmt
     *      | return_stmt
     */
    fn parse_stmt(&mut self) -> Result<Stmt> {
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
        } else if self.peek(TokenType::Return) {
            self.parse_return()
        } else {
            Err(Error::UnexpectedToken(
                self.curr_token(),
                vec![TokenType::Read, TokenType::Print,
                     TokenType::Id, TokenType::If, TokenType::While]))
        }
    }

    /*
     * read_stmt = "read" id ";"
     */
    fn parse_read(&mut self) -> Result<Stmt> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Read));
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Read(StmtRead { pos: pos, id: id }))
    }

    /*
     * print_stmt = "print" expr ";"
     */
    fn parse_print(&mut self) -> Result<Stmt> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Print));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Print(StmtPrint { pos: pos, expr: e }))
    }

    /*
     * assign_stmt = id "=" expr ";"
     */
    fn parse_assign(&mut self) -> Result<Stmt> {
        let pos = self.token_pos();
        let id = try!(self.eat_lexeme(TokenType::Id));
        try!(self.eat(TokenType::Equal));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Semicolon));
        Ok(Stmt::Assign(StmtAssign { pos: pos, id: id, expr: e }))
    }

    /*
     * if_stmt = "if" expr "then" { stmt } [ "else" { stmt } ] "end"
     */
    fn parse_if(&mut self) -> Result<Stmt> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::If));
        let e = try!(self.parse_expr());
        try!(self.eat(TokenType::Then));
        let then_stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::Else));
        let else_stmts = try!(self.parse_stmts());
        try!(self.eat(TokenType::End));
        Ok(Stmt::If(StmtIf {
            pos: pos,
            expr: e,
            then_stmts: then_stmts,
            else_stmts: else_stmts,
        }))
    }

    /*
     * while_stmt = "while" expr "do" { stmt } "done"
     */
    fn parse_while(&mut self) -> Result<Stmt> {
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

    /*
     * return_stmt = "return" [ expr ] ";"
     */
    fn parse_return(&mut self) -> Result<Stmt> {
        let pos = self.token_pos();
        try!(self.eat(TokenType::Return));

        let expr =
            if self.peek(TokenType::Semicolon) {
                None
            } else {
                Some(try!(self.parse_expr()))
            };
        try!(self.eat(TokenType::Semicolon));

        Ok(Stmt::Return(StmtReturn {
            pos: pos,
            expr: expr,
        }))
    }

    /*
     * expr = expr "+" term
     *      | expr "-" term
     *      | term
     */
    fn parse_expr(&mut self) -> Result<Expr> {
        let pos = self.token_pos();
        let mut expr = try!(self.parse_term());
        while self.next_is_add() {
            if self.peek(TokenType::Plus) {
                try!(self.eat(TokenType::Plus));
                let term = try!(self.parse_term());
                expr = Expr::Binop(ExprBinop {
                    pos: pos,
                    op: Binop::Add,
                    expr1: Box::new(expr),
                    expr2: Box::new(term)
                });
            } else if self.peek(TokenType::Minus) {
                try!(self.eat(TokenType::Minus));
                let term = try!(self.parse_term());
                expr = Expr::Binop(ExprBinop {
                    pos: pos,
                    op: Binop::Sub,
                    expr1: Box::new(expr),
                    expr2: Box::new(term)
                });
            } else {
                return Err(Error::UnexpectedToken(
                    self.curr_token(),
                    vec![TokenType::Plus, TokenType::Minus]));
            }
        }
        Ok(expr)
    }

    /*
     * term = term "+" factor
     *      | term "-" factor
     *      | factor
     */
    fn parse_term(&mut self) -> Result<Expr> {
        let pos = self.token_pos();
        let mut fact = try!(self.parse_factor());
        while self.next_is_mul() {
            if self.peek(TokenType::Star) {
                try!(self.eat(TokenType::Star));
                let f2 = try!(self.parse_factor());
                fact = Expr::Binop(ExprBinop {
                    pos: pos,
                    op: Binop::Mul,
                    expr1: Box::new(fact),
                    expr2: Box::new(f2),
                });
            } else if self.peek(TokenType::Slash) {
                try!(self.eat(TokenType::Slash));
                let f2 = try!(self.parse_factor());
                fact = Expr::Binop(ExprBinop {
                    pos: pos,
                    op: Binop::Div,
                    expr1: Box::new(fact),
                    expr2: Box::new(f2),
                });
            } else {
                return Err(Error::UnexpectedToken(
                    self.curr_token(),
                    vec![TokenType::Star, TokenType::Slash]));
            }
        }
        Ok(term)
    }

    /*
     * factor = id
     *        | id "(" expr* ")"
     *        | int_literal
     *        | float_literal
     *        | "(" expr ")"
     *        | "-" expr
     */
    fn parse_factor(&mut self) -> Result<Expr> {
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
            Ok(Expr::Negate(ExprNegate { pos: pos, expr: Box::new(e) }))
        } else {
            Err(Error::UnexpectedToken(
                self.curr_token(),
                vec![TokenType::Int, TokenType::Float, TokenType::Id,
                     TokenType::Minus, TokenType::LParen]))
        }
    }

    fn parse_int(&mut self) -> Result<Expr> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Int));
        match lexeme.parse::<i64>() {
            Ok(n) => Ok(Expr::Int (ExprInt { pos: pos, value: n })),
            Err(_) => Err(Error::InvalidIntLiteral(pos, lexeme))
        }
    }

    fn parse_float(&mut self) -> Result<Expr> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Float));
        match lexeme.parse::<f64>() {
            Ok(n) => Ok(Expr::Float(ExprFloat { pos: pos, value: Float(n) })),
            Err(_) => Err(Error::InvalidFloatLiteral(pos, lexeme))
        }
    }

    fn parse_id(&mut self) -> Result<Expr> {
        let pos = self.token_pos();
        let lexeme = try!(self.eat_lexeme(TokenType::Id));
        Ok(Expr::Id(ExprId { pos: pos, id: lexeme }))
    }

    fn is_stmt_start(&self) -> bool {
        self.peek(TokenType::Id) ||
            self.peek(TokenType::If) ||
            self.peek(TokenType::While) ||
            self.peek(TokenType::Read) ||
            self.peek(TokenType::Print) ||
            self.peek(TokenType::Return)
    }

    fn next_is_add(&self) -> bool {
        self.peek(TokenType::Plus) || self.peek(TokenType::Minus)
    }

    fn next_is_mul(&self) -> bool {
        self.peek(TokenType::Star) || self.peek(TokenType::Slash)
    }
}
