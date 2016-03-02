use types::Type;
use pos::Pos;

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

#[derive(Debug)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct ExprId {
    pub id: String
}

#[derive(Debug)]
pub struct ExprInt {
    pub value: i64,
}

#[derive(Debug)]
pub struct ExprFloat {
    pub value: f64,
}

#[derive(Debug)]
pub struct ExprString {
    pub value: String,
}

#[derive(Debug)]
pub struct ExprNegate {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct ExprBinop {
    pub op: Binop,
    pub expr1: Box<Expr>,
    pub expr2: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr_ {
    Id(ExprId),
    Int(ExprInt),
    Float(ExprFloat),
    String(ExprString),
    Negate(ExprNegate),
    Binop(ExprBinop),
}


#[derive(Debug)]
pub struct Expr {
    pub pos: Pos,
    pub node_id: u64,
    pub expr: Expr_,
}

#[derive(Debug)]
pub struct Program {
    pub decls: Vec<Decl>,
    pub stmts: Vec<Stmt>,
}
