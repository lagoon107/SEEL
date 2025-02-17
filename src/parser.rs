/// A statement.
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    // A print statement
    Print(PrintStmt),
    // An assignment
    Assign(AssignStmt),
    // An expression
    Expr(Box<Expr>)
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

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
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

#[cfg(test)]
mod tests {
    // Use stuff
    use super::*;
    use crate::grammar;

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
        let code = "let x = 23;";
        let parser = grammar::StmtParser::new();

        assert_eq!(parser.parse(code), Ok(
            Stmt::Assign(AssignStmt {
                name: "x".to_string(),
                value: Box::new(Expr::Num(23.0))
            })
        ));
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
