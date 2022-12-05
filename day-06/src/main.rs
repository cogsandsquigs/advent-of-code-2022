use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{complete::digit1, streaming::char},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-05/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn part_1(input: &str) -> usize {
    todo!()
}
