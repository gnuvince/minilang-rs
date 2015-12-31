use std::collections::HashMap;

use parser::{Program, Decl, Stmt, Expr};
use types::Type;
use error::Error;

type Symtable = HashMap<Expr, Type>;

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
    match *decl {
        Decl::Decl(ref id, ty) => {
            let id_expr = Expr::Id(id.clone());
            if symtable.contains_key(&id_expr) {
                return Err(Error::GenericError);
            } else {
                symtable.insert(id_expr, ty);
            }
        }
    }
    Ok(())
}

fn tc_stmts(stmts: &Vec<Stmt>, symtable: &mut Symtable) -> Result<(), Error> {
    for stmt in stmts {
        try!(tc_stmt(&stmt, symtable));
    }
    Ok(())
}

fn tc_stmt(stmt: &Stmt, symtable: &mut Symtable) -> Result<(), Error> {
    match *stmt {
        Stmt::Assign(ref id, ref e) => tc_stmt_assign(id, e, symtable),
        Stmt::Read(ref id) => tc_stmt_read(id, symtable),
        Stmt::Print(ref e) => tc_stmt_print(e, symtable),
        Stmt::If(ref e, ref then_stmts, ref else_stmts) => tc_stmt_if(e, then_stmts, else_stmts, symtable),
        Stmt::While(ref e, ref stmts) => tc_stmt_while(e, stmts, symtable),
    }
}

fn tc_stmt_assign(id: &String, expr: &Expr, symtable: &mut Symtable) -> Result<(), Error> {
    let expr_id = Expr::Id(id.clone());
    let expr_ty = try!(tc_expr(expr, symtable));
    match symtable.get(&expr_id) {
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

fn tc_stmt_read(id: &String, symtable: &Symtable) -> Result<(), Error> {
    let expr_id = Expr::Id(id.clone());
    if symtable.contains_key(&expr_id) {
        Ok(())
    } else {
        Err(Error::GenericError)
    }
}

fn tc_stmt_print(expr: &Expr, symtable: &mut Symtable) -> Result<(), Error> {
    try!(tc_expr(expr, symtable));
    Ok(())
}

fn tc_stmt_if(expr: &Expr, then_stmts: &Vec<Stmt>, else_stmts: &Vec<Stmt>, symtable: &mut Symtable) -> Result<(), Error> {
    let t = try!(tc_expr(expr, symtable));
    match t {
        Type::Int => {
            try!(tc_stmts(then_stmts, symtable));
            try!(tc_stmts(else_stmts, symtable));
            Ok(())
        }
        _ => {
            Err(Error::GenericError)
        }
    }
}

fn tc_stmt_while(expr: &Expr, stmts: &Vec<Stmt>, symtable: &mut Symtable) -> Result<(), Error> {
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

fn tc_expr(expr: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    let ty = try!(match *expr {
        Expr::Int(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),
        Expr::Id(_) => tc_expr_id(expr, symtable),
        Expr::Negate(ref e) => tc_expr_negate(e, symtable),
        Expr::Add(ref e1, ref e2) |
        Expr::Sub(ref e1, ref e2) |
        Expr::Mul(ref e1, ref e2) |
        Expr::Div(ref e1, ref e2) => tc_expr_binop(e1, e2, symtable),
    });
    // TODO(vfoley): insert expr -> ty into symtable
    Ok(ty)
}

fn tc_expr_id(expr: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    match symtable.get(expr) {
        Some(ty) => Ok(*ty),
        None => Err(Error::GenericError),
    }
}

fn tc_expr_negate(e: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    tc_expr(e, symtable)
}

fn tc_expr_binop(e1: &Expr, e2: &Expr, symtable: &mut Symtable) -> Result<Type, Error> {
    let t1 = try!(tc_expr(e1, symtable));
    let t2 = try!(tc_expr(e2, symtable));

    match (t1, t2) {
        (Type::Int   , Type::Int)   => Ok(Type::Int),
        (Type::Int   , Type::Float) => Ok(Type::Float),
        (Type::Float , Type::Int)   => Ok(Type::Float),
        (Type::Float , Type::Float) => Ok(Type::Float),
    }
}
