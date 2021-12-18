use aoc2021::read_file;
use itertools::Itertools;
use std::str::FromStr as _;

#[derive(Debug)]
enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl std::str::FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_whitespace().next_tuple().unwrap();

        let amount = amount.parse::<i32>().unwrap();

        match direction {
            "forward" => Ok(Movement::Forward(amount)),
            "down" => Ok(Movement::Down(amount)),
            "up" => Ok(Movement::Up(amount)),
            _ => panic!("String ({}) couldn't be read as a valid movement", s),
        }
    }
}

fn part_one() {
    let input: Vec<Movement> =
        read_file("/home/developer/huws-stuff/aoc2021/inputs/day02_1.txt".to_string())
            .unwrap()
            .iter()
            .map(|line| Movement::from_str(line).unwrap())
            .collect();

    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for movement in input {
        match movement {
            Movement::Forward(amount) => {
                horizontal += amount;
            }
            Movement::Down(amount) => {
                depth += amount;
            }
            Movement::Up(amount) => {
                depth -= amount;
            }
        }
    }

    println!("part_one:");
    println!("  depth: {}", depth);
    println!("  horizontal: {}", horizontal);
    println!("  multiplication: {}", depth * horizontal);
}

fn part_two() {
    let input: Vec<Movement> =
        read_file("/home/developer/huws-stuff/aoc2021/inputs/day02_1.txt".to_string())
            .unwrap()
            .iter()
            .map(|line| Movement::from_str(line).unwrap())
            .collect();

    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;

    for movement in input {
        match movement {
            Movement::Forward(amount) => {
                horizontal += amount;
                depth += amount * aim;
            }
            Movement::Down(amount) => {
                aim += amount;
            }
            Movement::Up(amount) => {
                aim -= amount;
            }
        }
    }

    println!("part_two:");
    println!("  depth: {}", depth);
    println!("  horizontal: {}", horizontal);
    println!("  aim: {}", aim);
    println!("  multiplication: {}", depth * horizontal);
}

fn main() {
    part_one();
    part_two();
}
