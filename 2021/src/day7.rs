use std::fs;

// https://adventofcode.com/2021
// Day 7: The Treachery of Whales

pub fn run() {
    let input = fs::read_to_string("./inputs/7.txt").expect("File required");

    println!("Day 7 Result - Part 2: {:?}", solution(&input));
}

fn solution(input: &str) -> i64 {
    let mut crabs: Vec<i64> = input
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    crabs.sort();

    let mut positions = crabs.clone();
    positions.dedup();

    let mut lowest_cost = i64::MAX;
    for position in positions[0]..positions[positions.len() - 1] {
        let fuel = cost_of_fuel(&crabs, position);

        if fuel > lowest_cost {
            break;
        } else {
            lowest_cost = fuel
        };
    }

    lowest_cost
}

fn cost_of_fuel(crabs: &Vec<i64>, position: i64) -> i64 {
    let distance: Vec<i64> = crabs
        .iter()
        .map(|crab| {
            let dist = (crab - position).abs();
            (1 + dist) * dist / 2
        })
        .collect();
    distance.iter().sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn first() {
        assert_eq!(solution(INPUT), 168);
    }
}
