use std::fs;

// https://adventofcode.com/2021/day/3
// Day 6: Lanternfish

pub fn run() {
    let input = fs::read_to_string("./inputs/6.txt").expect("File required");

    println!("Day 6 Result - Part 1: {:?}", part1(&input, 80));
    println!("Day 6 Result - Part 2: {:?}", part1(&input, 256));
}

fn part1(input: &str, count: usize) -> i64 {
    let lantern_fish: Vec<usize> = input
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let mut fishes: [i64; 9] = [0; 9];

    for starting_fish in lantern_fish {
        fishes[starting_fish + 1] += 1;
    }

    for day in 0..count + 1 {
        let reborn = fishes.clone()[0];
        let next = &fishes.clone()[1..];

        for (i, &next_fish) in next.iter().enumerate() {
            fishes[i] = next_fish;
        }
        fishes[8] = reborn;
        fishes[6] += reborn;
    }

    fishes.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn first() {
        assert_eq!(part1(INPUT, 80), 5934);
    }
}
