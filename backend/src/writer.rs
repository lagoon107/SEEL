use std::cell::RefCell;

use crate::c_ast::*;

/// Generates functions for Writer trait.
macro_rules! gen_writer_fn {
    ($( $param_ty: ty => $fn_name: ident ),*) => {
        $(
            fn $fn_name(&self, _item: $param_ty) -> anyhow::Result<()>;
        )*
    }
}

pub trait Writer {
    gen_writer_fn!{
        // Statements
        &CStmt => write_stmt,
        &CFnDef => write_stmt_fndef,
        &CExpr => write_stmt_return,
        &Vec<CStmt> => write_stmt_block,
        &CStmt => write_stmt_if,
        &CAssignStmt => write_stmt_assign,

        // Expressions
        &CExpr => write_expr,
        &CFnCall => write_expr_fncall,
        &CComparison => write_expr_comparison,
        &CBinaryExpr => write_expr_binary,
        &bool => write_expr_bool,
        &f64 => write_expr_num,
        &str => write_expr_str,
        &str => write_expr_ident,

        &COp => write_num_op,
        &CCompareOp => write_compare_op
    }
}

/// C++ code that is written to.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Code {
    /// The raw code that is written to.
    code: String
}

impl Code {
    /// Constructs a new Code object.
    pub fn new() -> Self {
        // For now, constructs object using `Default` trait
        Self::default()
    }

    /// Writes code to this object.
    pub fn write(&mut self, code: &str) {
        self.code.push_str(code);
    }
}

/// Translates C Ast to C++ code.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CWriter {
    code: RefCell<Code>
}

impl CWriter {
    /// Constructs a new CWriter object
    pub fn new() -> Self {
        // For now, constructs object using `Default` trait
        Self::default()
    }

    /// Returns code, that contains items that are essential for every SEEL program.
    fn custom_std() -> &'static str {
        "
        /// Simply prints something to the screen.
        #define PRINT(item) std::cout << (item) << std::endl;

        /// Returns terminal input (stdin).
        std::string read() {
            std::string input;
            getline(std::cin, input);

            return input;
        }
        "
    }

    /// Returns code, combined with essential code that must be placed before & after.
    fn combine_code(code: String) -> String {
        format!(
            "
            #include <iostream>
            #include <string>
            #include <stdio.h>
            #include <stdlib.h>

            {custom_std}

            int main() {{
                {code}
            }}
            ",
            custom_std = Self::custom_std()
        )
    }

    /// Converts CAst to C code.
    pub fn run(&self, ast: &Vec<CStmt>) -> anyhow::Result<String> {
        self.write_stmt_block(ast)?;

        // Return code emmited
        Ok(Self::combine_code(self.code.borrow().code.to_owned()))
    }
}

impl Writer for CWriter {
    fn write_stmt(&self, stmt: &CStmt) -> anyhow::Result<()> {
        match stmt {
            CStmt::FnDef(f) => self.write_stmt_fndef(f),
            CStmt::Return(val) => self.write_stmt_return(val),
            CStmt::Block(b) => self.write_stmt_block(b),
            CStmt::If { .. } => self.write_stmt_if(stmt),
            CStmt::Assign(c) => self.write_stmt_assign(c),
            CStmt::Expr(e) => self.write_expr(e)
        }
    }

    fn write_stmt_fndef(&self, stmt: &CFnDef) -> anyhow::Result<()> {
        // Write return type
        self.code.borrow_mut().write("auto ");
        // Write function identifier
        self.write_expr_ident(&stmt.name)?;
        self.write_expr_ident("=")?;

        // Write opening of param list
        self.code.borrow_mut().write("[]");
        self.code.borrow_mut().write("(");

        // Write function params
        let mut i = 1;
        for param in stmt.params.iter() {
            // Write param type
            self.write_expr_ident(format!("{} ", param.0).as_str())?;
            // Write param name
            self.write_expr_ident(&param.1)?;

            // Add comma if not at end of params list
            if i != stmt.params.len() {
                self.code.borrow_mut().write(", ");
            }

            i += 1;
        }

        // Write closing paren of param list
        self.code.borrow_mut().write(")");

        // Write body
        self.write_stmt_block(&stmt.code)?;

        Ok(())
    }

    fn write_stmt_return(&self, value: &CExpr) -> anyhow::Result<()> {
        self.code.borrow_mut().write("return ");

        // Write expr
        self.write_expr(value)?;

        // Write semicolon
        self.code.borrow_mut().write(";");

        Ok(())
    }

