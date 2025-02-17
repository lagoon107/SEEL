/*!
    Contains `Visitor` trait and structs that implement `Visitor` trait.
*/
use crate::{parser::{Expr, Stmt}, runtime::{self, RuntimeVal}};

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
pub trait Visitor: Default {
    /// The type to return from functions.
    type Target;

    visitor_macros::gen_visitor_trait_fn!{
        // Program
        Vec<Stmt> => visit_program,

        // Statements
        Stmt => visit_stmt,
        Stmt => visit_stmt_print,
        Stmt => visit_stmt_assign,

        // Expressions
        Box<Expr> => visit_expr,
        Box<Expr> => visit_binary_expr,
        Box<Expr> => visit_num_expr,
        Box<Expr> => visit_str_expr,
        Box<Expr> => visit_ident_expr
    }
}

// Use helpful visitor macros
use visitor_macros::with_extract_enum_variant;

/// General visitor.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralVisitor {

}

impl Visitor for GeneralVisitor {
    type Target = anyhow::Result<runtime::RuntimeVal>;
    
    fn visit_program(&self, stmts: &Vec<Stmt>) -> Self::Target {
        // Visit all statements in program, evaluating each
        for stmt in stmts {
            self.visit_stmt(stmt);
        }

        // Return null runtime value
        Ok(RuntimeVal::Null)
    }

    fn visit_stmt(&self, stmt: &Stmt) -> Self::Target {
        match stmt {
            Stmt::Print(_) => self.visit_stmt_print(stmt),
            Stmt::Assign(_) => self.visit_stmt_assign(stmt),
            Stmt::Expr(e) => self.visit_expr(e),
        }
    }

    fn visit_stmt_print(&self, stmt: &Stmt) -> Self::Target {
        with_extract_enum_variant!(stmt, Stmt::Print(print_stmt), {
            // Get runtime value
            let runtime_assign_value = self.visit_expr(&print_stmt.value)?;
            
            // Print runtime value
            match runtime_assign_value {
                RuntimeVal::Null => println!("null"),
                RuntimeVal::Str(s) => println!("{}", s),
                RuntimeVal::Num(n) => println!("{}", n)
            };

            // Return null because it doesn't eval to anything
            return Ok(RuntimeVal::Null);
        });
    }

    fn visit_expr(&self, expr: &Box<Expr>) -> Self::Target {
        match **expr {
            Expr::Binary(_) => self.visit_binary_expr(expr),
            Expr::Str(_) => self.visit_str_expr(expr),
            Expr::Num(_) => self.visit_num_expr(expr),
            Expr::Ident(_) => self.visit_ident_expr(expr)
        }
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

            // Return left + right value
            with_extract_enum_variant!{runtime_lhs_val, RuntimeVal::Num(l), {
                with_extract_enum_variant!{runtime_rhs_val, RuntimeVal::Num(r), {
                    return Ok(RuntimeVal::Num(l + r));
                }}
            }}
        });
    }

    fn visit_num_expr(&self, expr: &Box<Expr>) -> Self::Target {
        with_extract_enum_variant!(**expr, Expr::Num(n), {
            return Ok(RuntimeVal::Num(n));
        });
    }
}
