use std::fs;
use clap::{Parser};

/// Simple program to remove files and folders
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to remove
    paths: Vec<String>,
    /// Remove empty directory
    #[arg(short= 'd', long)]
    dir: bool,
    /// Remove files and directories recursively
    #[arg(short= 'R', long)]
    recursive: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    for path in args.paths {
        if args.recursive {
            fs::remove_dir_all(path)?
        } else if args.dir {
            fs::remove_dir(path)?
        } else {
            fs::remove_file(path)?
        }
    }
    Ok(())
}