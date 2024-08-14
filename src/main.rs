use anyhow::Result;
use log::{error, info};
use std::env::args;
use std::fs;
use std::path::Path;


struct FileInformation {
    bytes: u64,
    lines: u32,
    words: u32,
}

impl FileInformation {
    fn new(bytes: u64, lines: u32, words: u32) -> Self {
        FileInformation {
            bytes,
            lines,
            words,
        }
    }

    fn from(tuple: (u64, u32, u32)) -> Self {
        let (bytes, lines, words) = tuple;
        FileInformation::new(bytes, lines, words)
    }
}

enum Commands {
    GetBytes,
    GetLines,
    GetWords,
    Empty,
}

fn main() -> Result<(), anyhow::Error> {
    initial_logger();
    let mut args = get_args()?;
    let path = get_path(&mut args)?;
    let command = parse_args_to_command(&args);
    let file_information = get_all_information(&path).map(|tuple| FileInformation::from(tuple))?;
    print_command_result(&command, &file_information);
    Ok(())
}

fn initial_logger() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}

fn get_args() -> Result<Vec<String>, anyhow::Error> {
    let args: Vec<String> = args().skip(1).collect();
    args_is_empty(&args)?;
    Ok(args)
}

fn get_path(arg: &mut Vec<String>) -> Result<String, anyhow::Error> {
    if let Some(path) = arg.pop() {
        return if Path::new(&path).exists() {
            Ok(path)
        } else {
            error!("Path does not exist!");
            Err(anyhow::anyhow!("Path does not exist!"))
        };
    }
    error!("Argument path does not exist!");
    Err(anyhow::anyhow!("Argument path does not exist!"))
}

fn args_is_empty(args: &Vec<String>) -> Result<(), anyhow::Error> {
    if args.is_empty() {
        error!("No arguments provided");
        return Err(anyhow::anyhow!("No arguments provided"));
    }
    Ok(())
}

fn parse_args_to_command(args: &Vec<String>) -> Commands {
    return match args.first().map(String::as_str) {
        Some("-c") => Commands::GetBytes,
        Some("-l") => Commands::GetLines,
        Some("-w") => Commands::GetWords,
        _ => Commands::Empty,
    };
}

fn print_command_result(command: &Commands, file: &FileInformation) {
    match command {
        Commands::GetBytes => info!("Bytes in file: {}", file.bytes),
        Commands::GetLines => info!("Lines in file: {}", file.lines),
        Commands::GetWords => info!("Words in file: {}", file.words),
        Commands::Empty => info!(
            "All information:\nBytes: {}\nLines: {}\nWords: {}",
            file.bytes, file.lines, file.words
        ),
    };
}

fn get_all_information(path: &str) -> Result<(u64, u32, u32), anyhow::Error> {
    let all_text = fs::read_to_string(path).map_err(|e| {
        error!("Failed to read file content: {}", e);
        e
    })?;

    let count_bytes = fs::metadata(path)
        .map_err(|e| {
            error!("Failed to retrieve file metadata: {}", e);
            e
        })?
        .len();
    let count_lines = all_text.lines().count() as u32;
    let count_words = all_text.split_whitespace().count() as u32;

    Ok((count_bytes, count_lines, count_words))
}
