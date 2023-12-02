pub use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The part of the challenge
    #[arg(short, long, default_value_t = 1)]
    pub part: u8,

    /// Path to the test file
    #[arg()]
    pub file: PathBuf,
}

/// Iterate over lines of a file
/// ```rust
/// if let Ok(lines) = read_lines("path/to/file.txt") {
///     for line in lines {
///         if let Ok(line) = line {
///             println!("{}", line);
///         }
///     }
/// }
/// ```
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
