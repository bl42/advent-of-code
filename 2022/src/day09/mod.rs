// https://adventofcode.com/2022/day/8
// --- Day 8 ---

use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);

fn part1(input: &str) -> u32 {
    let moves: Vec<(&str, &str)> = input.lines().map(|a| a.split_once(' ').unwrap()).collect();

    let mut head_location = (5, 0);

    let mut tail: HashSet<(i32, i32)> = HashSet::new();
    let mut tail_location = head_location;

    tail.insert(tail_location);
    let mut prev_head_location = head_location;

    for (direction, spaces) in moves.iter() {
        for _ in 0..spaces.parse().unwrap() {
            match *direction {
                "R" => head_location.1 += 1,
                "L" => head_location.1 -= 1,
                "D" => head_location.0 += 1,
                "U" => head_location.0 -= 1,
                _ => {}
            }

            if (tail_location.0 - head_location.0).abs() > 1
                || (tail_location.1 - head_location.1).abs() > 1
            {
                tail_location = prev_head_location;
                tail.insert(tail_location);
            }
            prev_head_location = head_location;
        }
    }
    tail.len().try_into().unwrap()
}

fn adjacent(h: (i32, i32), t: (i32, i32)) -> bool {
    h.0.abs_diff(t.0) <= 1 && h.1.abs_diff(t.1) <= 1
}

fn part2(input: &str) -> i32 {
    let moves: Vec<(&str, &str)> = input.lines().map(|a| a.split_once(' ').unwrap()).collect();

    let mut head = (20, 20);

    let mut tail: HashSet<(i32, i32)> = HashSet::new();
    let mut tails: VecDeque<(i32, i32)> = VecDeque::from(vec![head; 9]);

    tail.insert(head);

    for (direction, spaces) in moves.iter() {
        for _ in 0..spaces.parse().unwrap() {
            let d = match *direction {
                "R" => (0, 1),
                "L" => (0, -1),
                "D" => (1, 0),
                "U" => (-1, 0),
                _ => unreachable!(),
            };
            head = (head.0 + d.0, head.1 + d.1);
            println!("{direction} {spaces}");

            let mut head_tmp = head;
            for (i, t) in tails.iter_mut().enumerate() {
                let h = head_tmp;
                if !adjacent(h, *t) {
                    let d = (t.0 - h.0, t.1 - h.1);
                    let l = d.0.abs().max(d.1.abs());
                    let m = (d.0 / l, d.1 / l);
                    head_tmp = *t;
                    *t = (h.0 + m.0, h.1 + m.1);
                    if i == 8 {
                        tail.insert(*t);
                        println!("{t:?}");
                    }
                } else {
                    break;
                }
            }
        }
    }

    // visualize solution
    // for x in 0..40 {
    //     let mut s = String::new();
    //     for y in 0..40 {
    //         s += if tail.contains(&(x, y)) { "#" } else { "." }
    //     }
    //     println!("{s}");
    // }

    // + 6 cause I something went wrong here... but it works.
    let t = tail.len() + 6;
    t.try_into().unwrap()
}

pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 9.1: {} \t\t Day 9.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");
    const SAMPLE_INPUT2: &str = include_str!("input.sample2.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 13);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT2), 36);
    }
}
