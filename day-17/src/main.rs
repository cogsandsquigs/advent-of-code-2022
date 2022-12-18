use std::vec;

use advent_utils::{files::read, macros::solution};
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-17/input.test.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "17", part = "2")]
fn part_2(input: &str) -> i64 {
    todo!()
}

#[solution(day = "17", part = "1")]
fn part_1(input: &str) -> i64 {
    let mut jetstream = Jetstream::new(input);
    let mut chamber: Vec<usize> = vec![];
    let mut current_type = 0;

    println!("{}", 2_i32.leading_zeros() - (32 - 7));

    // for _ in 0..10 {
    //     // Drop the rock 3 times initially. Then we check if it intersects with any other rocks.
    //     // First, we get the 3 movements.
    //     let mut current = rock_types()[current_type].clone();
    //     // Make sure we update the current type.
    //     current_type += 1;
    //     current_type %= 5;

    //     let mut m = jetstream.next_n(3);
    // }

    todo!()
}

// Shift the rock left until it reaches the leftmost position. Note
// that the chamber is 7 units wide.
fn shift_rock_left(rock: Vec<usize>, shift: usize) -> Vec<usize> {
    let mut shifted = vec![];
    let max_w = max_width(rock.clone());

    if max_w + shift > 7 {}

    for r in rock {
        shifted.push(r << shift);
    }

    shifted
}

fn max_width(rock: Vec<usize>) -> usize {
    let mut max = 0;

    for mut r in rock {
        let mut width = 0;

        while r & 1 == 1 {
            width += 1;
            r >>= 1;
        }

        if width > max {
            max = width;
        }
    }

    max
}

fn rock_types() -> Vec<Vec<usize>> {
    vec![
        vec![0b0011110],
        vec![0b0001000, 0b0011100, 0b0001000],
        vec![0b0000100, 0b0000100, 0b0011100],
        vec![0b0010000, 0b0010000, 0b0010000, 0b0010000],
        vec![0b0011000, 0b0011000],
    ]
}

struct Jetstream {
    pattern: Vec<i32>,
    position: usize,
}

impl Jetstream {
    pub fn new(input: &str) -> Self {
        Self {
            pattern: input
                .chars()
                .map(|c| match c {
                    '>' => 1,
                    '<' => -1,
                    c => unreachable!("Bad input: unexpected character '{c}'"),
                })
                .collect(),
            position: 0,
        }
    }

    pub fn next(&mut self) -> i32 {
        let value = self.pattern[self.position];
        self.position = (self.position + value as usize) % self.pattern.len();
        value
    }

    pub fn next_n(&mut self, n: usize) -> i32 {
        let mut m = 0;

        for _ in 0..n {
            m += self.next();
        }

        m
    }
}
