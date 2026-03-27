use std::{path::Path};

use crate::{cli::args::parse_args};
mod services;
mod cli;
mod utils;
fn main() {
    let args = parse_args();
    let path = Path::new(&args.path);
    // let tree = build_tree(path);
    // let formatted_tree = format_tree(&tree);

    let current_time = std::time::Instant::now();
    let results = services::build::build(path.to_path_buf());
    let elapsed = current_time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);

    for _ in results {
        // println!(
        //     "{:?} {} {} ({})",
        //     r.kind,
        //     r.name,
        //     r.path,
        //     r.size
        // );
        return ();
    }

}
