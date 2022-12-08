// https://adventofcode.com/2022/day/7
// --- Day 7 ---

use std::collections::HashMap;

fn parse_directory(s: &str) -> HashMap<String, u32> {
    let commands: Vec<&str> = s.split("$ ").filter(|s| !s.is_empty()).collect();
    let mut current_path = vec![];
    let mut files: HashMap<String, u32> = HashMap::new();

    for (cmd, output) in commands.iter().map(|c| c.split_at(2)) {
        let output = output.trim();
        let mut dir = String::new();

        match (cmd, output) {
            ("cd", "..") => {
                current_path.pop();
            }
            ("cd", _) => {
                current_path.push(output);
            }
            ("ls", _) => {
                let folder_size: u32 = output
                    .lines()
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|line| {
                        let (size, file) = line.split_once(' ').unwrap();
                        return size.parse::<u32>().unwrap_or(0);
                    })
                    .into_iter()
                    .sum();

                let p = current_path.clone();
                for path in p {
                    if dir.is_empty() {
                        dir = "/".into();
                    } else {
                        dir = dir + path + "/";
                    }
                    if files.contains_key(&dir) {
                        let file_size = files.get_mut(&dir).unwrap();
                        *file_size += folder_size;
                    } else {
                        files.insert(dir.clone(), folder_size);
                    }
                }
            }
            _ => panic!("unknown '{cmd}'"),
        }
    }

    files
}

fn part1(input: &str) -> u32 {
    let files = parse_directory(input);
    files
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, &size)| size)
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let files = parse_directory(input);

    files
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, &size)| size)
        .sum::<u32>();

    let total = files.get("/").unwrap();

    let free_space = 70000000 - total;
    let needed_for_update = 30000000 - free_space;

    let (_, &size_to_remove) = files
        .iter()
        .filter(|(k, &v)| v > needed_for_update)
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    size_to_remove
}

pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 7.1: {} \t Day 7.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 95437);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 24933642);
    }
}
