// https://adventofcode.com/2022/day/6
// --- Day 6 ---
use std::collections::HashSet;

fn find_start(input: &str, marker: usize) -> usize {
    for (i, arr) in input.as_bytes().windows(marker).enumerate() {
        let mut set = HashSet::new();
        //test
        for item in arr {
            set.insert(item);
        }
        if set.len() == marker {
            return i + marker;
        }
    }
    0
}

fn part1(input: &str) -> usize {
    find_start(input, 4)
}

fn part2(input: &str) -> usize {
    find_start(input, 14)
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("Day 6.1: {} \t\t Day 6.2: {}", part1(input), part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    }
    #[test]
    fn second() {
        assert_eq!(find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    }
}
