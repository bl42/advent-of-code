use std::fs;

// https://adventofcode.com/2021/day/2
// Day 2: Dive!

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
    fn swim(&mut self, direction: &str, amount: i32) {
        match direction {
            "forward" => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
            "up" => {
                self.aim -= amount;
            }
            "down" => {
                self.aim += amount;
            }
            _ => panic!("unknown direction"),
        };
    }
    fn result(&self) -> i64 {
        let result: i64 = (self.horizontal * self.depth).into();
        println!("Day 2 Result: {}", result);
        result
    }
}

pub fn run() {
    let input = fs::read_to_string("./inputs/2.txt").expect("File required");
    let commands: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();

    let mut position = Position::new();

    commands.into_iter().for_each(|command| {
        let direction = command[0];
        let amount: i32 = command[1].parse().unwrap();

        position.swim(direction, amount);

        // println!("{:?} {} {}", position, direction, amount);
    });

    position.result();
}
