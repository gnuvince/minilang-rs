use parser::{Program, Stmt, Expr, Binop};
use types::Type;
use typecheck::{Symtable, Exprtable};

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
            Stmt::Read { ref id, .. } => {
                match self.symtable.get(id) {
                    Some(&Type::Int) => { println!("scanf(\"%d\", &{});", id); }
                    Some(&Type::Float) => { println!("scanf(\"%f\", &{});", id); }
                    None => { println!("/* read error */"); }
                }
            }
            Stmt::Print { ref expr, .. } => {
                let tmp = self.codegen_expr(expr);
                match self.exprtable.get(expr) {
                    Some(&Type::Int) => { println!("printf(\"%d\\n\", {});", tmp); }
                    Some(&Type::Float) => { println!("printf(\"%f\\n\", {});", tmp); }
                    None => { println!("/* read error */"); }
                }
            }
            Stmt::Assign { ref id, ref expr, .. } => {
                let tmp = self.codegen_expr(expr);
                println!("{} = {};", id, tmp);
            }
            Stmt::If { ref expr, ref then_stmts, ref else_stmts, .. } => {
                let tmp = self.codegen_expr(expr);
                println!("if ({}) {{", tmp);
                self.codegen_stmts(then_stmts);
                println!("}} else {{");
                self.codegen_stmts(else_stmts);
                println!("}}");
            }
            Stmt::While { ref expr, ref stmts, .. } => {
                let tmp = self.codegen_expr(expr);
                println!("while ({}) {{", tmp);
                self.codegen_stmts(stmts);
                println!("}}");
            }
        }
    }

    fn new_tmp(&mut self) -> String {
        self.tmp_counter += 1;
        let tmp = format!("tmp_{}", self.tmp_counter);
        tmp.to_string()
    }

    fn codegen_expr(&mut self, expr: &Expr) -> String {
        let tmp = self.new_tmp();
        let ty_str = match self.exprtable.get(expr) {
            Some(&Type::Int) => "int",
            Some(&Type::Float) => "float",
            None => "/* fail */",
        };
        match *expr {
            Expr::Int { value: v, .. } => { println!("{} {} = {};", ty_str, tmp, v); }
            Expr::Float { value: v, .. } => { println!("{} {} = {};", ty_str, tmp, v.0); }
            Expr::Id { ref id, .. } => { return id.clone(); }
            Expr::Negate { ref expr, .. } => {
                let id1 = self.codegen_expr(expr);
                println!("{} {} = -{};", ty_str, tmp, id1);
            }
            Expr::Binop { op, ref expr1, ref expr2, .. } => {
                let op_char = match op {
                    Binop::Add => '+',
                    Binop::Sub => '-',
                    Binop::Mul => '*',
                    Binop::Div => '/',
                };
                let id1 = self.codegen_expr(expr1);
                let id2 = self.codegen_expr(expr2);
                println!("{} {} = {} {} {};", ty_str, tmp, id1, op_char, id2);
            }
        }
        tmp
    }
}
