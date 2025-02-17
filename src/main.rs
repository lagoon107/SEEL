use clap::Parser;
use args::Args;
use backend::interpreter;
use frontend::{grammar, helper::convert_string_to_static_str};

// Mod declarations
mod args;

fn main() -> anyhow::Result<()> {
    // Get command line args
    let args = Args::parse();

    // Get code from file
    let code = std::fs::read_to_string(args.file)?;

    // Show ast if option enabled in args
    if args.show_ast {
        // Create parser to parse ast
        let parser = grammar::ProgramParser::new();

        // Print parsed code as ast
        println!("Ast:\n\t{:#?}", parser.parse(unsafe { convert_string_to_static_str(code) })?);
    } else {
        // Evaluate code
        interpreter::run_code(&code)?;
    }

    // Return no errors
    Ok(())
}
