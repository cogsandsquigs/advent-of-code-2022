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
    let jetstream = Jetstream::new(input);

    print!("{}", 1 >> 1);

    todo!()
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
}
