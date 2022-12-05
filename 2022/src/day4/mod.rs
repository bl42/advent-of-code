// https://adventofcode.com/2022/day/4
// --- Day 4 ---

mod chore;
use chore::*;

pub fn run() {
    let input = include_str!("input.txt");

    println!("Day 4.1: {} \t\t Day 4.2: {}", part1(&input), part2(&input));
}

fn convert_input_to_elf_chores(input: &str) -> Vec<Vec<Chore>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|chore| chore.parse::<Chore>().expect("Shoud match n-n"))
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let elf_chores = convert_input_to_elf_chores(input);

    elf_chores
        .into_iter()
        .filter(|chores| chores[0].contains_or_cotained(&chores[1]))
        .count()
}

fn part2(input: &str) -> usize {
    let elf_chores = convert_input_to_elf_chores(input);

    elf_chores
        .into_iter()
        .filter(|chores| chores[0].overlaps_or_overlaped(&chores[1]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 2);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 4);
    }
}
