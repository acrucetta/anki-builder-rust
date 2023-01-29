use clap::Parser;
use log::{error, info};
use std::env;
use anki_builder_rust::run;

#[derive(Parser)]
struct Cli {
    command: String,
    path: std::path::PathBuf,
}

enum Command {
    Ankify,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    env_logger::init();
    info!("Starting the program");

    let args = Cli::parse();
    let result = std::fs::read_to_string(args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => return Err(CustomError(error.to_string())),
    };
    info!("File content preview: {}", &content[0..20]);

    let command = match args.command.as_str() {
        "ankify" => Command::Ankify,
        _ => return Err(CustomError(format!("Unknown command: {}", args.command))),
    };

    match command {
        Command::Ankify => run(&content),
    };

    Ok(())
}
