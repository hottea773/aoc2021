use aoc2021::read_file;
use itertools::Itertools;

fn part_one() {
    let input: Vec<usize> =
        read_file("/home/developer/huws-stuff/aoc2021/inputs/day01_1.txt".to_string())
            .unwrap()
            .iter()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

    let mut increase_count: usize = 0;

    for (previous, current) in input.iter().tuple_windows() {
        if current > previous {
            increase_count += 1;
        }
    }

    println!("part_one increase_count: {}", increase_count)
}

fn part_two() {
    let input: Vec<usize> =
        read_file("/home/developer/huws-stuff/aoc2021/inputs/day01_1.txt".to_string())
            .unwrap()
            .iter()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

    let mut increase_count: usize = 0;

    for (n_minus_three, _, _, current) in input.iter().tuple_windows() {
        if current > n_minus_three {
            increase_count += 1;
        }
    }

    println!("part_two increase_count: {}", increase_count)
}

fn main() {
    part_one();
    part_two();
}
