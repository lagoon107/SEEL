use lalrpop_util::lalrpop_mod;
use clap::Parser;
use args::Args;

// Add lalrpop parsed grammer file to project
lalrpop_mod!(pub grammar);

// Mod declarations
mod parser;
mod interpreter;
mod visitor;
mod runtime;
mod args;

fn main() -> anyhow::Result<()> {
    // Create sample code
    let code = r#"print "Hello, world!";"#;

    // Get command line args
    let args = Args::parse();

    // Show ast if option enabled in args
    if args.show_ast {
        // Create parser to parse ast
        let parser = grammar::ProgramParser::new();

        // Print parsed code as ast
        println!("{:?}", parser.parse(code)?);
    }

    // Return no errors
    Ok(())
}
