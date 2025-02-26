use std::{io::Write, process::Command};

use args::parse_args;
use backend::{transpiler, writer::CWriter};
use frontend::{grammar, helper::convert_string_to_static_str};

/// Recieves SEEL code and returns it as C code.
fn run_code(code: &str) -> anyhow::Result<String> {
    let mut lines_with_no_comments = Vec::new();

    // Filter any comments
    for line in code.lines() {
        if !line.starts_with("//") {
            lines_with_no_comments.push(line)
        }
    }

    // Get processed code
    let processed_code = lines_with_no_comments.join("\n");

    let ast = grammar::ProgramParser::new().parse(unsafe { convert_string_to_static_str(processed_code) })?;
    let c_ast = transpiler::Transpiler::new(&ast).run()?;

    CWriter::default().run(&c_ast)
}

/// Saves code to specified file_path.
fn save_code(code: &str, file_folder: &str, file_path: &str) -> anyhow::Result<()> {
    // Create dir for transpiled files
    if !std::fs::exists(file_folder)? {
        std::fs::create_dir(file_folder)?;
    }

    // Save code to file
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(code.as_bytes())?;
    file.flush()?;

    // Format file with "clang-format"
    let clang_format_output = Command::new("clang-format")
        .arg(file_path)
        .output()?;

    // Ensure clang-format no errors
    if !clang_format_output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8(clang_format_output.stderr)?);
    }

    // Save clang file output to file
    let mut formatted_file = std::fs::File::create(file_path)?;
    formatted_file.write_all(&clang_format_output.stdout)?;

    Ok(())
}

/// Compiles code with clang++ to executable.
fn compile_code(code_path: &str) -> anyhow::Result<()> {
    // compile code with clang++
    let clang_output = Command::new("clang++")
        .arg(code_path)
        .arg("-o")
        .arg(code_path.replace(".cpp", ".exe"))
        .output()?;

    // Print any clang++ errors
    if !clang_output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8(clang_output.stderr)?);
    }

    Ok(())
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

    // Get C code
    let c_code = run_code(&code)?;

    // Save code to file
    save_code(&c_code, "./test/transpiled", "./test/transpiled/cpp_code.cpp")?;

    // Compile code with clang++
    compile_code("./test/transpiled/cpp_code.cpp")?;

    // Return no errors
    Ok(())
}
