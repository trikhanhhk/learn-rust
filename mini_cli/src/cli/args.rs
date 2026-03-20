use std::{env, fs, process};

#[derive(Debug)]
pub struct Config {
    pub path: String
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| {
        eprintln!("Usage: mini_cli -- <path>");
        process::exit(1);
    });

    fs::metadata(&path).unwrap_or_else(|_| {
        eprintln!("Error: Unable to access the provided path.");
        process::exit(1);
    });

    Config { path }    
}