/*!
    Contains things related to runtime
*/

use std::{cell::RefCell, collections::HashMap};

/// A runtime value.
#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeVal {
    Ident(String),
    Str(String),
    Num(f64),
    Null
}

/// A runtime environment containing items in current scope.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuntimeEnv {
    /// The optional parent of this environment
    pub parent: Option<Box<RuntimeEnv>>,
    /// The symbols in an environment, each containing a runtime value.
    pub symbols: RefCell<HashMap<String, RuntimeVal>>
}

impl RuntimeEnv {
    /// Constructs a new runtime environment with an optional parent and symbols.
    pub fn new(parent: Option<Box<RuntimeEnv>>, symbols: RefCell<HashMap<String, RuntimeVal>>) -> Self {
        Self { parent, symbols }
    }

    /// Constructs a new runtime environment with a parent and no vars.
    pub fn create_with_parent(parent: Box<RuntimeEnv>) -> Self {
        Self { parent: Some(parent), symbols: RefCell::default() }
    }

    /// Returns the value of a variable with `name`.
    pub fn get_var(&self, name: &str) -> Option<RuntimeVal> {
        if self.parent.is_none() {
            self.symbols.borrow().get(name).and_then(|s| Some((*s).clone()))
        } else {
            panic!("Getting variable from parent env is not yet implemented!")
        }
    }
    
    /// Returns true if a var with `name` exists in this env.
    pub fn var_exists(&self, name: &str) -> bool {
        self.symbols.borrow().contains_key(name)
    }
}
