use advent_utils::files::read;
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() -> Result<()> {
    let input = read("day-13/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| packet(l).unwrap().1)
        .collect::<Vec<_>>();

    packets.push(Packet::List(vec![Packet::List(vec![Packet::Number(2)])]));
    packets.push(Packet::List(vec![Packet::List(vec![Packet::Number(6)])]));

    packets.sort_by(|a, b| {
        let order = correct_order(a, b);
        if order == Some(true) {
            std::cmp::Ordering::Less
        } else if order == Some(false) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    (packets
        .iter()
        .position(|p| p == &Packet::List(vec![Packet::List(vec![Packet::Number(2)])]))
        .unwrap()
        + 1)
        * (packets
            .iter()
            .position(|p| p == &Packet::List(vec![Packet::List(vec![Packet::Number(6)])]))
            .unwrap()
            + 1)
}

fn part_1(input: &str) -> usize {
    let packets = packet_pairs(input).unwrap().1;

    packets
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (left, right))| {
            if correct_order(left, right).unwrap_or(false) {
                acc + (i + 1)
            } else {
                acc
            }
        })
}

fn correct_order(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Packet::Number(left), Packet::Number(right)) => {
            if left == right {
                None
            } else {
                Some(left < right)
            }
        }
        (Packet::List(left), Packet::List(right)) => left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| correct_order(l, r))
            .find(|o| o.is_some())
            .unwrap_or(if left.len() == right.len() {
                None
            } else {
                Some(left.len() < right.len())
            }),
        (left, Packet::List(right)) => correct_order(
            &Packet::List(vec![left.clone()]),
            &Packet::List(right.to_vec()),
        ),
        (Packet::List(left), right) => correct_order(
            &Packet::List(left.to_vec()),
            &Packet::List(vec![right.clone()]),
        ),
    }
}

fn packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(tag("\n\n"), separated_pair(packet, tag("\n"), packet))(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    delimited(
        tag("["),
        separated_list0(tag(","), alt((packet, parse_number))),
        tag("]"),
    )(input)
    .map(|(i, v)| (i, Packet::List(v)))
}

fn parse_number(input: &str) -> IResult<&str, Packet> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input).map(|(i, n)| (i, Packet::Number(n)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}
