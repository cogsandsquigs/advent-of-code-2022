use std::collections::HashMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{
        complete::{alpha1, digit1, one_of},
        streaming::char,
    },
    error::{ErrorKind, ParseError},
    multi::{count, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-05/input.test.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn part_1(input: &str) -> usize {
    let stacks = Stacks::new(input);

    todo!()
}

struct Stacks {
    stacks: Vec<Vec<char>>,
    moves: Vec<(
        // Number of crates to move
        usize,
        // From which stack to move them
        usize,
        // Which stack to move them to
        usize,
    )>,
}

impl Stacks {
    fn new(input: &str) -> Stacks {
        let stacks: Stacks = Stacks {
            stacks: vec![],
            moves: vec![],
        };

        let (_, (mut stack_crates, stack_idxs)) = stack_parser(input).unwrap();

        // Reverse them so we can traverse them and push/pop them on as needed
        stack_crates.reverse();

        for crates in stack_crates {
            for (idx, cr) in crates.into_iter().enumerate() {
                stacks
            }
        }

        stacks
    }
}

fn stack_parser(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<&str>)> {
    // Get the stacks themselves
    let empty = tag("   ");
    let stack_crate =
        delimited::<&str, _, _, _, _, _, _, _>(char('['), take(1 as usize), char(']'));
    let line = separated_list1(char(' '), alt((stack_crate, empty)));

    separated_pair(
        separated_list1(char('\n'), line),
        char('\n'),
        separated_list1(char(' '), delimited(char(' '), digit1, char(' '))),
    )(input)
}
