/// A statement.
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    /// An if statement.
    If { comparison: Box<Expr>, code: Vec<Stmt> },

    /// A code block
    Block(Vec<Stmt>),

    // A statement that runs bash code
    Bash(String),
    // A print statement
    Print(PrintStmt),
    // An assignment
    Assign(AssignStmt),
    // An expression
    Expr(Box<Expr>),
}

/// A print statement.
/// 
/// Example: `print "Hello, world!";`
#[derive(Clone, Debug, PartialEq)]
pub struct PrintStmt {
    pub value: Box<Expr>
}

/// An assignment statement.
/// 
/// Example: `let x = 23;`
#[derive(Clone, Debug, PartialEq)]
pub struct AssignStmt {
    pub name: String,
    pub value: Box<Expr>
}

/// An operator (eg. '+', '-').
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Mult,
    Div
}

/// A comparison operator (eg. '>', '<').
#[derive(Clone, Debug, PartialEq)]
pub enum CompareOp {
    Greater,
    Less,
    Equal,
    GreaterEqual,
    LessEqual,
}

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // A read expression, with an optional prepended message.
    Read,

    /// A comparison expression.
    Comparison {
        lhs: Box<Expr>,
        op: CompareOp,
        rhs: Box<Expr>
    },

    // Number expressions
    Binary(BinaryExpr),
    Num(f64),

    Str(String),
    // An identifier
    Ident(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>
}

/// Returns code with comments (lines starting with '//') processed out.
pub fn filter_comments(code: &str) -> String {
    let mut lines = Vec::new();

    // Skips every line starting with "//"
    for line in code.lines() {
        if !line.starts_with("//") {
            lines.push(line)
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    // Use stuff
    use super::*;
    use crate::grammar;

    #[test]
    fn test_parser_if() {
        let code = "if 12 < 23 { print 23; }";
        let parser = grammar::StmtParser::new();

        assert_eq!(parser.parse(code).unwrap(), Stmt::If {
            comparison: Box::new(Expr::Comparison {
                lhs: Box::new(Expr::Num(12.0)),
                op: CompareOp::Less,
                rhs: Box::new(Expr::Num(23.0))
            }),
            code: vec![Stmt::Print(PrintStmt { value: Box::new(Expr::Num(23.0)) })]
        });
    }
    
    #[test]
    fn test_parser_code_block() {
        let code = "{ print 23; }";
        let parser = grammar::StmtParser::new();

        assert_eq!(parser.parse(code).unwrap(), Stmt::Block(
            vec![Stmt::Print(PrintStmt { value: Box::new(Expr::Num(23.0)) })]
        ));
    }

    #[test]
    fn test_parser_comparison() {
        let code = "2 + 2 > 3 * 2";
        let parser = grammar::ComparisonParser::new();

        if let Expr::Comparison { .. } = *parser.parse(code).unwrap() {

        } else {
            panic!("Comparison should result from ComparisonParser!");
        }

        assert_eq!(*parser.parse(code).unwrap(), Expr::Comparison {
            lhs: Box::new(Expr::Binary(BinaryExpr {
                lhs: Box::new(Expr::Num(2.0)),
                op: Op::Plus,
                rhs: Box::new(Expr::Num(2.0))
            })),
            op: CompareOp::Greater,
            rhs: Box::new(Expr::Binary(BinaryExpr {
                lhs: Box::new(Expr::Num(3.0)),
                op: Op::Mult,
                rhs: Box::new(Expr::Num(2.0))
            }))
        });
    }

    #[test]
    fn test_parser_bash_code() {
        let code = r#"'"echo Hello"';"#;
        let parser = grammar::StmtParser::new();

        assert_eq!(
            parser.parse(code).unwrap(),
            Stmt::Bash("echo Hello".to_string())
        );
    }

    #[test]
    fn test_parser_postfix_question_print() {
        let code = "1 + 2?";
        let parser = grammar::StmtParser::new();

        assert_eq!(
            parser.parse(code).unwrap(),
            Stmt::Print(PrintStmt {
                value: Box::new(
                    Expr::Binary(BinaryExpr {
                        lhs: Box::new(Expr::Num(1.0)),
                        op: Op::Plus,
                        rhs: Box::new(Expr::Num(2.0))
                    })
                )
            })
         );
    }

    #[test]
    fn test_parser_print() {
        let code = r#"print "Hello, world!";"#;
        let parser = grammar::StmtParser::new();

        assert_eq!(
            parser.parse(code).unwrap(),
            Stmt::Print(PrintStmt {
                value: Box::new(Expr::Str("Hello, world!".to_string()))
            })
        );
    }

    #[test]
    fn test_parser_assign() {
        let code = "x = 23;";
        let parser = grammar::StmtParser::new();

        assert_eq!(parser.parse(code), Ok(
            Stmt::Assign(AssignStmt {
                name: "x".to_string(),
                value: Box::new(Expr::Num(23.0))
            })
        ));
    }

    #[test]
    fn test_parser_read() {
        let code = "read";
        let parser = grammar::StmtParser::new();

        assert_eq!(parser.parse(code), Ok(Stmt::Expr(Box::new(Expr::Read))));
    }

    #[test]
    fn test_parser_binary() {
        let code = "23 + 42";
        let parser = grammar::StmtParser::new();

        assert_eq!(
            parser.parse(code),
            Ok(
                Stmt::Expr(
                    Box::new(
                        Expr::Binary(BinaryExpr {
                            lhs: Box::new(Expr::Num(23.0)),
                            op: Op::Plus,
                            rhs: Box::new(Expr::Num(42.0))
                        })
                    )
                )
            )
        );
    }

    #[test]
    fn test_parser_ident() {
        let code = "x";
        let parser = grammar::IdentParser::new();

        assert_eq!(parser.parse(code), Ok(Box::new(Expr::Ident("x".to_string()))));
    }

    #[test]
    fn test_parser_num() {
        let code = "23";
        let parser = grammar::StmtParser::new();

        assert_eq!(parser.parse(code), Ok(Stmt::Expr(Box::new(Expr::Num(23.0)))));
    }

    #[test]
    fn test_parser_str() {
        let code = r#""Hello, world!""#;
        let parser = grammar::StrParser::new();

        assert_eq!(parser.parse(code), Ok(Box::new(Expr::Str("Hello, world!".to_string()))));
    }
}
