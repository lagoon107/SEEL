use args::parse_args;
use backend::interpreter;
use frontend::{grammar, helper::convert_string_to_static_str};

fn main() -> anyhow::Result<()> {
    // Get command line args
    let args = parse_args();

    // Get code from file
    let code = std::fs::read_to_string(args.file)?;

    // Show ast if option enabled in args
    if args.show_ast {
        // Create parser to parse ast
        let parser = grammar::ProgramParser::new();

        // Print parsed code as ast
        println!("Ast:\n{:#?}", parser.parse(unsafe { convert_string_to_static_str(code.clone()) })?);
    } else {
        // Evaluate code
        _ = interpreter::run_code(&code)?;
    }

    // Return no errors
    Ok(())
}
