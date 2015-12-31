use std::collections::HashMap;

use parser::{Program, Decl, Stmt, Expr};
use types::Type;
use error::Error;

type Symtable = HashMap<Expr, Type>;

pub fn tc_program(p: &Program) -> Result<(), Error> {
    let mut init_symtable = try!(tc_decls(&p.decls));
    tc_stmts(&p.stmts, init_symtable)
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
            match symtable.get(&id_expr) {
                Some(_) => { return Err(Error::GenericError); }
                None => {
                    symtable.insert(id_expr, ty);
                }
            }
        }
    }
    Ok(())
}

fn tc_stmts(stmts: &Vec<Stmt>, mut symtable: Symtable) -> Result<(), Error> {
    return Ok(());
}
