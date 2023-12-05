use clap::Parser;
use utils::{read_lines, Config};

#[warn(unused_variables)]
fn part_one(line: String) -> usize {
    let _ = line;
    todo!();
}

#[warn(unused_variables)]
fn part_two(line: String) -> usize {
    let _ = line;
    todo!();
}

fn main() {
    let config = Config::parse();
    let mut sum: usize = 0;
    for line in read_lines(config.file).unwrap() {
        if let Ok(text) = line {
            sum += match config.part {
                1 => part_one(text) as usize,
                2 => part_two(text) as usize,
                _ => panic!("Part should be either 1 or 2 (1 by default)"),
            }
        }
    }
    println!("{}", sum);
}
