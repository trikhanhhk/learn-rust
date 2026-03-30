use std::{env, fs, process};

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    SingleThread,
    MultiThread,
    Rayon,
}
pub struct Config {
    pub path: String,
    pub mode: Mode,
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| {
        eprintln!("Usage: mini_cli -- <path> --mode <mod>");
        process::exit(1);
    });

    fs::metadata(&path).unwrap_or_else(|_| {
        eprintln!("Error: Unable to access the provided path.");
        process::exit(1);
    });

    let mode = args.get(3).cloned().unwrap_or_else(|| {
        eprintln!("Usage: mini_cli -- <path> --mode <mode>");
        process::exit(1);
    });

    let mode = match mode.as_str() {
        "single" => Mode::SingleThread,
        "multi" => Mode::MultiThread,
        "rayon" => Mode::Rayon,
        _ => {
            eprintln!("Error: Invalid mode.");
            process::exit(1);
        }
    };

    Config { path, mode }
}