// https://adventofcode.com/2022/day/8
// --- Day 8 ---

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Grid {
    fn visible(&self, x: i32, y: i32) -> bool {
        self.visible_by_direction(x, y, Direction::Up)
            || self.visible_by_direction(x, y, Direction::Down)
            || self.visible_by_direction(x, y, Direction::Right)
            || self.visible_by_direction(x, y, Direction::Left)
    }
    fn scenic_score(&self, x: i32, y: i32) -> i32 {
        let &start = self.find(x, y).unwrap();
        let mut total: Vec<Option<i32>> = vec![None; 4];
        for i in 1.. {
            // UP
            if total[0].is_none() && !self.greather_than(start, x - i, y) {
                total[0] = if self.find(x - i, y).is_none() {
                    Some(i - 1)
                } else {
                    Some(i)
                };
            }
            // RIGHT
            if total[1].is_none() && !self.greather_than(start, x, y + i) {
                total[1] = if self.find(x, y + i).is_none() {
                    Some(i - 1)
                } else {
                    Some(i)
                };
            }
            // DOWN
            if total[2].is_none() && !self.greather_than(start, x + i, y) {
                total[2] = if self.find(x + i, y).is_none() {
                    Some(i - 1)
                } else {
                    Some(i)
                };
            }
            // LEFT
            if total[3].is_none() && !self.greather_than(start, x, y - i) {
                total[3] = if self.find(x, y - i).is_none() {
                    Some(i - 1)
                } else {
                    Some(i)
                };
            }

            if !total.contains(&None) {
                break;
            }
        }

        total
            .iter()
            .fold(1, |acc, x| if let Some(val) = x { acc * val } else { acc })
    }

    fn greather_than(&self, value: u32, x: i32, y: i32) -> bool {
        let Some(&val) = self.find(x, y) else {
	 		 return false;
	    };

        value > val
    }

    fn visible_by_direction(&self, x: i32, y: i32, direction: Direction) -> bool {
        let start = self.find(x, y).unwrap();

        match direction {
            Direction::Down => {
                for i in x.. {
                    let Some(val) = self.find(i+1, y) else {
                 		 return true;
                    };
                    if *val >= *start {
                        return false;
                    };
                }
            }
            Direction::Up => {
                for i in 0..x {
                    let Some(val) = self.find(i, y) else {
                 		 return true;
                    };
                    if *val >= *start {
                        return false;
                    };
                }
            }
            Direction::Right => {
                for i in y.. {
                    let Some(val) = self.find(x, i+1) else {
                 		 return true;
                    };
                    if *val >= *start {
                        return false;
                    };
                }
            }
            Direction::Left => {
                for i in 0..y {
                    let Some(val) = self.find(x, i) else {
                 		 return true;
                    };
                    if *val >= *start {
                        return false;
                    };
                }
            }
        };
        true
    }

    fn find(&self, x: i32, y: i32) -> Option<&u32> {
        if x.is_negative() || y.is_negative() {
            return None;
        };
        let Some(row) = self.0.get(x as usize) else {
       		return None;
        };

        let Some(val) = row.get(y as usize) else {
        	return None;
        };

        Some(val)
    }
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|num| char::to_digit(num, 10).unwrap())
                .collect()
        })
        .collect();

    let grid = Grid(grid);

    let mut count = 0;
    for (x, row) in grid.0.iter().enumerate().peekable() {
        for (y, val) in row.iter().enumerate().peekable() {
            let x = x as i32;
            let y = y as i32;

            let is_visible = grid.visible(x, y);

            if is_visible {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<u32>> = input
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|num| char::to_digit(num, 10).unwrap())
                .collect()
        })
        .collect();

    let grid = Grid(grid);

    let mut highest_score = 0;

    for (x, row) in grid.0.iter().enumerate().peekable() {
        for (y, v) in row.iter().enumerate().peekable() {
            let x = x as i32;
            let y = y as i32;
            let score = grid.scenic_score(x, y);

            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 8.1: {} \t\t Day 8.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 21);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 8);
    }
}
