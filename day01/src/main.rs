use std::collections::HashMap;
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

fn finddall(text: &String, pattern: &str) -> Vec<usize> {
    let mut output = Vec::new();
    let mut slice = &text[0..text.len()];
    let mut tot_pos = 0;
    while let Some(pos) = slice.find(pattern) {
        output.push(tot_pos + pos);
        tot_pos += pos + pattern.len();
        slice = &slice[pos + pattern.len()..];
    }
    output
}

fn extract_number(text: String) -> u8 {
    let text_to_digit: HashMap<&str, u8> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        // Comment bellow for part one
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut indexes: HashMap<usize, &str> = HashMap::new();
    for digit_str in text_to_digit.keys().into_iter() {
        for pos in finddall(&text, digit_str).into_iter() {
            indexes.insert(pos, digit_str);
        }
    }

    let first = text_to_digit
        .get(indexes.get(indexes.keys().min().unwrap()).unwrap())
        .unwrap();
    let last = text_to_digit
        .get(indexes.get(indexes.keys().max().unwrap()).unwrap())
        .unwrap();
    let first_and_last = first.to_string() + last.to_string().as_str();

    return first_and_last.parse::<u8>().unwrap();
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
