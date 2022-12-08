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

fn part_2(input: &str) -> String {
    let mut supply = Supply::new(input);

    for (amount, from, to) in supply.moves {
        let from_len = supply.stacks[from - 1].len();
        let tail = supply.stacks[from - 1].split_off(from_len - amount);
        supply.stacks[to - 1].extend(tail.iter());
    }

    String::from_iter(supply.stacks.iter().map(|stack| stack.last().unwrap()))
}

fn part_1(input: &str) -> String {
    let mut supply = Supply::new(input);

    for (amount, from, to) in supply.moves {
        let from_len = supply.stacks[from - 1].len();
        let mut tail = supply.stacks[from - 1].split_off(from_len - amount);
        tail.reverse();
        supply.stacks[to - 1].extend(tail.iter());
    }

    String::from_iter(supply.stacks.iter().map(|stack| stack.last().unwrap()))
}

#[derive(Debug)]
struct Supply {
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

impl Supply {
    fn new(input: &str) -> Supply {
        let mut supply: Supply = Supply {
            stacks: vec![],
            moves: vec![],
        };

        let (input, (mut stack_crates, _)) = stack_parser(input).unwrap();

        // Reverse them so we can traverse them and push/pop them on as needed
        stack_crates.reverse();

        for crates in stack_crates {
            for (idx, cr) in crates.iter().enumerate() {
                if supply.stacks.is_empty() || supply.stacks.len() < idx + 1 {
                    supply.stacks.push(vec![]);
                }

                let char = cr.chars().next().unwrap();

                if char != ' ' {
                    supply.stacks[idx].push(char);
                }
            }
        }

        supply.moves = moves_parser(input).unwrap().1;

        supply
    }
}

fn moves_parser(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    let (input, _) = tag("\n\n")(input)?;
    separated_list1(tag("\n"), move_parser)(input)
}

fn move_parser(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, _) = tag("move ")(input)?;

    let (input, amount_str) = digit1(input)?;
    let amount: usize = amount_str.parse().unwrap();

    let (input, _) = tag(" from ")(input)?;

    let (input, from_str) = digit1(input)?;
    let from: usize = from_str.parse().unwrap();

    let (input, _) = tag(" to ")(input)?;

    let (input, to_str) = digit1(input)?;
    let to: usize = to_str.parse().unwrap();

    Ok((input, (amount, from, to)))
}

fn stack_parser(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<&str>)> {
    // Get the stacks themselves
    let empty = tag("   ");
    let stack_crate = delimited::<&str, _, _, _, _, _, _, _>(char('['), take(1_usize), char(']'));
    let line = separated_list1(char(' '), alt((stack_crate, empty)));

    separated_pair(
        separated_list1(char('\n'), line),
        char('\n'),
        separated_list1(char(' '), delimited(char(' '), digit1, char(' '))),
    )(input)
}
