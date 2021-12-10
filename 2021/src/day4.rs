use std::fs;

// https://adventofcode.com/2021/day/3
// Day 4: Giant Squid

pub fn run() {
    let input = fs::read_to_string("./inputs/4.txt").expect("File required");

    println!("Day 4 - part 1: {:?}", part1(&input));
    println!("Day 4 - part 2: {:?}", part2(&input));
}

#[derive(Debug)]
struct Bingo {
    cards: Vec<Card>,
    winners: i32,
}

impl Bingo {
    fn new(cards: Vec<Card>) -> Bingo {
        Bingo { cards, winners: 0 }
    }
    fn call(&mut self, number: i32) {
        for card in self.cards.iter_mut() {
            if !card.winner {
                card.mark(number);
                if card.winner {
                    self.winners += 1;
                    card.place = Some(self.winners);
                }
            }
        }
    }
    fn winner(&self) -> Option<&Card> {
        self.cards.iter().filter(|card| card.winner).nth(0)
    }
    fn last(&self) -> Option<&Card> {
        if self.cards.iter().filter(|card| !card.winner).count() != 0 {
            return None;
        }
        self.cards.iter().reduce(|accum, card| {
            if Some(accum.place) < Some(card.place) {
                card
            } else {
                accum
            }
        })
    }
}

#[derive(Debug, Clone)]
struct Card {
    numbers: Vec<Vec<(i32, bool)>>,
    winner: bool,
    place: Option<i32>,
}

impl Card {
    fn from(raw: &str) -> Card {
        let numbers: Vec<Vec<(i32, bool)>> = raw
            .replace("  ", " ")
            .split("\n")
            .into_iter()
            .map(|row| {
                row.trim()
                    .split(' ')
                    .map(|num| (num.parse::<i32>().unwrap(), false))
                    .collect()
            })
            .collect();
        Card {
            numbers,
            winner: false,
            place: None,
        }
    }
    fn mark(&mut self, number: i32) {
        'numbers: for row in self.numbers.iter_mut() {
            for num in row.iter_mut() {
                if num.0 == number {
                    num.1 = true;

                    break 'numbers;
                }
            }
        }
        let mut columns = [0; 5];
        let mut rows = [0; 5];
        for (ic, row) in self.numbers.iter().enumerate() {
            for (ir, num) in row.iter().enumerate() {
                if num.1 == true {
                    columns[ic] += 1;
                    rows[ir] += 1;
                }
            }
        }
        if rows.iter().filter(|&&v| v == 5).count() != 0
            || columns.iter().filter(|&&v| v == 5).count() != 0
        {
            self.winner = true;
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut data: Vec<&str> = input.split("\n\n").collect();
    let bingo_numbers: Vec<i32> = data
        .remove(0)
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let cards: Vec<Card> = data.into_iter().map(|card| Card::from(card)).collect();

    let mut bingo = Bingo::new(cards);

    let mut result = 0;

    for number in bingo_numbers {
        bingo.call(number);

        if let Some(card) = bingo.winner() {
            let mut unused = 0;

            for row in &card.numbers {
                for num in row {
                    if !num.1 {
                        unused += num.0;
                    }
                }
            }

            result = number * unused;
            break;
        }
    }

    result
}

fn part2(input: &str) -> i32 {
    let mut data: Vec<&str> = input.split("\n\n").collect();
    let bingo_numbers: Vec<i32> = data
        .remove(0)
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let cards: Vec<Card> = data.into_iter().map(|card| Card::from(card)).collect();

    let mut bingo = Bingo::new(cards);

    let mut result = 0;

    for number in bingo_numbers {
        bingo.call(number);

        if let Some(card) = bingo.last() {
            let mut unused = 0;

            for row in &card.numbers {
                for num in row {
                    if !num.1 {
                        unused += num.0;
                    }
                }
            }

            result = number * unused;
            break;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1,0

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn first() {
        assert_eq!(part1(INPUT), 4512);
    }

    #[test]
    fn second() {
        assert_eq!(part2(INPUT), 1924);
    }
}
