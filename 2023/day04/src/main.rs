use clap::Parser;
use std::collections::{hash_map::RandomState, HashSet};
use utils::{read_lines, Config};

#[derive(Debug)]
struct Card {
    winning: Vec<usize>,
    current: Vec<usize>,
    id: usize,
}

fn deserialize_numbers(s: &str) -> Vec<usize> {
    // 58  6 71 93 96 38 25 29 17  8
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

fn vec_to_set(value: Vec<usize>) -> HashSet<usize, RandomState> {
    let mut output = HashSet::new();
    for item in value.iter() {
        output.insert(*item);
    }
    output
}

#[warn(unused_variables)]
fn part_one(line: String) -> usize {
    let card = Card::from(line);

    // Converting into HashSet and ensuring there are no duplicate
    let expected_c = card.current.len();
    let expected_w = card.winning.len();
    let current_set: HashSet<usize> = vec_to_set(card.current);
    let winning_set: HashSet<usize> = vec_to_set(card.winning);
    assert_eq!(expected_c, current_set.len());
    assert_eq!(expected_w, winning_set.len());

    let winners: HashSet<usize, RandomState> = current_set
        .intersection(&winning_set)
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
