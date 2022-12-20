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
    let v_len = v.len(); // Caching this here b/c it'll be used a lot
    let mut indicies: Vec<usize> = (0..v_len).into_iter().collect();

    while let Some(idx) = indicies.first().copied() {
        indicies.drain(0..1); // rm first index element

        let shift_val = v[idx];

        // Skip 0 as it does not move
        if shift_val == 0 {
            continue;
        }

        // IK this is jank but at least it works ig
        let final_idx = if idx as i64 + shift_val == 0 {
            v_len - 1
        } else if idx as i64 + shift_val < 0 {
            v_len - 1 - ((idx as i64 + shift_val).unsigned_abs() as usize % v_len)
        } else if ((idx as i64 + shift_val) as usize) % v_len != (idx as i64 + shift_val) as usize {
            (((idx as i64 + shift_val) as usize) + 1) % v_len
        } else {
            (idx as i64 + shift_val) as usize
        };

        v.remove(idx);
        v.insert(final_idx, shift_val);

        for i in 0..indicies.len() {
            if indicies[i] <= final_idx
                || shift_val < 0 && indicies[i] >= idx && indicies[i] < final_idx
            {
                indicies[i] -= 1;
            }
        }
    }
}

fn nums(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
