use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_number(text: String) -> u8 {
    let mut buffer: String = "".to_string();
    let digits: Vec<char> = text
        .chars()
        .into_iter()
        .filter(|c| c.is_ascii_digit())
        .collect();
    buffer += (digits.first().unwrap().to_string() + digits.last().unwrap().to_string().as_str())
        .as_str();
    return buffer.parse::<u8>().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines(args.get(1).unwrap()) {
        for line in lines {
            if let Ok(text) = line {
                sum += extract_number(text) as u64;
            }
        }
    }
    println!("TOTAL: {}", sum);
}
