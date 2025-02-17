/*!
    Contains things related to the interpreter.
*/

use crate::{grammar, parser::{self, *}, runtime::RuntimeEnv, visitor::{GeneralVisitor, Visitor}};

/// The interpreter that evaluates ast.
#[derive(Clone, Debug, PartialEq)]
pub struct Interpreter {
    /// The ast to evaluate.
    pub ast: Vec<Stmt>,
}

impl Interpreter {
    /// Constructs a new interpreter with ast
    pub fn new(ast: Vec<Stmt>) -> Self {
        Self { ast }
    }

    /// Runs the interpreter, evaluating the ast.
    pub fn run(&self) -> anyhow::Result<()> {
        // Create runtime environment for general visitor
        let runtime_env = RuntimeEnv::default();

        // Run general visitor to evaluate everything
        _ = GeneralVisitor::new(Box::new(runtime_env)).visit_program(&self.ast)?;

        // Return no errors
        Ok(())
    }
}

/// Takes code, parses that code to an ast, and runs the ast.
pub fn run_code(code: &'static str) -> anyhow::Result<()> {
    Interpreter::new(grammar::ProgramParser::new().parse(parser::filter_comments(&code))?).run()
}

mod tests {
    use std::collections::HashMap;

    use crate::runtime::RuntimeVal;

    // Use outside scope
    use super::*;

    #[test]
    fn test_interp_full() {
        // Complete code file for interpreter to run
        let code = r#"
            print 23;
        "#;

        _ = run_code(code).unwrap();
    }

    #[test]
    fn test_interp_print() {
        assert!(run_code("print 23;").is_ok());
        assert!(run_code("print 23 * 42 + 56;").is_ok());
        assert!(run_code("print (23 + 42) * 56;").is_ok());
        _ = run_code("let x = 23; print x;").unwrap();
    }

    #[test]
    fn test_interp_var_decl() {
        // Create parser
        let parser = grammar::ProgramParser::new();
        // Create interpreter that parses assign statement
        let interp = Interpreter::new(parser.parse("let x = 23;").unwrap());

        // Run interpreter
        _ = interp.run().unwrap();
    }
}
