/// A C lang statement.
#[derive(Clone, Debug, PartialEq)]
pub enum CStmt {
    /// An if statement.
    If { comparison: CComparison, code: Vec<CStmt> },

    /// A code block
    Block(Vec<CStmt>),
    // An assignment
    Assign(CAssignStmt),
    // An expression
    Expr(Box<CExpr>),
}

/// An C assignment statement.
/// 
/// Example: `auto x = 12;`
#[derive(Clone, Debug, PartialEq)]
pub struct CAssignStmt {
    pub name: String,
    pub value: Box<CExpr>
}

/// An C operator (eg. '+', '-').
#[derive(Clone, Debug, PartialEq)]
pub enum COp {
    Plus,
    Minus,
    Mult,
    Div
}

/// A C comparison operator (eg. '>', '<').
#[derive(Clone, Debug, PartialEq)]
pub enum CCompareOp {
    Greater,
    Less,
    NEqual,
    Equal,
    GreaterEqual,
    LessEqual,
}

/// A C expression.
#[derive(Clone, Debug, PartialEq)]
pub enum CExpr {
    // A fn call.
    FnCall(CFnCall),

    // Bools
    Bool(bool),

    /// A comparison expression.
    Comparison(CComparison),

    // Number expressions
    Binary(CBinaryExpr),
    Num(f64),

    Str(String),
    // An identifier
    Ident(String)
}

/// A C comparison (eg. "true == true")
#[derive(Clone, Debug, PartialEq)]
pub struct CComparison {
    pub lhs: Box<CExpr>,
    pub op: CCompareOp,
    pub rhs: Box<CExpr>
}

/// A C fn call (eg. "malloc()")
#[derive(Clone, Debug, PartialEq)]
pub struct CFnCall {
    pub name: String,
    pub args: Vec<Box<CExpr>>
}

/// A C Binary Expression.
/// 
/// Example: `1 + 2`.
#[derive(Clone, Debug, PartialEq)]
pub struct CBinaryExpr {
    pub lhs: Box<CExpr>,
    pub op: COp,
    pub rhs: Box<CExpr>
}
