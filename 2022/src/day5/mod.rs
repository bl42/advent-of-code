pub mod yard;
use yard::*;

// https://adventofcode.com/2022/day/5
// --- Day 5 ---

pub fn run() {
    let input = include_str!("input.txt");

    println!("Day 5.1: {} \t Day 5.2: {}", part1(&input), part2(&input));
}

fn process_input(input: &str) -> (Yard, Vec<(usize, usize, usize)>) {
    let (yard_raw, moves_raw) = input.split_once("\n\n").unwrap();

    let yard: Yard = yard_raw.parse().unwrap();
    let moves = moves_raw
        .lines()
        .map(|m| {
            let action: Vec<&str> = m.split(' ').collect();
            let count = action[1].parse::<usize>().unwrap();
            let from = action[3].parse::<usize>().unwrap();
            let to = action[5].parse::<usize>().unwrap();
            (count, from, to)
        })
        .collect();

    (yard, moves)
}

fn part1(input: &str) -> String {
    let (mut yard, moves) = process_input(input);

    for (count, from, to) in moves {
        yard.relocate(count, from, to)
    }

    yard.message()
}

fn part2(input: &str) -> String {
    let (mut yard, moves) = process_input(input);

    for (count, from, to) in moves {
        yard.relocate_batch(count, from, to)
    }

    yard.message()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), "CMZ");
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), "MCD");
    }
}
