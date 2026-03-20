use std::path::Path;

use crate::{cli::args::parse_args, services::build::build_tree, utils::formatter::format_tree};

mod services;
mod cli;
mod utils;
fn main() {
    let args = parse_args();
    let path = Path::new(&args.path);
    let tree = build_tree(path).unwrap();
    let formatted_tree = format_tree(&tree);
    
    println!("{}", formatted_tree);
}
