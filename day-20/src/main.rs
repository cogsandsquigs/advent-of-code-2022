use std::collections::VecDeque;

use advent_utils::{files::read, macros::solution};
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-20/input.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "20", part = "2")]
fn part_2(input: &str) -> i64 {
    let nums = mix_n(nums(input).iter().map(|num| num * 811589153).collect(), 10);

    let zero_pos = nums.iter().position(|&x| x == 0).unwrap();

    nums[(1000 + zero_pos) % nums.len()]
        + nums[(2000 + zero_pos) % nums.len()]
        + nums[(3000 + zero_pos) % nums.len()]
}

#[solution(day = "20", part = "1")]
fn part_1(input: &str) -> i64 {
    let nums = mix_n(nums(input), 1);

    let zero_pos = nums.iter().position(|&x| x == 0).unwrap();

    nums[(1000 + zero_pos) % nums.len()]
        + nums[(2000 + zero_pos) % nums.len()]
        + nums[(3000 + zero_pos) % nums.len()]
}

fn mix_n(list: Vec<i64>, n: usize) -> Vec<i64> {
    let mut enumerated: VecDeque<(usize, i64)> = list.into_iter().enumerate().collect();
    let len = enumerated.len(); // Caching this here

    for _ in 0..n {
        for num in 0..len {
            let idx = enumerated.iter().position(|(i, _)| i == &num).unwrap();
            let val = enumerated[idx].1;
            let final_idx = (idx as i64 + val).rem_euclid(len as i64 - 1) as usize
                + if
                // If we somehow are at the beginning of the list, we actually need to go to the
                // last spot, as that is the place where the list wraps around.
                ((idx as i64 + val).rem_euclid(len as i64 - 1)) == 0 {
                    len - 1
                } else {
                    0
                };

            enumerated.remove(idx);
            enumerated.insert(final_idx, (num, val));
        }
    }

    enumerated.into_iter().map(|(_, x)| x).collect()
}

fn nums(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
