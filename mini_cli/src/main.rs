// use std::{path::Path, vec};

use mini_cli::server::tcp::start_server;

// use crate::cli::args::parse_args;
fn main() {
    // let args = parse_args();
    // let path = Path::new(&args.path);
    // let mode = args.mode;
    // if mode == cli::args::Mode::SingleThread {
    //     let start = std::time::Instant::now();
    //     let _ = services::build::build_single(path);
    //     let duration = start.elapsed();
    //     println!("Single thread: {} ms", duration.as_millis());
    // } else if mode == cli::args::Mode::MultiThread {
    //     let mut performance_test = vec![];
    //     let thread_options = 1..30;
    //     for threads in thread_options.clone() {
    //         let start = std::time::Instant::now();
    //         let _ =services::build::build_threads(path.to_path_buf(), threads);
    //         let duration = start.elapsed();
    //         performance_test.push(duration);
    //     }
    //     if let Some((min_index, &min_time)) =
    //         performance_test.iter().enumerate().min_by_key(|(_, t)| **t)
    //     {
    //         println!(
    //             "Good at: {} threads → {} ms",
    //             thread_options.clone().nth(min_index).unwrap(),
    //             min_time.as_millis()
    //         );
    //     }
    // } else if mode == cli::args::Mode::Rayon {
    //     let start = std::time::Instant::now();
    //     let _ =services::build::build_rayon(path.to_path_buf());
    //     let duration = start.elapsed();
    //     println!("Rayon: {} ms", duration.as_millis());
    // }

    start_server("127.0.0.1:8080");
}
