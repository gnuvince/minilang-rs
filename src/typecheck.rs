use std::collections::HashMap;

use parser::{Program, Decl, Stmt, Expr};
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

    fn tc_decls(&mut self, decls: &Vec<Decl>) -> Result<(), Error> {
        for decl in decls {
            try!(self.tc_decl(&decl));
        }
        Ok(())
    }

    fn tc_decl(&mut self, decl: &Decl) -> Result<(), Error> {
        if self.symtable.contains_key(&decl.id) {
            Err(Error::DuplicateVariable(decl.pos, decl.id.clone()))
        } else {
            self.symtable.insert(decl.id.clone(), decl.ty);
            Ok(())
        }
    }

    fn tc_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            try!(self.tc_stmt(&stmt));
        }
        Ok(())
    }

    fn tc_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match *stmt {
            Stmt::Assign { .. } => self.tc_stmt_assign(stmt),
            Stmt::Read { .. } => self.tc_stmt_read(stmt),
            Stmt::Print { .. } => self.tc_stmt_print(stmt),
            Stmt::If { .. } => self.tc_stmt_if(stmt),
            Stmt::While { .. } => self.tc_stmt_while(stmt),
        }
    }

    fn tc_stmt_assign(&mut self, stmt: &Stmt) -> Result<(), Error> {
        if let Stmt::Assign { pos, ref id, ref expr } = *stmt {
            let expr_ty = try!(self.tc_expr(expr));
            match self.symtable.get(id) {
                Some(&id_ty) => {
                    match (id_ty, expr_ty) {
                        (Type::Int, Type::Int) => Ok(()),
                        (Type::Int, Type::Float) =>
                            Err(Error::UnexpectedType { pos: pos, expected: Type::Int, actual: Type::Float }),
                        (Type::Float, Type::Int) => Ok(()),
                        (Type::Float, Type::Float) => Ok(()),
                    }
                }
                None => Err(Error::UndeclaredVariable(pos, id.clone()))
            }
        } else {
            Err(Error::GenericError)
        }
    }

    fn tc_stmt_read(&mut self, stmt: &Stmt) -> Result<(), Error> {
        if let Stmt::Read { pos, ref id } = *stmt {
            if self.symtable.contains_key(id) {
                Ok(())
            } else {
                Err(Error::UndeclaredVariable(pos, id.clone()))
            }
        } else {
            Err(Error::GenericError)
        }
    }

    fn tc_stmt_print(&mut self, stmt: &Stmt) -> Result<(), Error> {
        if let Stmt::Print { ref expr, .. } = *stmt {
            try!(self.tc_expr(expr));
            Ok(())
        } else {
            Err(Error::GenericError)
        }
    }

    fn tc_stmt_if(&mut self, stmt: &Stmt) -> Result<(), Error> {
        if let Stmt::If { pos, ref expr, ref then_stmts, ref else_stmts } = *stmt {
            let t = try!(self.tc_expr(expr));
            match t {
                Type::Int => {
                    try!(self.tc_stmts(then_stmts));
                    try!(self.tc_stmts(else_stmts));
                    Ok(())
                }
                _ => { Err(Error::UnexpectedType{ pos: pos, expected: Type::Int, actual: t }) }
            }
        } else {
            Err(Error::GenericError)
        }
    }

    fn tc_stmt_while(&mut self, stmt: &Stmt) -> Result<(), Error> {
        if let Stmt::While { pos, ref expr, ref stmts } = *stmt {
            let t = try!(self.tc_expr(expr));
            match t {
                Type::Int => { self.tc_stmts(stmts) }
                _ => { Err(Error::UnexpectedType { pos: pos, expected: Type::Int, actual: t }) }
            }
        } else {
            Err(Error::GenericError)
        }
    }

    fn tc_expr(&mut self, expr: &Expr) -> Result<Type, Error> {
        let ty = try!(match *expr {
            Expr::Int { .. } => Ok(Type::Int),
            Expr::Float { .. } => Ok(Type::Float),
            Expr::Id { .. } => self.tc_expr_id(expr),
            Expr::Negate { .. } => self.tc_expr_negate(expr),
            Expr::Binop { .. } => self.tc_expr_binop(expr),
        });

        let expr_copy = expr.clone();
        self.expr_table.insert(expr_copy, ty);

        Ok(ty)
    }

    fn tc_expr_id(&mut self, expr: &Expr) -> Result<Type, Error> {
        if let Expr::Id { pos, ref id } = *expr {
            match self.symtable.get(id) {
                Some(ty) => Ok(*ty),
                None => Err(Error::UndeclaredVariable(pos, id.clone())),
            }
        } else {
            Err(Error::GenericError)
        }
    }

    fn tc_expr_negate(&mut self, expr: &Expr) -> Result<Type, Error> {
        if let Expr::Negate { ref expr, .. } = *expr {
            self.tc_expr(expr)
        } else {
            Err(Error::GenericError)
        }
    }

    fn tc_expr_binop(&mut self, expr: &Expr) -> Result<Type, Error> {
        match *expr {
            Expr::Binop { ref expr1, ref expr2, .. } => {
                let t1 = try!(self.tc_expr(expr1));
                let t2 = try!(self.tc_expr(expr2));

                match (t1, t2) {
                    (Type::Int   , Type::Int)   => Ok(Type::Int),
                    (Type::Int   , Type::Float) => Ok(Type::Float),
                    (Type::Float , Type::Int)   => Ok(Type::Float),
                    (Type::Float , Type::Float) => Ok(Type::Float),
                }
            }
            _ => Err(Error::GenericError)
        }
    }
}
