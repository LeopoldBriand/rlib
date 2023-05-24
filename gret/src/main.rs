use std::{io::{self, Write}};
use clap::{Parser};
use regex::Regex;

/// Simple program find patterns in files or pipelines
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search
    regex: String,
    /// Prefix each output line by the line number of the input
    #[arg(short= 'n', long)]
    line_number: bool,
    /// Output number of founded elements instead of elements
    #[arg(short= 'c', long)]
    count: bool,
    /// Color matched pattern
    #[arg(short= 'C', long)]
    color: bool,
}

fn main() -> io::Result<()>{
    let args = Args::parse();
    let stdin_data: Vec<String> = io::stdin().lines().filter_map(|d| d.ok()).collect();
    let regex = Regex::new(&args.regex).unwrap();
    let result = search(stdin_data, regex.clone());

    let mut stdout = io::stdout().lock();
    if args.count {
        stdout.write_all(format!("{}\n",result.len()).as_bytes())?;
    }
    else {
        for r in result {
            let mut output: String = r.1.clone(); // Standard output
            if args.color {
                output = regex.replace_all(&r.1, "\x1b[1;31m$0\x1b[0m").to_string();
            }
            if args.line_number { // Prefix output with line number
                output = format!("{} - {}", r.0, output);
            }
            
            stdout.write_all(format!("{}{}", output, "\n").as_bytes())?;
        }
    }
    Ok(())
}

fn search(data: Vec<String>, regex: Regex) -> Vec<(i64, String)> {
    let mut result: Vec<(i64, String)> = vec![];
    for (i, line) in data.iter().enumerate() {
        if regex.is_match(&line) {
            result.push((i as i64, line.to_string()));
        }
    }
    result
}