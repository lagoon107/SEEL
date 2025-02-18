pub fn main() {
    // Process frontend grammar file
    _ = lalrpop::Configuration::new()
        .set_out_dir("")
        .process_file("frontend/src/grammar.lalrpop")
        .unwrap();
}
