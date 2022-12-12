// https://adventofcode.com/2022/day/11
// --- Day 11 ---
mod monkey;
use monkey::{Monkey, Test};

fn part1(input: &str) -> i32 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|i| i.parse::<Monkey>().unwrap())
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((destination, value)) = monkeys[i].throw_item(None) {
                monkeys.get_mut(destination).unwrap().items.push(value);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));

    monkeys[0].inspect_count * monkeys[1].inspect_count
}

fn part2(input: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|i| i.parse::<Monkey>().unwrap())
        .collect();

    let relief: f32 = monkeys
        .iter()
        .map(|mon| match mon.test {
            Test::Divisible(i) => i,
        })
        .product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some((destination, value)) = monkeys[i].throw_item(Some(relief as f64)) {
                monkeys.get_mut(destination).unwrap().items.push(value);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));

    (monkeys[0].inspect_count as i64) * (monkeys[1].inspect_count as i64)
}
pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 11.1: {} \t Day 11.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 10605);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 2713310158);
    }
}