    fn write_stmt_block(&self, block: &Vec<CStmt>) -> anyhow::Result<()> {
        self.code.borrow_mut().write("{");

        for stmt in block {
            self.write_stmt(stmt)?;
            // Write semicolon and newline
            self.code.borrow_mut().write(";\n");
        }

        self.code.borrow_mut().write("}");

        Ok(())
    }

    fn write_stmt_if(&self, stmt: &CStmt) -> anyhow::Result<()> {
        // Statement must be If Statement here!
        match stmt {
            CStmt::If { comparison, code } => {
                // Write C if keyword
                self.code.borrow_mut().write("if ");

                // Write condition
                self.write_expr_comparison(comparison)?;

                // Write code block
                self.write_stmt_block(code)?;

                Ok(())
            },
            _ => todo!()
        }
    }

    fn write_stmt_assign(&self, stmt: &CAssignStmt) -> anyhow::Result<()> {
        // Write "auto" type for type inference
        self.code.borrow_mut().write("auto ");

        // Write var name and `=` token
        self.write_expr_ident(&stmt.name)?;
        self.code.borrow_mut().write(" = ");

        // Write var value
        self.write_expr(&stmt.value)?;

        Ok(())
    }

    fn write_expr(&self, expr: &CExpr) -> anyhow::Result<()> {
        match expr {
            CExpr::FnCall(cfncall) => self.write_expr_fncall(cfncall),
            CExpr::Binary(binary_expr) => self.write_expr_binary(binary_expr),
            CExpr::Comparison(c) => self.write_expr_comparison(c),
            CExpr::Bool(b) => self.write_expr_bool(b),
            CExpr::Ident(i) => self.write_expr_ident(i),
            CExpr::Num(n) => self.write_expr_num(n),
            CExpr::Str(s) => self.write_expr_str(s)
        }
    }

    fn write_expr_fncall(&self, cfncall: &CFnCall) -> anyhow::Result<()> {
        // Write function ident
        self.code.borrow_mut().write(format!("{}", cfncall.name).as_str());

        // Write function opening parne
        self.code.borrow_mut().write("(");
        
        // Write function args
        let mut i = 1;
        for arg in cfncall.args.iter() {
            self.write_expr(arg)?;

            // Add commas between args correctly
            if i != cfncall.args.len() {
                self.code.borrow_mut().write(", ");
            }

            i += 1;
        }

        // Write function closing paren
        self.code.borrow_mut().write(")");
        Ok(())
    }

    fn write_expr_binary(&self, b: &CBinaryExpr) -> anyhow::Result<()> {
        // Write opening parne
        self.code.borrow_mut().write("(");

        // Write lhs of comparison
        self.write_expr(&b.lhs)?;
        // Write op
        self.write_num_op(&b.op)?;
        // Write lhs
        self.write_expr(&b.rhs)?;

        // Write closing paren
        self.code.borrow_mut().write(")");

        Ok(())
    }

    fn write_num_op(&self, num_op: &COp) -> anyhow::Result<()> {
        self.code.borrow_mut().write(match num_op {
            COp::Plus => "+",
            COp::Minus => "-",
            COp::Mult => "*",
            COp::Div => "/"
        });

        Ok(())
    }

    fn write_compare_op(&self, compare_op: &CCompareOp) -> anyhow::Result<()> {
        self.code.borrow_mut().write(match compare_op {
            CCompareOp::NEqual => "!=",
            CCompareOp::Equal => "=",
            CCompareOp::Less => "<",
            CCompareOp::LessEqual => "<=",
            CCompareOp::Greater => ">",
            CCompareOp::GreaterEqual => ">="
        });

        Ok(())
    }

    fn write_expr_comparison(&self, c: &CComparison) -> anyhow::Result<()> {
        // Write opening paren
        self.code.borrow_mut().write("(");

        // Write lhs of comparison
        self.write_expr(&c.lhs)?;
        // Write op
        self.write_compare_op(&c.op)?;
        // Write lhs
        self.write_expr(&c.rhs)?;

        // Write closing paren
        self.code.borrow_mut().write(")");

        Ok(())
    }

    fn write_expr_ident(&self, i: &str) -> anyhow::Result<()> {
        self.code.borrow_mut().write(format!("{}", i).as_str());

        Ok(())
    }
    
    fn write_expr_bool(&self, b: &bool) -> anyhow::Result<()> {
        self.code.borrow_mut().write(format!("{}", b).as_str());

        Ok(())
    }

    fn write_expr_str(&self, s: &str) -> anyhow::Result<()> {
        self.code.borrow_mut().write(format!(r#""{}""#, s).as_str());

        Ok(())
    }

    fn write_expr_num(&self, n: &f64) -> anyhow::Result<()> {
        self.code.borrow_mut().write(format!("{}", n).as_str());

        Ok(())
    }
}
