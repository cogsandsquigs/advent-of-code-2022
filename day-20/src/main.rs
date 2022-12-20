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

    println!("{:?}", nums);

    mix(&mut nums);

    println!("{:?}", nums);

    todo!()
}

fn mix(v: &mut Vec<i64>) {
    let v_len = v.len(); // Caching this here b/c it'll be used a lot
    let mut indicies: Vec<usize> = (0..v_len).into_iter().collect();

    while let Some(idx) = indicies.first().copied() {
        indicies.drain(0..1); // rm first index element

        let shift_val = v[idx];

        let final_idx = if idx as i64 + shift_val < 0 {
            v_len - (-(idx as i64 + shift_val)) as usize % v_len
        } else {
            (idx as i64 + shift_val) as usize % v_len
        };

        println!("{}", final_idx);

        v.remove(idx);
        v.insert(final_idx, shift_val);

        println!("{:?}", v);

        for i in 0..indicies.len() {
            if indicies[i] > idx && indicies[i] < final_idx {
                indicies[i] = indicies[i] as usize - 1;
            } else if indicies[i] > idx && indicies[i] > final_idx {
                indicies[i] = indicies[i] as usize + 1;
            }
        }
    }
}

fn nums(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
