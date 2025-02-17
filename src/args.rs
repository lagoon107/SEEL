/*!
    Contains items related to program args.
*/
use clap::Parser;

#[derive(Debug, Clone, PartialEq, Parser)]
pub struct Args {
    /// Enables or disables ast output.
    #[clap(short, long)]
    pub show_ast: bool,

    /// File to parse.
    pub file: String
}
