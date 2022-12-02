use anyhow::Result;
use utils::files::read_file_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// Returns the score you would get if you played this move
    /// against the `other` move, using puzzle 2 rules.
    pub fn calculate_correct_match(&self, other: &Move) -> usize {
        Self::correct_move(self, other).calculate_match(other)
    }

    /// Returns the score you would get if you played this move
    /// against the `other` move.
    pub fn calculate_match(&self, other: &Move) -> usize {
        if self.beats(other) {
            self.score() + 6
        } else if self.draws(other) {
            self.score() + 3
        } else {
            self.score()
        }
    }

    /// Gets the correct move according to puzzle 2 rules
    pub fn correct_move(instruction: &Move, other: &Move) -> Move {
        match instruction {
            // Lose
            Move::Rock => match other {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Move::Paper => *other,
            Move::Scissors => match other {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        }
    }

    /// Checks if this move beats the `other` move
    pub fn beats(&self, other: &Move) -> bool {
        match self {
            Move::Rock => other == &Move::Scissors,
            Move::Paper => other == &Move::Rock,
            Move::Scissors => other == &Move::Paper,
        }
    }

    /// Checks if this move draws to the `other` move.
    pub fn draws(&self, other: &Move) -> bool {
        self == other
    }

    /// Gets the score of playing this move
    pub fn score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            v => Err(format!("Cannot convert {} to a move", v)),
        }
    }
}

fn main() -> Result<()> {
    let input = read_file_string("day-02/input.txt")?;

    // Get all the moves
    let turns: Vec<(Move, Move)> = input
        .split('\n')
        .map(|s| {
            (
                s.chars().nth(0).unwrap().try_into().unwrap(),
                s.chars().nth(2).unwrap().try_into().unwrap(),
            )
        })
        .collect();

    println!(
        "Puzzle 1 answer: {}",
        turns
            .iter()
            .fold(0, |acc, x| acc + x.1.calculate_match(&x.0))
    );

    println!(
        "Puzzle 2 answer: {}",
        turns
            .iter()
            .fold(0, |acc, x| acc + x.1.calculate_correct_match(&x.0))
    );

    Ok(())
}
