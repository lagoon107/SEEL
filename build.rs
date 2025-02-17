pub fn main() {
    // Process frontend grammar file
    _ = lalrpop::Configuration::new()
        .process_file("frontend/src/grammar.lalrpop")
        .unwrap();
}
