// https://adventofcode.com/2022/day/13
// --- Day 13 ---

use regex::Regex;

fn part1(input: &str) -> usize {
    let packets: Vec<(&str, &str)> = input
        .split("\n\n")
        .map(|a| a.split_once('\n').unwrap())
        .collect();

    for (left, right) in packets {
        println!("{left:?} {right:?}");
    }

    0
}

fn part2(input: &str) -> usize {
    let pattern = Regex::new(r"\[(.+)\]").unwrap();
    let input = "[1,1,5,1,1]";
    let matched = pattern.captures_iter(input);
    println!("Matched text: {:#?}", matched);

    0
}

pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 13.1: {} \t\t Day 13.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 0);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 0);
    }
}
