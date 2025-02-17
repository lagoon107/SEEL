/*!
    Contains things related to the interpreter.
*/

use crate::{grammar, parser::*, visitor::{GeneralVisitor, Visitor}};

/// The interpreter that evaluates ast.
pub struct Interpreter {
    pub ast: Vec<Stmt>
}

impl Interpreter {
    /// Constructs a new interpreter with ast
    pub fn new(ast: Vec<Stmt>) -> Self {
        Self { ast }
    }

    /// Runs the interpreter, evaluating the ast.
    pub fn run(self) -> anyhow::Result<()> {
        // Run general visitor to evaluate everything
        _ = GeneralVisitor::default().visit_program(&self.ast)?;

        // Return no errors
        Ok(())
    }
}

/// Takes code, parses that code to an ast, and runs the ast.
pub fn run_code(code: &'static str) -> anyhow::Result<()> {
    Interpreter::new(grammar::ProgramParser::new().parse(code)?).run()
}

mod tests {
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
    }
}
