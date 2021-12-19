use aoc2021::read_file;
use itertools::Itertools;
use std::collections::HashMap;

fn part_one() {
    let input: Vec<String> =
        read_file("/home/developer/huws-stuff/aoc2021/inputs/day03_1.txt".to_string()).unwrap();

    let mut results: HashMap<usize, (usize, usize)> = HashMap::new();

    for number in input {
        for (column_number, column_value) in number.chars().enumerate() {
            let (mut zeros, mut ones) = results.get(&column_number).unwrap_or(&(0, 0));
            match column_value.to_digit(2).unwrap() {
                0 => zeros += 1,
                1 => ones += 1,
                _ => panic!("oh"),
            }
            results.insert(column_number, (zeros, ones));
        }
    }

    let mut gamma_rate: Vec<char> = Vec::new();
    for _ in 0..results.len() {
        gamma_rate.push('0')
    }

    for (column_number, (zeros, ones)) in results.iter() {
        if ones > zeros {
            let value = gamma_rate.get_mut(*column_number).unwrap();
            *value = '1';
        }
    }

    let mut epsilon_rate = gamma_rate.iter().map(|digit| match digit {
        '0' => '1',
        '1' => '0',
        _ => unreachable!(),
    });

    let gamma_rate = usize::from_str_radix(gamma_rate.iter().join("").as_str(), 2).unwrap();
    let epsilon_rate = usize::from_str_radix(epsilon_rate.join("").as_str(), 2).unwrap();

    println!("Gamma rate: {:?}", gamma_rate);
    println!("epsilon_rate: {:?}", epsilon_rate);
    println!("Multiple: {}", gamma_rate * epsilon_rate);
}

fn part_two() {}

fn main() {
    part_one();
    part_two();
}
