use clap::Parser;
use walkdir::WalkDir;
use std::fs;
use std::io::{self, BufRead};

#[derive(Parser)]
struct Cli {
    directory: String,
    keyword: String,
}

fn main() {
    let args = Cli::parse();
    search_directory(&args.directory, &args.keyword);
}

fn search_directory(directory: &str, keyword: &str) {
    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            if let Ok(file) = fs::File::open(entry.path()) {
                let reader = io::BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    if let Ok(line) = line {
                        if line.contains(keyword) {
                            println!("Found in {} at line {}: {}", entry.path().display(), index + 1, line);
                        }
                    }
                }
            }
        }
    }
}
