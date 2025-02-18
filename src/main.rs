use args::parse_args;
use backend::interpreter;
use frontend::{grammar, helper::convert_string_to_static_str};

fn main() -> anyhow::Result<()> {
    // Get command line args
    let args = parse_args();

    // Get code from file
    let code = "H".to_string();

    // Show ast if option enabled in args
    if args.show_ast {
        // Create parser to parse ast
        let parser = grammar::ProgramParser::new();

        // Print parsed code as ast
        println!("Ast:\n\t{:#?}", parser.parse(unsafe { convert_string_to_static_str(code.clone()) })?);
    } else {
        // Evaluate code
        interpreter::run_code(&code)?;
    }

    // Return no errors
    Ok(())
}
