use std::ops::{Range, RangeInclusive};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, error::ParseError, sequence::separated_pair,
};
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-04/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &String) -> usize {
    todo!()
}

fn part_1(input: &String) -> usize {
    let pairs: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = get_pairs(input);

    pairs
        .into_iter()
        .filter(|(a, b)| {
            a.start() <= b.start() && b.end() <= a.end()
                || a.start() >= b.start() && b.end() >= a.end()
        })
        .count()
}

fn get_pairs(input: &String) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    todo!()
}
