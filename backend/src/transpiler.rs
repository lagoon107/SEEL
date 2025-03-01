use frontend::parser::*;

use crate::c_ast::*;

/// A C statement anyhow result.
type CStmtRes = anyhow::Result<CStmt>;

/// A C expression anyhow result.
type CExprRes = anyhow::Result<Box<CExpr>>;

/// Wraps a CExpr in a box and then `Ok()`
macro_rules! c_expr_res {
    ($cexpr: expr) => {
        Ok(Box::new($cexpr))
    };
}

pub struct Transpiler<'a> {
    ast: &'a Vec<Stmt>
}

/// Generates owned values from borrowed values that are inside a box

impl<'a> Transpiler<'a> {
    /// Constructs a new transpiler with given ast.
    pub fn new(ast: &'a Vec<Stmt>) -> Self {
        Self { ast }
    }

    /// Runs itself, transpiling SEEL ast to C ast.
    pub fn run(&self) -> anyhow::Result<Vec<CStmt>> {
        self.transpile_program(self.ast)
    }

    pub fn transpile_program(&self, program: &Vec<Stmt>) -> anyhow::Result<Vec<CStmt>> {
        // Create vec for c ast
        let mut c_ast = Vec::new();

        for stmt in program {
            c_ast.push(self.transpile_stmt(stmt)?);
        }

        Ok(c_ast)
    }

    pub fn transpile_stmt(&self, stmt: &Stmt) -> CStmtRes {
        Ok(match stmt {
            // The "main()" function
            Stmt::EntryDef(e) => self.transpile_stmt_entrydef(e)?,
            // Regular function definition
            Stmt::FnDef(f) => self.transpile_stmt_fndef(f)?,
            Stmt::Return(val) => self.transpile_stmt_return(val)?,
            Stmt::Bash(b) => self.transpile_stmt_bash(b.to_owned())?,
            Stmt::If { comparison, code } => self.transpile_stmt_if(comparison, code)?,
            Stmt::Print(p) => self.transpile_stmt_print(p)?,
            Stmt::Block(b) => self.transpile_stmt_block(b)?,
            Stmt::Assign(a) => self.transpile_stmt_assign(a)?,
            Stmt::Expr(e) => CStmt::Expr(self.transpile_expr(e)?),
        })
    }

    pub fn transpile_stmt_return(&self, value: &Expr) -> CStmtRes {
        Ok(CStmt::Return(self.transpile_expr(value)?))
    }

    pub fn transpile_stmt_entrydef(&self, entrydef: &EntryDef) -> CStmtRes {
        Ok(CStmt::FnDef(CFnDef {
            name: "main".to_string(),
            params: vec![],
            code: self.transpile_program(&entrydef.code)?,
            return_t: "int".to_string()
        }))
    }

    pub fn transpile_stmt_fndef(&self, fndef: &FnDef) -> CStmtRes {
        Ok(CStmt::FnDef(CFnDef {
            name: fndef.name.to_owned(),
            params: fndef.params.to_owned().into_iter().map(|i| ("auto".to_string(), i)).collect(),
            code: self.transpile_program(&fndef.code)?,
            return_t: "auto".to_string()
        }))
    }

    pub fn transpile_stmt_bash(&self, code: String) -> CStmtRes {
        // Bash code transpiles to `system()` fn call from c lib
        Ok(CStmt::Expr(Box::new(CExpr::FnCall(CFnCall {
            name: "system".to_string(),
            args: vec![self.transpile_expr_str(code)?]
        }))))
    }

    pub fn transpile_stmt_if(&self, c: &Box<Expr>, code: &Vec<Stmt>) -> CStmtRes {
        Ok(CStmt::If {
            // comparison SHOULD be of type `Expr::Comparison`
            comparison: match (**c).clone() {
                Expr::Comparison { lhs, op, rhs } => {
                    CComparison {
                        lhs: self.transpile_expr(&lhs)?,
                        op: self.transpile_compare_op(&op),
                        rhs: self.transpile_expr(&rhs)?
                    }
                },
                _ => panic!("Non-condition in if statement!")
            },
            code: match self.transpile_stmt_block(code)? {
                CStmt::Block(b) => b,
                _ => panic!("transpile_stmt_block() didn't return a code block!")
            } })
    }

