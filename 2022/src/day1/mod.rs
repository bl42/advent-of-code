// https://adventofcode.com/2022/day/1
// --- Day 1: Calorie Counting ---

pub fn run() {
    let input = include_str!("input.txt");

    println!("Day 1 - Part 1: {}", part1(&input));
    println!("Day 1 - Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut cals = convert_input_into_calories(input);
    cals.sort_by(|a, b| b.cmp(a));

    cals[0]
}

fn part2(input: &str) -> u32 {
    let mut cals = convert_input_into_calories(input);
    cals.sort_by(|a, b| b.cmp(a));

    cals[0] + cals[1] + cals[2]
}

fn convert_input_into_calories(input: &str) -> Vec<u32> {
    let bags: Vec<&str> = input.split("\n\n").collect();

    bags.into_iter()
        .map(|bag| count_calories_in_bag(bag))
        .collect()
}

fn count_calories_in_bag(bag: &str) -> u32 {
    let cals: Vec<u32> = bag
        .lines()
        .into_iter()
        .map(|cal| cal.parse().expect("number"))
        .collect();

    cals.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn first() {
        assert_eq!(part1(INPUT), 24000);
    }
    #[test]
    fn second() {
        assert_eq!(part2(INPUT), 45000);
    }
}
