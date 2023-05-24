use std::{io::{self, Write}, fs};
use clap::{Parser};

/// Simple program to create files and folders
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to create
    path: String,
    /// The created object is a directory
    #[arg(short= 'd', long)]
    directory: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.directory {
        fs::create_dir_all(args.path)
    } else {
        let mut file = fs::File::create(args.path)?;
        // TODO: Check if stdin buffer is empty or not before reading data
        let stdin_data: Vec<String> = io::stdin().lines().filter_map(|d| d.ok()).collect();
        file.write(&stdin_data.join("\n").into_bytes())?;
        Ok(())
    }
}