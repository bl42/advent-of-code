// https://adventofcode.com/2022/day/2
// --- Day 2: Rock Paper Scissors ---

mod rock_paper_scissors;
use rock_paper_scissors::*;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Day 2.1: {} \t\t Day 2.2: {}", part1(&input), part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let letters: Vec<char> = line.chars().into_iter().collect();
            let (left_letter, right_letter) = (letters[0], letters[2]);

            let game = Game::new(Move::new(left_letter), Move::new(right_letter));

            game.points()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let letters: Vec<char> = line.chars().into_iter().collect();
            let (left_letter, right_letter) = (letters[0], letters[2]);

            let their_move = Move::new(left_letter);

            let my_move = {
                match right_letter {
                    'X' => their_move.wins_to(),
                    'Y' => their_move.draw_to(),
                    'Z' => their_move.loses_to(),
                    _ => panic!("Invalid Result"),
                }
            };

            let game = Game::new(their_move, my_move);

            game.points()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 15);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 12);
    }
}
