// https://adventofcode.com/2022/day/3
// --- Day 3 ---

use std::collections::HashSet;

pub fn run() {
    let input = include_str!("input.txt");

    println!("Day 3.1: {} \t\t Day 3.2: {}", part1(input), part2(input));
}

fn convert_char_to_u8(letter: char) -> u8 {
    let byte = *letter
        .to_string()
        .as_bytes()
        .first()
        .expect("&str provided doesnt have a byte");
    if byte > 96 {
        return byte - 96;
    }
    byte - 38
}

fn part1(input: &str) -> i32 {
    let errors_in_sack: Vec<char> = input
        .lines()
        .map(|rucksacks| {
            let (first, other) = rucksacks.split_at(rucksacks.len() / 2);

            let first_group: HashSet<char> = first.chars().collect();
            let second_group: HashSet<char> = other.chars().collect();
            let common = first_group.intersection(&second_group);

            common.copied().next().expect("")
        })
        .collect();

    let error_in_sack_total: i32 = errors_in_sack
        .into_iter()
        .map(|c| convert_char_to_u8(c) as i32)
        .into_iter()
        .sum();

    error_in_sack_total
}

fn part2(input: &str) -> i32 {
    let sacks: Vec<&str> = input.lines().collect();
    let groups: Vec<i32> = sacks
        .chunks(3)
        .map(|chunk| {
            let first: HashSet<char> = chunk[0].chars().collect();
            let second: HashSet<char> = chunk[1].chars().collect();
            let third: HashSet<char> = chunk[2].chars().collect();

            let common = first
                .intersection(&second)
                .cloned()
                .collect::<HashSet<char>>()
                .intersection(&third)
                .copied()
                .next()
                .expect("common in group");

            convert_char_to_u8(common) as i32
        })
        .collect();

    groups.into_iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 157);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 70);
    }
}
