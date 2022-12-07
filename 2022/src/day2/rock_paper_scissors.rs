#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    pub fn new(letter: char) -> Move {
        match letter {
            'A' => Self::Rock,
            'X' => Self::Rock,
            'B' => Self::Paper,
            'Y' => Self::Paper,
            'C' => Self::Scissors,
            'Z' => Self::Scissors,
            _ => panic!("Not a valid move"),
        }
    }
    pub fn wins_to(self) -> Move {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
    pub fn loses_to(self) -> Move {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    pub fn draw_to(self) -> Move {
        self
    }
}

pub struct Game {
    left: Move,
    right: Move,
}

impl Game {
    pub fn new(left: Move, right: Move) -> Game {
        Game { left, right }
    }

    pub fn draw(&self) -> bool {
        self.left == self.right
    }

    pub fn winner(player: Move, opponent: Move) -> bool {
        matches!(
            (player, opponent),
            (Move::Paper, Move::Rock)
                | (Move::Rock, Move::Scissors)
                | (Move::Scissors, Move::Paper)
        )
    }

    pub fn points(self) -> usize {
        if Game::winner(self.right, self.left) {
            return self.right as usize + 6;
        }

        if self.draw() {
            return self.right as usize + 3;
        }

        self.right as usize
    }
}
