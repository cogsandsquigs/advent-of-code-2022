use std::collections::VecDeque;

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
    let mut nums = nums(input);

    nums.iter_mut().for_each(|num| {
        *num *= 811589153;
    });

    mix_n(&mut nums, 10);

    let zero_pos = nums.iter().position(|&x| x == 0).unwrap();

    nums[(1000 + zero_pos) % nums.len()]
        + nums[(2000 + zero_pos) % nums.len()]
        + nums[(3000 + zero_pos) % nums.len()]
}

#[solution(day = "20", part = "1")]
fn part_1(input: &str) -> i64 {
    let mut nums = nums(input);

    mix_n(&mut nums, 1);

    let zero_pos = nums.iter().position(|&x| x == 0).unwrap();

    nums[(1000 + zero_pos) % nums.len()]
        + nums[(2000 + zero_pos) % nums.len()]
        + nums[(3000 + zero_pos) % nums.len()]
}

fn mix_n(list: &mut VecDeque<i64>, n: usize) {
    let idxs: Vec<usize> = (0..list.len()).into_iter().collect();

    for _ in 0..n {
        mix(list, idxs.clone());
        println!("{list:?}");
    }
}

fn mix(list: &mut VecDeque<i64>, mut idxs: Vec<usize>) {
    let len = list.len();

    while let Some(idx) = idxs.first().copied() {
        idxs.drain(0..1); // Remove first idx

        let shift = list.remove(idx).unwrap();

        let final_idx = if idx as i64 + shift > 0 {
            ((idx as i64 + shift) as usize) % (len - 1)
        } else {
            len - 1 + ((idx as i64 + shift) as usize) % (len - 1)
        };

        list.insert(final_idx, shift);

        for i in 0..idxs.len() {
            if idxs[i] <= final_idx && idxs[i] >= idx {
                idxs[i] -= 1;
            }
        }
    }
}

fn nums(input: &str) -> VecDeque<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
