use std::collections::HashMap;

use ast::*;
use types::Type;
use error::Error;

pub type Symtable = HashMap<String, Type>;
pub type Exprtable = HashMap<Expr, Type>;

pub struct TypeChecker {
    pub symtable: Symtable,
    pub expr_table: Exprtable,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symtable: HashMap::new(),
            expr_table: HashMap::new(),
        }
    }

    pub fn tc_program(&mut self, p: &Program) -> Result<(), Error> {
        try!(self.tc_decls(&p.decls));
        self.tc_stmts(&p.stmts)
    }

    fn tc_decls(&mut self, decls: &[Decl]) -> Result<(), Error> {
        for decl in decls {
            try!(self.tc_decl(&decl));
        }
        Ok(())
    }

    fn tc_decl(&mut self, decl: &Decl) -> Result<(), Error> {
        // if self.symtable.contains_key(&decl.id) {
        //     Err(Error::DuplicateVariable(decl.pos, decl.id.clone()))
        // } else {
        //     self.symtable.insert(decl.id.clone(), decl.ty);
        //     Ok(())
        // }
        unimplemented!()
    }

    fn tc_stmts(&mut self, stmts: &[Stmt]) -> Result<(), Error> {
        for stmt in stmts {
            try!(self.tc_stmt(&stmt));
        }
        Ok(())
    }

    fn tc_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match *stmt {
            Stmt::Assign(ref stmt_) => self.tc_stmt_assign(stmt_),
            Stmt::Read(ref stmt_) => self.tc_stmt_read(stmt_),
            Stmt::Print(ref stmt_) => self.tc_stmt_print(stmt_),
            Stmt::If(ref stmt_) => self.tc_stmt_if(stmt_),
            Stmt::While(ref stmt_) => self.tc_stmt_while(stmt_),
            Stmt::Return(_) => Ok(()),
        }
    }

    fn tc_stmt_assign(&mut self, stmt: &StmtAssign) -> Result<(), Error> {
        let expr_ty = try!(self.tc_expr(&stmt.expr));
        match self.symtable.get(&stmt.id) {
            Some(&id_ty) => {
                match (id_ty, expr_ty) {
                    (Type::Int, Type::Int) => Ok(()),
                    (Type::Int, Type::Float) =>
                        Err(Error::UnexpectedType { pos: stmt.pos, expected: Type::Int, actual: Type::Float }),
                    (Type::Float, Type::Int) => Ok(()),
                    (Type::Float, Type::Float) => Ok(()),
                }
            }
            None => Err(Error::UndeclaredVariable(stmt.pos, stmt.id.clone()))
        }
    }

    fn tc_stmt_read(&mut self, stmt: &StmtRead) -> Result<(), Error> {
        if self.symtable.contains_key(&stmt.id) {
            Ok(())
        } else {
            Err(Error::UndeclaredVariable(stmt.pos, stmt.id.clone()))
        }
    }

    fn tc_stmt_print(&mut self, stmt: &StmtPrint) -> Result<(), Error> {
        try!(self.tc_expr(&stmt.expr));
        Ok(())
    }

    fn tc_stmt_if(&mut self, stmt: &StmtIf) -> Result<(), Error> {
        let t = try!(self.tc_expr(&stmt.expr));
        match t {
            Type::Int => {
                try!(self.tc_stmts(&stmt.then_stmts));
                try!(self.tc_stmts(&stmt.else_stmts));
                Ok(())
            }
            _ => { Err(Error::UnexpectedType{ pos: stmt.pos, expected: Type::Int, actual: t }) }
        }
    }

    fn tc_stmt_while(&mut self, stmt: &StmtWhile) -> Result<(), Error> {
        let t = try!(self.tc_expr(&stmt.expr));
        match t {
            Type::Int => { self.tc_stmts(&stmt.stmts) }
            _ => { Err(Error::UnexpectedType { pos: stmt.pos, expected: Type::Int, actual: t }) }
        }
    }

    fn tc_expr(&mut self, expr: &Expr) -> Result<Type, Error> {
        let ty = try!(match *expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Id(ref expr_) => self.tc_expr_id(expr_),
            Expr::Negate(ref expr_) => self.tc_expr_negate(expr_),
            Expr::Binop(ref expr_) => self.tc_expr_binop(expr_),
        });

        let expr_copy = expr.clone();
        self.expr_table.insert(expr_copy, ty);

        Ok(ty)
    }

    fn tc_expr_id(&mut self, expr: &ExprId) -> Result<Type, Error> {
        match self.symtable.get(&expr.id) {
            Some(ty) => Ok(*ty),
            None => Err(Error::UndeclaredVariable(expr.pos, expr.id.clone())),
        }
    }

    fn tc_expr_negate(&mut self, expr: &ExprNegate) -> Result<Type, Error> {
        self.tc_expr(&expr.expr)
    }

    fn tc_expr_binop(&mut self, expr: &ExprBinop) -> Result<Type, Error> {
        let t1 = try!(self.tc_expr(&expr.expr1));
        let t2 = try!(self.tc_expr(&expr.expr2));

        match (t1, t2) {
            (Type::Int   , Type::Int)   => Ok(Type::Int),
            (Type::Int   , Type::Float) => Ok(Type::Float),
            (Type::Float , Type::Int)   => Ok(Type::Float),
            (Type::Float , Type::Float) => Ok(Type::Float),
        }
    }
}
