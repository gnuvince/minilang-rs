use std::collections::HashMap;

use parser::{Program, Decl, Stmt, Expr};
use types::Type;
use error::Error;

type Symtable = HashMap<String, Type>;

pub fn tc_program(p: &Program) -> Result<(), Error> {
    let mut init_symtable = try!(tc_decls(&p.decls));
    tc_stmts(&p.stmts, &mut init_symtable)
}

fn tc_decls(decls: &Vec<Decl>) -> Result<Symtable, Error> {
    let mut symtable: Symtable = HashMap::new();
    for decl in decls {
        try!(tc_decl(&decl, &mut symtable));
    }
    println!("{:?}", symtable);
    Ok(symtable)
}

fn tc_decl(decl: &Decl, symtable: &mut Symtable) -> Result<(), Error> {
    if symtable.contains_key(&decl.id) {
        return Err(Error::GenericError);
    } else {
        symtable.insert(decl.id.clone(), decl.ty);
        Ok(())
    }
}

fn tc_stmts(stmts: &Vec<Stmt>, symtable: &mut Symtable) -> Result<(), Error> {
    for stmt in stmts {
        try!(tc_stmt(&stmt, symtable));
    }
    Ok(())
}

fn tc_stmt(stmt: &Stmt, symtable: &mut Symtable) -> Result<(), Error> {
    match *stmt {
        Stmt::Assign { .. } => tc_stmt_assign(stmt, symtable),
        Stmt::Read { .. } => tc_stmt_read(stmt, symtable),
        Stmt::Print { .. } => tc_stmt_print(stmt, symtable),
        Stmt::If { .. } => tc_stmt_if(stmt, symtable),
        Stmt::While { .. } => tc_stmt_while(stmt, symtable),
    }
}

fn tc_stmt_assign(stmt: &Stmt, symtable: &mut Symtable) -> Result<(), Error> {
    match *stmt {
        Stmt::Assign { pos, ref id, ref expr } => {
            let expr_ty = try!(tc_expr(expr, symtable));
            match symtable.get(id) {
                Some(&id_ty) => {
                    match (id_ty, expr_ty) {
                        (Type::Int, Type::Int) => Ok(()),
                        (Type::Int, Type::Float) => Err(Error::GenericError),
                        (Type::Float, Type::Int) => Ok(()),
                        (Type::Float, Type::Float) => Ok(()),
                    }
                }
                None => Err(Error::GenericError)
            }
        }
        _ => { Err(Error::GenericError) }
    }
}

fn tc_stmt_read(stmt: &Stmt, symtable: &Symtable) -> Result<(), Error> {
    match *stmt {
        Stmt::Read { pos, ref id } => {
            if symtable.contains_key(id) {
                Ok(())
            } else {
                Err(Error::GenericError)
            }
        }
        _ => { Err(Error::GenericError) }
    }
}

fn tc_stmt_print(stmt: &Stmt, symtable: &mut Symtable) -> Result<(), Error> {
    match *stmt {
        Stmt::Print { pos, ref expr } => {
            try!(tc_expr(expr, symtable));
            Ok(())
        }
        _ => Err(Error::GenericError)
    }
}

fn tc_stmt_if(stmt: &Stmt, symtable: &mut Symtable) -> Result<(), Error> {
    match *stmt {
        Stmt::If { pos, ref expr, ref then_stmts, ref else_stmts } => {
            let t = try!(tc_expr(expr, symtable));
            match t {
                Type::Int => {
                    try!(tc_stmts(then_stmts, symtable));
                    try!(tc_stmts(else_stmts, symtable));
                    Ok(())
                }
                _ => { Err(Error::GenericError) }
            }
        }
        _ => { Err(Error::GenericError) }
    }
}

fn tc_stmt_while(stmt: &Stmt, symtable: &mut Symtable) -> Result<(), Error> {
    match *stmt {
        Stmt::While { pos, ref expr, ref stmts } => {
            let t = try!(tc_expr(expr, symtable));
            match t {
                Type::Int => {
                    tc_stmts(stmts, symtable)
                }
                _ => {
                    Err(Error::GenericError)
                }
            }
        }
        _ => { Err(Error::GenericError) }
    }
}

fn tc_expr(expr: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    let ty = try!(match *expr {
        Expr::Int { .. } => Ok(Type::Int),
        Expr::Float { .. } => Ok(Type::Float),
        Expr::Id { .. } => tc_expr_id(expr, symtable),
        Expr::Negate { .. } => tc_expr_negate(expr, symtable),
        Expr::Add { .. } |
        Expr::Sub { .. } |
        Expr::Mul { .. } |
        Expr::Div { .. } => tc_expr_binop(expr, symtable),
    });
    // TODO(vfoley): insert expr -> ty into symtable
    Ok(ty)
}

fn tc_expr_id(expr: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    match *expr {
        Expr::Id { pos, ref id } => {
            match symtable.get(id) {
                Some(ty) => Ok(*ty),
                None => Err(Error::GenericError),
            }
        }
        _ => { Err(Error::GenericError) }
    }
}

fn tc_expr_negate(e: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    tc_expr(e, symtable)
}

fn tc_expr_binop(expr: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    match *expr {
        Expr::Add { pos, ref expr1, ref expr2 } |
        Expr::Sub { pos, ref expr1, ref expr2 } |
        Expr::Mul { pos, ref expr1, ref expr2 } |
        Expr::Div { pos, ref expr1, ref expr2 } => {
            let t1 = try!(tc_expr(expr1, symtable));
            let t2 = try!(tc_expr(expr2, symtable));

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
