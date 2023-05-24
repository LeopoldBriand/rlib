use std::{io::{self, Write}, fs};
use clap::{Parser};
use utils::is_readable_stdin;

/// Simple program to create files and folders
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to create
    paths: Vec<String>,
    /// The created object is a directory
    #[arg(short= 'd', long)]
    directory: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    for path in args.paths {
        if args.directory {
            fs::create_dir_all(path)?
        } else {
            let mut file = fs::File::create(path)?;
            if is_readable_stdin() {
                let stdin_data: Vec<String> = io::stdin().lines().filter_map(|d| d.ok()).collect();
                file.write(&stdin_data.join("\n").into_bytes())?;
            }
        }
    }
    Ok(())
}