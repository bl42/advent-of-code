// https://adventofcode.com/2022/day/10
// --- Day 10 ---

fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> i32 {
    0
}

pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 10.1: {} \t\t Day 10.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");
    const SAMPLE_INPUT2: &str = include_str!("input.sample2.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 0);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT2), 0);
    }
}
