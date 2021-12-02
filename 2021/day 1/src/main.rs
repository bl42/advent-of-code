use std::fs;

// https://adventofcode.com/2021/day/1
// Day 1: Sonar Sweep

fn main() {
    let input = fs::read_to_string("input.txt").expect("File required");

    let list_of_depths = input
        .split("\n")
        .into_iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut depths_grped = Vec::new();

    for (pos, depth) in list_of_depths.iter().enumerate() {
        let mut v: u32 = *depth;

        if pos + 1 < list_of_depths.len() {
            v += list_of_depths[pos + 1];
        }
        if pos + 2 < list_of_depths.len() {
            v += list_of_depths[pos + 2];
        }

        depths_grped.push(v);
    }

    let mut count = 0;
    let mut prev: u32 = 0;

    depths_grped.iter().for_each(|current| {
        if prev == 0 {
            println!("{:?} (no change)", current);
        } else if current > &prev {
            println!("{:?} (increased)", current);
            count += 1
        } else if &prev > current {
            println!("{:?} (decreased)", current);
            // count -= 1
        };
        prev = *current;
    });

    println!("the answer is: {}", count);
}
