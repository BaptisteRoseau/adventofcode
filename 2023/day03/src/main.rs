use clap::Parser;
use std::path::Path;
use utils::{read_lines, Config};

const EMPTY: char = '.';
const GEAR: char = '*';

/// A (x, y) position used to ease access into a grid and list its neighbors.
#[derive(Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn around(&self) -> Vec<Pos> {
        let mut output = vec![];
        output.push(Pos {
            x: self.x + 1,
            y: self.y,
        });
        output.push(Pos {
            x: self.x + 1,
            y: self.y + 1,
        });
        output.push(Pos {
            x: self.x + 1,
            y: self.y - 1,
        });
        output.push(Pos {
            x: self.x - 1,
            y: self.y,
        });
        output.push(Pos {
            x: self.x - 1,
            y: self.y + 1,
        });
        output.push(Pos {
            x: self.x - 1,
            y: self.y - 1,
        });
        output.push(Pos {
            x: self.x,
            y: self.y + 1,
        });
        output.push(Pos {
            x: self.x,
            y: self.y - 1,
        });
        output
    }
}

/// A two-dimensional grid representing the grid given in input.
/// Use `get` and `set` with a `Pos` object to interract with the grid.
struct Engine {
    map: Vec<Vec<char>>,
    length: usize,
    width: usize,
}

impl Engine {
    pub fn from_file(file: &Path) -> Result<Self, std::io::Error> {
        let mut map = vec![];
        for line in read_lines(file)? {
            if let Ok(line) = line {
                let l: Vec<char> = line.as_bytes().into_iter().map(|b| *b as char).collect();
                map.push(l);
            }
        }
        let length = map.len();
        let width = map.first().unwrap().len();
        Ok(Self { map, length, width })
    }
    pub fn get(&self, pos: &Pos) -> Option<&char> {
        self.map.get(pos.x)?.get(pos.y)
    }

    pub fn set(&mut self, pos: &Pos, value: &char) -> Result<&mut Self, ()> {
        // Maybe clean this mess ?
        if let Some(line) = self.map.get_mut(pos.x) {
            if let Some(item) = line.get_mut(pos.y) {
                *item = *value;
            } else {
                return Err(());
            }
        } else {
            return Err(());
        }
        Ok(self)
    }
}

/// Complete a number from a grid's position.
/// For example, in the row `...789*..`, if given the position (0, 4),
/// you will fall on the digit 8.
/// Calling `complete_number(&mut engine, Pos {x: 0, y: 4})` will then return 789.
fn complete_number(engine: &mut Engine, pos: &Pos) -> usize {
    let mut buffer: Vec<char> = vec![];
    let mut current_pos = *pos;
    // Fill digits from right
    while let Some(c) = engine.get(&current_pos) {
        if !c.is_ascii_digit() {
            break;
        }
        buffer.push(*c);
        engine.set(&current_pos, &EMPTY).unwrap();
        current_pos.y += 1;
    }

    // Fill digits from left
    current_pos = *pos;
    current_pos.y -= 1;
    while let Some(c) = engine.get(&current_pos) {
        if !c.is_ascii_digit() {
            break;
        }
        buffer.insert(0, *c);
        engine.set(&current_pos, &EMPTY).unwrap();
        if current_pos.y == 0 {
            break;
        }
        current_pos.y -= 1;
    }

    let buffer: String = buffer.iter().collect();
    buffer.parse::<usize>().unwrap()
}

fn find_neighbor_numbers(engine: &mut Engine, pos: &Pos) -> Vec<usize> {
    let mut numbers = vec![];
    let current = engine.get(pos);
    if current.is_none() || *current.unwrap() == EMPTY || current.unwrap().is_ascii_digit() {
        return numbers;
    }
    for neighbor in pos.around().into_iter() {
        if let Some(c) = engine.get(&neighbor) {
            if c.is_ascii_digit() {
                let number = complete_number(engine, &neighbor);
                numbers.push(number);
            }
        }
    }
    numbers
}

fn find_part_numbers_part_1(engine: &mut Engine) -> Vec<usize> {
    let mut number = vec![];
    for x in 0..engine.length {
        for y in 0..engine.width {
            let pos = Pos { x, y };
            if let Some(token) = engine.get(&pos) {
                if *token == EMPTY || token.is_ascii_digit() {
                    continue;
                }
                number.append(&mut find_neighbor_numbers(engine, &pos));
                engine.set(&pos, &EMPTY).unwrap();
            }
        }
    }
    number
}

fn find_gear_products_part_2(engine: &mut Engine) -> Vec<usize> {
    let mut number = vec![];
    for x in 0..engine.length {
        for y in 0..engine.width {
            let pos = Pos { x, y };
            if let Some(token) = engine.get(&pos) {
                if *token != GEAR {
                    continue;
                }
                let neighbor_numbers = find_neighbor_numbers(engine, &pos);
                if neighbor_numbers.len() >= 2 {
                    number.push(neighbor_numbers.into_iter().product());
                    engine.set(&pos, &EMPTY).unwrap();
                }
            }
        }
    }
    number
}

fn main() {
    let config = Config::parse();
    let mut engine = Engine::from_file(&config.file).unwrap();
    let values = match config.part {
        1 => find_part_numbers_part_1(&mut engine),
        2 => find_gear_products_part_2(&mut engine),
        _ => panic!("Part should be either 1 or 2"),
    };
    let sum: usize = values.into_iter().sum();
    println!("{}", sum);
}
