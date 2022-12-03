// https://adventofcode.com/2022/day/3
// --- Day 3 ---

use crate::day2::game::*;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Day 3.1: {} \t\t Day 3.2: {}", part1(&input), part2(&input));
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
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
