use std::mem;
use std::hash::{Hash, Hasher};

use types::Type;
use pos::Pos;

/* newtype to allow putting an ExprFloat node in a hashmap */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float(pub f64);

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
pub struct StmtRead {
    pub pos: Pos,
    pub id: String
}

#[derive(Debug)]
pub struct StmtPrint {
    pub pos: Pos,
    pub expr: Expr
}

#[derive(Debug)]
pub struct StmtAssign {
    pub pos: Pos,
    pub id: String,
    pub expr: Expr
}

#[derive(Debug)]
pub struct StmtIf {
    pub pos: Pos,
    pub expr: Expr,
    pub then_stmts: Vec<Stmt>,
    pub else_stmts: Vec<Stmt>
}

#[derive(Debug)]
pub struct StmtWhile {
    pub pos: Pos,
    pub expr: Expr,
    pub stmts: Vec<Stmt>
}

#[derive(Debug)]
pub enum Stmt {
    Read(StmtRead),
    Print(StmtPrint),
    Assign(StmtAssign),
    If(StmtIf),
    While(StmtWhile),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExprId {
    pub pos: Pos,
    pub id: String
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExprInt {
    pub pos: Pos,
    pub value: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExprFloat {
    pub pos: Pos,
    pub value: Float,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExprNegate {
    pub pos: Pos,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExprBinop {
    pub pos: Pos,
    pub op: Binop,
    pub expr1: Box<Expr>,
    pub expr2: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expr {
    Id(ExprId),
    Int(ExprInt),
    Float(ExprFloat),
    Negate(ExprNegate),
    Binop(ExprBinop),
}

#[derive(Debug)]
pub struct Program {
    pub decls: Vec<Decl>,
    pub stmts: Vec<Stmt>,
}
