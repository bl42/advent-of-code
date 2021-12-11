use std::collections::HashSet;
use std::fs;
// https://adventofcode.com/2021
// Day 8: Seven Segment Search

pub fn run() {
    let input = fs::read_to_string("./inputs/8.txt").expect("File required");

    println!("Day 8 Result - Part 1: {:?}", part1(&input));
    println!("Day 8 Result - Part 2: {:?}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut found = 0;
    for line in input.lines() {
        let (_, output) = line.split_once('|').unwrap();
        for number in output.split(' ') {
            match number.len() {
                2 | 3 | 4 | 7 => {
                    found += 1;
                }
                _ => {}
            }
        }
    }

    found
}

// len(2) = 1
// len(3) = 7
// len(4) = 4
// len(7) = 8
// len(5) = 5 | 3 | 2
// len(6) = 6 | 9 | 0

// 7 is subset of *3*
// 4 is subset of 6
// 4+3 = *9*
// *0* has uniq -(1 & 7)
// *6* left over of len(5)

// *5* is subset of 6
// 2 is left over

fn part2(input: &str) -> i64 {
    let lines = input.lines();
    let mut total: i64 = 0;
    for line in lines {
        let (head, output) = line.split_once('|').unwrap();

        let decoded_head = decode_head_into_numbers(head);

        let mut number = String::new();

        for coded_number in output.trim().split(' ') {
            let hashmap = coded_number.chars().into_iter().collect::<HashSet<char>>();
            let found = decoded_head.iter().position(|coded_num| {
                if coded_num.as_ref().unwrap().len() == hashmap.len() {
                    return hashmap.is_superset(coded_num.as_ref().unwrap());
                }
                false
            });

            number += found.unwrap().to_string().as_str();
        }

        total += number.parse::<i64>().unwrap();
    }

    total
}

fn decode_head_into_numbers(head: &str) -> Vec<Option<HashSet<char>>> {
    let mut map: Vec<Option<HashSet<char>>> = vec![None; 10];

    let coded: Vec<HashSet<char>> = head
        .trim()
        .split(' ')
        .map(|coded| coded.chars().into_iter().collect::<HashSet<char>>())
        .collect();

    let mut rest = vec![];
    for code in coded {
        match code.len() {
            // len(2) = 1
            2 => {
                map[1] = Some(code);
            }
            // len(3) = 7
            3 => {
                map[7] = Some(code);
            }
            // len(4) = 4
            4 => {
                map[4] = Some(code);
            }
            // len(7) = 8
            7 => {
                map[8] = Some(code);
            }
            _ => rest.push(code),
        };
    }
    let coded = rest;
    let mut rest = vec![];

    for code in coded {
        if code.is_superset(map[7].as_ref().unwrap()) && code.len() == 5 {
            // 7 is subset of *3*
            map[3] = Some(code);
        } else if code.is_superset(map[4].as_ref().unwrap()) {
            // 4 is subset of *9*
            map[9] = Some(code);
        } else {
            rest.push(code);
        }
    }

    let coded = rest;
    let mut rest = vec![];

    for code in coded {
        if code.is_superset(map[7].as_ref().unwrap()) && code.len() == 6 {
            // 7 inside 0;
            map[0] = Some(code);
        } else if code.is_subset(map[9].as_ref().unwrap()) && code.len() == 5 {
            // 5 inside 9
            map[5] = Some(code);
        } else {
            rest.push(code);
        }
    }
    let coded = rest;

    for code in coded {
        match code.len() {
            5 => map[2] = Some(code),
            6 => map[6] = Some(code),
            _ => {}
        }
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn first() {
        assert_eq!(part1(INPUT), 26);
    }
    #[test]

    fn second() {
        assert_eq!(part2(INPUT), 61229);
    }
}
