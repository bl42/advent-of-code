use std::fs;

// https://adventofcode.com/2021/day/3
// Day 3: Binary Diagnostic

pub fn run() {
    let input = fs::read_to_string("./inputs/3.txt").expect("File required");
    // part1(&input);
    part2(&input);
}

fn part2(input: &str) {
    let commands = input.lines().collect::<Vec<&str>>();

    let sample = commands.clone().first().unwrap().chars();

    let mut oxygen = commands.clone();
    let mut c02 = commands.clone();

    for i in sample.into_iter().enumerate() {
        oxygen = find_common(&mut oxygen, i.0, true);
        c02 = find_common(&mut c02, i.0, false);
    }
    println!("{:?}", oxygen);
    println!("{:?}", c02);

    let result = usize::from_str_radix(oxygen.first().unwrap(), 2).unwrap()
        * usize::from_str_radix(c02.first().unwrap(), 2).unwrap();

    println!("Day 3 Result:  {:?}", result);
}

fn find_common<'a>(commands: &mut Vec<&'a str>, position: usize, tie_up: bool) -> Vec<&'a str> {
    let cmd = commands.clone();

    if cmd.len() == 1 {
        return cmd;
    }

    let indexed = cmd
        .into_iter()
        .map(|line| (line.chars().nth(position).unwrap(), line))
        .collect::<Vec<(char, &str)>>();

    let total = &indexed.len();

    let ones = indexed
        .clone()
        .into_iter()
        .filter(|item| item.0 == '1')
        .collect::<Vec<(char, &str)>>()
        .len();

    let zeros = total - ones;

    let common = match tie_up {
        true => {
            if ones >= zeros {
                '1'
            } else {
                '0'
            }
        }
        false => {
            if ones < zeros {
                '1'
            } else {
                '0'
            }
        }
    };

    let filtered = indexed
        .into_iter()
        .filter(|item| item.0 == common)
        .map(|(_, line)| line)
        .collect::<Vec<&str>>();
    // let common = indexed.reduce(|accum, bit| (accum.0 + bit.0, ""));

    filtered
}

fn part1(input: &str) {
    let mut gamma_check = [0, 0, 0, 0, 0];
    let mut epsilon_check = gamma_check.clone();

    let commands = input.split("\n").into_iter().for_each(|line| {
        for (i, ln) in line.chars().enumerate() {
            match ln {
                '1' => {
                    gamma_check[i] += 1;
                    epsilon_check[i] -= 1;
                }
                '0' => {
                    gamma_check[i] -= 1;
                    epsilon_check[i] += 1;
                }
                _ => panic!("Bad input, expecting 1 or 0"),
            }
        }
    });
    let gamma: String = gamma_check
        .iter()
        .map(|item| if item >= &1 { '1' } else { '0' })
        .collect();
    let epsilon: String = epsilon_check
        .iter()
        .map(|item| if item >= &1 { '1' } else { '0' })
        .collect();

    let gamma_value = usize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_value = usize::from_str_radix(epsilon.as_str(), 2).unwrap();

    println!("answer {:?} ", gamma_value * epsilon_value);
}
