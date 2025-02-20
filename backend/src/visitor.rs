/*!
    Contains `Visitor` trait and structs that implement `Visitor` trait.
*/
use std::{ffi::CString, io::prelude::*};
use frontend::parser::{Expr, Op, Stmt};
use crate::runtime::{RuntimeEnv, RuntimeVal};

/// Macros that `Visitor` trait uses.
mod visitor_macros {
    /// Generates visitor trait functions.  
    /// This should only be used by `Visitor` trait internally.
    macro_rules! gen_visitor_trait_fn {
        ($( $fn_name:ident ),*) => {
            $(
                fn $fn_name(&self) -> Self::Target {
                    todo!()
                }
            )*
        };

        ($( $item_t:ty => $fn_name:ident ),*) => {
            $(
                fn $fn_name(&self, _item: &$item_t) -> Self::Target {
                    todo!()
                }
            )*
        }
    }

    /// Extracts a enum variant from a enum and performs code with it.
    /// 
    /// This can only be called from a function that returns `anyhow::Result`.
    macro_rules! with_extract_enum_variant {
        ($enum:expr, $variant:pat, $code:block) => {
            if let $variant = $enum {
                $code
            } else {
                // Return anyhow error out of enclosing function
                use anyhow::anyhow;
                return Err(anyhow!("Incorrect type requested for enum extraction!"));
            }
        }
    }

    // Export macros to current module
    pub(super) use gen_visitor_trait_fn;
    pub(super) use with_extract_enum_variant;
}

/// A trait that allows a struct to visit parser AST and return values.
pub trait Visitor {
    /// The type to return from functions.
    type Target;

    visitor_macros::gen_visitor_trait_fn!{
        // Program
        Vec<Stmt> => visit_program,

        // Statements
        Stmt => visit_stmt,
        Stmt => visit_stmt_print,
        Stmt => visit_stmt_assign,
        Stmt => visit_bash_code_stmt,

        // Expressions
        Box<Expr> => visit_expr,
        Box<Expr> => visit_read_expr,
        Box<Expr> => visit_binary_expr,
        Box<Expr> => visit_num_expr,
        Box<Expr> => visit_str_expr,
        Box<Expr> => visit_ident_expr
    }
}

// Use helpful visitor macros
use visitor_macros::with_extract_enum_variant;

/// General visitor.
#[derive(Clone, Debug, PartialEq)]
pub struct GeneralVisitor {
    env: Box<RuntimeEnv>
}

impl GeneralVisitor {
    /// Return a new general visitor, given a runtime environment.
    pub fn new(env: Box<RuntimeEnv>) -> Self {
        Self { env }
    }
}

/// Prints the value of a `RuntimeVal`.
fn print_runtime_val(env: &RuntimeEnv, runtime_val: &RuntimeVal) {
    // Print runtime value
    match runtime_val {
        RuntimeVal::Ident(name) => {
            // Call function recursively to print value of ident
            print_runtime_val(env,&env.get_var(&name).unwrap())
        },
        RuntimeVal::Str(s) => println!("{}", s),
        RuntimeVal::Num(n) => println!("{}", n),
        RuntimeVal::Null => println!("null")
    };
}

impl Visitor for GeneralVisitor {
    type Target = anyhow::Result<RuntimeVal>;
    
    fn visit_program(&self, stmts: &Vec<Stmt>) -> Self::Target {
        // Visit all statements in program, evaluating each
        for stmt in stmts {
            _ = self.visit_stmt(stmt)?;
        }

        // Return null runtime value
        Ok(RuntimeVal::Null)
    }

    fn visit_stmt(&self, stmt: &Stmt) -> Self::Target {
        match stmt {
            Stmt::Bash(_) => self.visit_bash_code_stmt(stmt),
            Stmt::Print(_) => self.visit_stmt_print(stmt),
            Stmt::Assign(_) => self.visit_stmt_assign(stmt),
            Stmt::Expr(e) => self.visit_expr(e),
        }
    }

    fn visit_bash_code_stmt(&self, stmt: &Stmt) -> Self::Target {
        with_extract_enum_variant!(stmt, Stmt::Bash(code), {
            // Call system() func from libc
            unsafe { libc::system(CString::new(code.as_bytes())?.as_ptr()) }
        });
        
        // Return null because this is a statement
        Ok(RuntimeVal::Null)
    }

    fn visit_stmt_print(&self, stmt: &Stmt) -> Self::Target {
        with_extract_enum_variant!(stmt, Stmt::Print(print_stmt), {
            // Get runtime value
            let runtime_assign_value = self.visit_expr(&print_stmt.value)?;
            
            // Print runtime value
            print_runtime_val(&self.env, &runtime_assign_value);

            // Return null because it doesn't eval to anything
            return Ok(RuntimeVal::Null);
        });
    }

    fn visit_stmt_assign(&self, stmt: &Stmt) -> Self::Target {
        with_extract_enum_variant!(stmt, Stmt::Assign(a), {
            // Insert var into runtime environment
            self.env.symbols.borrow_mut().insert(a.name.clone(), self.visit_expr(&a.value)?);
        });

        // Return null because this doesn't eval to anything
        return Ok(RuntimeVal::Null);
    }

    fn visit_expr(&self, expr: &Box<Expr>) -> Self::Target {
        match **expr {
            Expr::Read => self.visit_read_expr(expr),
            Expr::Binary(_) => self.visit_binary_expr(expr),
            Expr::Str(_) => self.visit_str_expr(expr),
            Expr::Num(_) => self.visit_num_expr(expr),
            Expr::Ident(_) => self.visit_ident_expr(expr)
        }
    }

    fn visit_read_expr(&self, _expr: &Box<Expr>) -> Self::Target {
        // Get terminal input
        let mut terminal_input = String::new();
        _ = std::io::stdin().lock().read_line(&mut terminal_input)?;

        // Return terminal input as runtime string
        Ok(RuntimeVal::Str(terminal_input))
    }

    fn visit_str_expr(&self, expr: &Box<Expr>) -> Self::Target {
        with_extract_enum_variant!((**expr).clone(), Expr::Str(s), {
            return Ok(RuntimeVal::Str(s));
        });
    }

    fn visit_binary_expr(&self, expr: &Box<Expr>) -> Self::Target {
        with_extract_enum_variant!((**expr).clone(), Expr::Binary(b), {
            // Evaluate left and right side of binary expr
            let runtime_lhs_val = self.visit_expr(&b.lhs)?;
            let runtime_rhs_val = self.visit_expr(&b.rhs)?;

            // Return value of binary expression
            with_extract_enum_variant!{runtime_lhs_val, RuntimeVal::Num(l), {
                with_extract_enum_variant!{runtime_rhs_val, RuntimeVal::Num(r), {
                    return Ok(RuntimeVal::Num(match b.op {
                        Op::Plus => l + r,
                        Op::Minus => l - r,
                        Op::Mult => l * r,
                        Op::Div => l / r
                    }));
                }
            }}}
        });
    }

    fn visit_num_expr(&self, expr: &Box<Expr>) -> Self::Target {
        with_extract_enum_variant!(**expr, Expr::Num(n), {
            return Ok(RuntimeVal::Num(n));
        });
    }

    fn visit_ident_expr(&self, expr: &Box<Expr>) -> Self::Target {
        with_extract_enum_variant!((**expr).clone(), Expr::Ident(i), {
            // Return runtime type representing ident
            return Ok(RuntimeVal::Ident(i.to_owned()));
        });
    }
}
