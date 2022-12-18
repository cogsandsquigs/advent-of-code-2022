use advent_utils::{files::read, macros::solution};
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-19/input.test.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "19", part = "2")]
fn part_2(input: &str) -> i64 {
    todo!()
}

#[solution(day = "19", part = "1")]
fn part_1(input: &str) -> i64 {
    todo!()
}
