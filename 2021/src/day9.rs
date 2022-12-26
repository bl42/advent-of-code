use std::fs;
// https://adventofcode.com/2021
// Day 9: Smoke Basin

pub fn run() {
    let input = fs::read_to_string("./inputs/9.txt").expect("File required");

    print!("Day 9 - Part 1: {:?}", part1(&input));
}

fn part1(input: &str) -> u32 {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|p| p.to_digit(10).unwrap()).collect())
        .collect();
    let mut risk = 0;

    for (x, row) in lines.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            let neighboors = {
                let top = if x > 0 {
                    lines.get(x - 1).unwrap().get(y)
                } else {
                    None
                };

                let bottom = if let Some(row) = lines.get(x + 1) {
                    row.get(y)
                } else {
                    None
                };

                let left = if y > 0 {
                    lines.get(x).unwrap().get(y - 1)
                } else {
                    None
                };

                let right = lines.get(x).unwrap().get(y + 1);

                vec![top, right, bottom, left]
            };
            let mut low_point = true;
            for neighboor in neighboors {
                low_point = if low_point == false {
                    false
                } else {
                    if let Some(v) = neighboor {
                        v > value
                    } else {
                        true
                    }
                };
            }

            if low_point {
                risk += value + 1
            }
        }
    }

    risk
}

fn part2(input: &str) -> u64 {
    let mut lines: Vec<Vec<(Option<u8>, u8)>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|p| (None, p.to_string().parse::<u8>().unwrap()))
                .collect()
        })
        .collect();

    let mut found: u8 = 0;
    for (x, row) in lines.clone().into_iter().enumerate() {
        for (y, item) in row.into_iter().enumerate() {
            let neighboors = {
                let &top = if x > 0 {
                    lines.get(x - 1).unwrap().get(y).unwrap()
                } else {
                    &(None, 9)
                };

                let &bottom = if let Some(row) = lines.get(x + 1) {
                    row.get(y).unwrap()
                } else {
                    &(None, 9)
                };

                let &left = if y > 0 {
                    lines.get(x).unwrap().get(y - 1).unwrap()
                } else {
                    &(None, 9)
                };

                let &right = lines.get(x).unwrap().get(y + 1).unwrap_or(&(None, 0));

                vec![top, right, bottom, left]
            };

            if item.1 != 9 {
                let mut next_to_basin = None;
                for (basin_num, _) in neighboors {
                    next_to_basin = if let Some(num) = basin_num {
                        Some(num)
                    } else {
                        next_to_basin
                    };
                }

                lines[x][y].0 = if next_to_basin.is_some() {
                    next_to_basin
                } else {
                    found += 1;
                    Some(found)
                }
            }
        }
    }

    println!("FOUND: {:?} ---- {:?}", found, lines);

    1134
}

fn search_neighboors() {}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn first() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn second() {
        assert_eq!(part2(INPUT), 1134);
    }
}
