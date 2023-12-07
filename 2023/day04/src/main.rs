use clap::Parser;
use std::{
    cmp::Ordering,
    collections::{hash_map::RandomState, HashSet},
    path::Path,
};
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

impl Card {
    fn points(&self) -> usize {
        let winners: HashSet<usize, RandomState> = self
            .current
            .intersection(&self.winning)
            .map(|item| *item)
            .collect();
        if winners.is_empty() {
            return 0;
        }

        let points = (2 as usize).pow((winners.len() - 1) as u32);
        points
    }

    fn winners_amount(&self) -> usize {
        self.current
            .intersection(&self.winning)
            .map(|item| *item)
            .count()
    }
}

// Sort only by card ID
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.id).cmp(&other.id)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Card {}

fn part_one(file: &Path) -> usize {
    let mut sum: usize = 0;
    for line in read_lines(&file).unwrap() {
        if let Ok(text) = line {
            sum += Card::from(text).points() as usize;
        }
    }
    sum
}

#[warn(unused_variables)]
fn part_two(file: &Path) -> usize {
    let mut cards = vec![];
    for line in read_lines(&file).unwrap() {
        if let Ok(text) = line {
            cards.push(Card::from(text));
        }
    }
    cards.sort();

    let mut amounts: Vec<usize> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let winners_amount = card.winners_amount();
        let current_card_amount: usize = *amounts.get(i).unwrap_or(&(0 as usize));
        for next_i in 1..winners_amount + 1 {
            if let Some(amount) = amounts.get_mut(i + next_i) {
                *amount += current_card_amount;
            }
        }
    }

    amounts.iter().sum()
}

fn main() {
    let config = Config::parse();
    let result = match config.part {
        1 => part_one(&config.file) as usize,
        2 => part_two(&config.file) as usize,
        _ => panic!("Part should be either 1 or 2 (1 by default)"),
    };
    println!("{}", result);
}
