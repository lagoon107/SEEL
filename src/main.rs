use lalrpop_util::lalrpop_mod;

// Add lalrpop parsed grammer file to project
lalrpop_mod!(pub grammar);

// Mod declarations
mod parser;
mod interpreter;
mod visitor;
mod runtime;

fn main() {
    println!("Hello, world!");
}
