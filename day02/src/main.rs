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

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

fn pop_game_id(line: &mut String) -> usize {
    let split = line.split_once(':').unwrap();
    let game_id = split.0[split.0.find(' ').unwrap() + 1..]
        .parse::<usize>()
        .unwrap();
    *line = split.1.to_string();
    game_id
}

fn balls_to_vector(line: String) -> Vec<(usize, &'static str)> {
    let colors: Vec<&str> = Vec::from([RED, GREEN, BLUE]);
    let mut output = vec![];
    for balls_set in line.split(';').into_iter() {
        for single_color_balls in balls_set.split(',').into_iter() {
            let single_color_balls = single_color_balls.trim();
            for color in colors.iter() {
                if single_color_balls.contains(color) {
                    let amount = single_color_balls[0..single_color_balls.find(' ').unwrap()]
                        .parse::<usize>()
                        .unwrap();
                    output.push((amount, *color));
                }
            }
        }
    }
    output
}

/// Returns the ID of the game if it is possible, 0 if not
fn game_id_part_one(line: String) -> usize {
    let limits: HashMap<&str, usize> = HashMap::from([(RED, 12), (GREEN, 13), (BLUE, 14)]);
    let mut line = line.clone();
    let game_id = pop_game_id(&mut line);

    for (amount, color) in balls_to_vector(line).into_iter() {
        if amount > *limits.get(color).unwrap() {
            println!("GAME {}:\tIMPOSSIBLE", game_id);
            return 0;
        }
    }

    println!("GAME {}:\tPOSSIBLE", game_id);
    game_id
}

fn minimum_cube_amount_part_two(line: String) -> usize {
    let mut minimum_seen: HashMap<&str, usize> = HashMap::from([(RED, 1), (GREEN, 1), (BLUE, 1)]);
    let mut line = line.clone();
    let game_id = pop_game_id(&mut line);

    for (amount, color) in balls_to_vector(line).into_iter() {
        if amount > *minimum_seen.get(color).unwrap() {
            minimum_seen.insert(color, amount);
        }
    }

    let power = minimum_seen.into_values().product();
    println!("GAME {}:\t{}", game_id, power);
    power
}

fn main() {
    let default_part: String = "1".to_string();
    let args: Vec<String> = env::args().collect();
    let part = args.get(2).unwrap_or(&default_part);
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines(args.get(1).unwrap()) {
        for line in lines {
            if let Ok(text) = line {
                sum += match part.as_str() {
                    "1" => game_id_part_one(text) as u64,
                    "2" => minimum_cube_amount_part_two(text) as u64,
                    _ => panic!("Part should be either 1 or 2 (1 by default)"),
                }
            }
        }
    }
    println!("TOTAL: {}", sum);
}
