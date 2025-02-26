use args::parse_args;
use backend::{transpiler, writer::CWriter};
use frontend::{grammar, helper::convert_string_to_static_str};

/// Recieves SEEL code and returns it as C code.
fn run_code(code: &str) -> anyhow::Result<String> {
    let ast = grammar::ProgramParser::new().parse(unsafe { convert_string_to_static_str(code.to_owned()) })?;
    let c_ast = transpiler::Transpiler::new(&ast).run()?;

    CWriter::default().run(&c_ast)
}

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
    }

    // Print code
    println!("Transpiled code:");
    println!("{}", run_code(&code)?);

    // Return no errors
    Ok(())
}