    pub fn transpile_stmt_print(&self, p: &PrintStmt) -> CStmtRes {
        Ok(CStmt::Expr(Box::new(CExpr::FnCall(CFnCall {
            name: "PRINT".to_string(),
            args: vec![self.transpile_expr(&p.value)?]
        }))))
    }

    pub fn transpile_stmt_block(&self, b: &Vec<Stmt>) -> CStmtRes {
        let mut c_statements = Vec::new();

        for stmt in b {
            c_statements.push(self.transpile_stmt(stmt)?);
        }

        Ok(CStmt::Block(c_statements))
    }

    pub fn transpile_stmt_assign(&self, stmt: &AssignStmt) -> CStmtRes {
        Ok(CStmt::Assign(CAssignStmt {
            name: stmt.name.to_owned(),
            value: self.transpile_expr(&stmt.value)?
        }))
    }

    pub fn transpile_expr(&self, expr: &Expr) -> CExprRes {
        match expr {
            Expr::FnCall(f) => self.transpile_expr_fncall(f),
            Expr::Binary(b) => self.transpile_expr_binary(b),
            Expr::Read => self.transpile_expr_read(),
            Expr::Comparison { lhs, op, rhs } => self.transpile_expr_compare(lhs, op, rhs),
            Expr::Str(s) => self.transpile_expr_str(s.to_owned()),
            Expr::Num(n) => self.transpile_expr_num(*n),
            Expr::Bool(b) => self.transpile_expr_bool(*b),
            Expr::Ident(i) => self.transpile_expr_ident(i.to_owned()),
        }
    }

    pub fn transpile_expr_fncall(&self, f: &FnCall) -> CExprRes {
        c_expr_res!(CExpr::FnCall(CFnCall {
            name: f.name.to_owned(),
            args: {
                // Transpile each expression
                let mut transpiled_expressions = Vec::new();

                for expr in f.args.iter() {
                    transpiled_expressions.push(self.transpile_expr(expr)?);
                }

                transpiled_expressions
            }
        }))
    }

    pub fn transpile_expr_binary(&self, b: &BinaryExpr) -> CExprRes {
        c_expr_res!(CExpr::Binary(CBinaryExpr {
            lhs: self.transpile_expr(&b.lhs)?,
            op: self.transpile_num_op(b.op.to_owned()),
            rhs: self.transpile_expr(&b.rhs)?
        }))
    }

    pub fn transpile_num_op(&self, op: Op) -> COp {
        match op {
            Op::Plus => COp::Plus,
            Op::Minus => COp::Minus,
            Op::Mult => COp::Mult,
            Op::Div => COp::Div
        }
    }

    pub fn transpile_expr_read(&self) -> CExprRes {
        // "read" in our language will call a specially defined "read()" that we define later
        c_expr_res!(CExpr::FnCall(CFnCall {
            name: "read".to_string(),
            args: Vec::new()
        }))
    }

    pub fn transpile_expr_compare(&self, lhs: &Box<Expr>, op: &CompareOp, rhs: &Box<Expr>) -> CExprRes {
        c_expr_res!(CExpr::Comparison(CComparison {
            lhs: self.transpile_expr(lhs)?,
            op: self.transpile_compare_op(op),
            rhs: self.transpile_expr(rhs) ?
        }))
    }

    pub fn transpile_compare_op(&self, op: &CompareOp) -> CCompareOp {
        match op {
            CompareOp::NEqual => CCompareOp::NEqual,
            CompareOp::Equal => CCompareOp::Equal,
            CompareOp::Greater => CCompareOp::Greater,
            CompareOp::GreaterEqual => CCompareOp::GreaterEqual,
            CompareOp::Less => CCompareOp::Less,
            CompareOp::LessEqual => CCompareOp::LessEqual
        }
    }

    pub fn transpile_expr_str(&self, str: String) -> CExprRes {
        c_expr_res!(CExpr::Str(str))
    }

    pub fn transpile_expr_num(&self, num: f64) -> CExprRes {
        c_expr_res!(CExpr::Num(num))
    }

    pub fn transpile_expr_bool(&self, bool: bool) -> CExprRes {
        c_expr_res!(CExpr::Bool(bool))
    }

    pub fn transpile_expr_ident(&self, i: String) -> CExprRes {
        c_expr_res!(CExpr::Ident(i))
    }
}
