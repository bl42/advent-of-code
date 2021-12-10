use std::fs;

mod ocean;

// https://adventofcode.com/2021/day/3
// Day 5: Hydrothermal Venture

pub fn run() {
    let input = fs::read_to_string("./inputs/5.txt").expect("File required");

    println!("Day 5 - Part 1 - Answer: {:?}", part1(&input));
    println!("Day 5 - Part 2 - Answer: {:?}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut ocean_floor: ocean::Floor = input.parse().unwrap();

    ocean_floor.find_horizontal_segments();
    ocean_floor.find_vertical_segments();

    let answer = ocean_floor.vents_overlap();

    answer
}

fn part2(input: &str) -> i32 {
    let mut ocean_floor: ocean::Floor = input.parse().unwrap();

    ocean_floor.find_horizontal_segments();
    ocean_floor.find_vertical_segments();
    ocean_floor.find_diagonal_segments();

    let answer = ocean_floor.vents_overlap();

    answer
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn first() {
        assert_eq!(part1(INPUT), 5);
    }

    #[test]
    fn second() {
        assert_eq!(part2(INPUT), 12);
    }
}
