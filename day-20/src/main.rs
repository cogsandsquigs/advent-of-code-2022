use advent_utils::{files::read, macros::solution};
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-20/input.test.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "20", part = "2")]
fn part_2(input: &str) -> i64 {
    todo!()
}

#[solution(day = "20", part = "1")]
fn part_1(input: &str) -> i64 {
    let mut nums = nums(input);

    mix(&mut nums);

    let zero_pos = nums.iter().position(|&x| x == 0).unwrap();

    nums[(1000 + zero_pos) % nums.len()]
        + nums[(2000 + zero_pos) % nums.len()]
        + nums[(3000 + zero_pos) % nums.len()]
}

fn mix(v: &mut Vec<i64>) {
    todo!()
}

fn nums(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
