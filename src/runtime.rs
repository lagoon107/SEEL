/*!
    Contains things related to runtime
*/

/// A runtime value.
#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeVal {
    Str(String),
    Num(f64),
    Null
}
