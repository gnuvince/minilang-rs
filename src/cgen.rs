use std::fmt;
use std::fmt::Display;

use ast::*;
use types::Type;
use typecheck::{Symtable, Exprtable};

enum ExprReturn {
    Id(String),
    Int(i64),
    Float(f64),
}

impl Display for ExprReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExprReturn::Id(ref s) => write!(f, "{}", s.clone()),
            ExprReturn::Int(n) => write!(f, "{}", n),
            ExprReturn::Float(fl) => write!(f, "{}", fl),
        }
    }
}

pub struct Generator<'a> {
    tmp_counter: i32,
    symtable: &'a Symtable,
    exprtable: &'a Exprtable,
}

pub fn codegen(program: &Program, symtable: &Symtable, exprtable: &Exprtable) {
    let mut generator = Generator {
        tmp_counter: 0,
        symtable: symtable,
        exprtable: exprtable,
    };
    generator.codegen_program(program);
}

impl<'a> Generator<'a> {
    fn codegen_program(&mut self, program: &Program) {
        println!("#include <stdio.h>");
        println!("int main(void) {{");

        self.codegen_decls();
        self.codegen_stmts(&program.stmts);

        println!("}}");
    }

    fn codegen_decls(&mut self) {
        for (id, ty) in self.symtable {
            match *ty {
                Type::Int => { println!("int {};", id); }
                Type::Float => { println!("float {};", id); }
            }
        }
    }

    fn codegen_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.codegen_stmt(&stmt);
        }
    }

    fn codegen_stmt(&mut self, stmt: &Stmt) {
        match *stmt {
            Stmt::Read(ref stmt_) => {
                match self.symtable.get(&stmt_.id) {
                    Some(ty) => { println!("scanf(\"%{}\", &{});", ty.format_letter(), stmt_.id); }
                    None => { println!("/* read error */"); }
                }
            }
            Stmt::Print(ref stmt_) => {
                let expr_ret = self.codegen_expr(&stmt_.expr);
                match self.exprtable.get(&stmt_.expr) {
                    Some(ty) => { println!("printf(\"%{}\\n\", {});", ty.format_letter(), expr_ret); }
                    None => { println!("/* read error */"); }
                }
            }
            Stmt::Assign(ref stmt_) => {
                let expr_ret = self.codegen_expr(&stmt_.expr);
                println!("{} = {};", stmt_.id, expr_ret);
            }
            Stmt::If(ref stmt_) => {
                let expr_ret = self.codegen_expr(&stmt_.expr);
                println!("if ({}) {{", expr_ret);
                self.codegen_stmts(&stmt_.then_stmts);
                println!("}} else {{");
                self.codegen_stmts(&stmt_.else_stmts);
                println!("}}");
            }
            Stmt::While(ref stmt_) => {
                let expr_ret = self.codegen_expr(&stmt_.expr);
                println!("while ({}) {{", expr_ret);
                self.codegen_stmts(&stmt_.stmts);
                println!("}}");
            }
        }
    }

    fn new_tmp(&mut self) -> String {
        self.tmp_counter += 1;
        let tmp = format!("tmp_{}", self.tmp_counter);
        tmp.to_string()
    }

    fn codegen_expr(&mut self, expr: &Expr) -> ExprReturn {
        let ty_str = match self.exprtable.get(expr) {
            Some(&Type::Int) => "int",
            Some(&Type::Float) => "float",
            None => "/* fail */",
        };

        match *expr {
            Expr::Int(ref expr_) => { ExprReturn::Int(expr_.value) }
            Expr::Float(ref expr_) => { ExprReturn::Float(expr_.value.0) }
            Expr::Id(ref expr_) => { ExprReturn::Id(expr_.id.clone()) }
            Expr::Negate(ref expr_) => {
                let tmp = self.new_tmp();
                let expr_ret = self.codegen_expr(&expr_.expr);
                println!("{} {} = -{};", ty_str, tmp, expr_ret);
                ExprReturn::Id(tmp)
            }
            Expr::Binop(ref expr_) => {
                let op_char = match expr_.op {
                    Binop::Add => '+',
                    Binop::Sub => '-',
                    Binop::Mul => '*',
                    Binop::Div => '/',
                };
                let tmp = self.new_tmp();
                let expr_ret1 = self.codegen_expr(&expr_.expr1);
                let expr_ret2 = self.codegen_expr(&expr_.expr2);
                println!("{} {} = {} {} {};", ty_str, tmp, expr_ret1, op_char, expr_ret2);
                ExprReturn::Id(tmp)
            }
        }
    }
}
