use clap::Parser;
use std::collections::{hash_map::RandomState, HashSet};
use utils::{read_lines, Config};

/// Information of a Card:
///
///     Card  16: 13 93  1 50 51 28 73 67 56  4 | 12 81 20 82  9 48 21 78 36 17 76 35 57 91 18 27 11 16 49 23  5 65 58 29 62
///
/// Will result in:
///
/// ```
/// Card {
///     winning: {13, 93, 1, 50, 51, 28, 73, 67, 56, 4},
///     current: {12, 81, 20, 82, 9, 48, 21, 78, 36, 17, 76, 35, 57, 91, 18, 27, 11, 16, 49, 23, 5, 65, 58, 29, 62},
///     id: 16,
/// }
/// ```
///
/// Note that we can use a HashSet because winning and got numbers
/// are not supposed to contain duplicates in a lottery.
#[derive(Debug)]
struct Card {
    winning: HashSet<usize>,
    current: HashSet<usize>,
    id: usize,
}

fn deserialize_numbers(s: &str) -> HashSet<usize> {
    s.split_ascii_whitespace()
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect()
}

impl From<String> for Card {
    fn from(value: String) -> Self {
        let split = value.split_once(':').unwrap();
        let id = split
            .0
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();

        let split = split.1.split_once('|').unwrap();
        let winning = deserialize_numbers(split.0);
        let current = deserialize_numbers(split.1);

        Self {
            winning,
            current,
            id,
        }
    }
}

#[warn(unused_variables)]
fn part_one(line: String) -> usize {
    let card = Card::from(line);
    let winners: HashSet<usize, RandomState> = card
        .current
        .intersection(&card.winning)
        .map(|item| *item)
        .collect();
    if winners.is_empty() {
        return 0;
    }

    let points = (2 as usize).pow((winners.len() - 1) as u32);
    // println!("Card {:?}: {:?}", card.id, points);
    points
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
