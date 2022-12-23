use advent_utils::{grid::Grid, macros::solution, point::Point};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

fn main() {
    part_1();
    part_2();
}

#[solution(day = "23", part = "2")]
fn part_2(input: &str) -> i64 {
    todo!()
}

#[solution(day = "23", part = "1")]
fn part_1(input: &str) -> i64 {
    let elves = elves(input);
    let moving_to: HashMap<Point<usize>, Vec<Point<usize>>> = HashMap::new();
    // Position of where to check:
    // north -> 2
    // south -> 3
    // west -> 0
    // east -> 1
    let position_order: VecDeque<usize> = vec![2, 3, 0, 1].into();

    todo!()
}

fn second_half(
    elves: &mut Grid<Tile>,
    moving_to: &mut HashMap<Point<usize>, Vec<Point<usize>>>,
    position_order: &mut VecDeque<usize>,
) {
    // Clear the field so that new elves can move in
    elves.replace(|x| x == &Tile::Elf, Tile::Open);

    // Get rid of any spots where there are multiple elves moving to the same place
    moving_to.retain(|_, v| {
        if v.len() <= 1 {
            // Put back the elves that we won't move
            for elf in v {
                elves[*elf] = Tile::Elf
            }

            true
        } else {
            false
        }
    });

    // Shift all the positions
    position_order.rotate_left(1);

    // Now set the elves back in
    moving_to
        .iter()
        .for_each(|(point, _)| elves.set(*point, Tile::Elf));

    // Finally, clear the `moving_to` map, as we don't need it full anymore
    moving_to.clear();
}

fn first_half(
    elves: &Grid<Tile>,
    moving_to: &mut HashMap<Point<usize>, Vec<Point<usize>>>,
    position_order: &VecDeque<usize>,
) {
    todo!()
}

fn elves(input: &str) -> Grid<Tile> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Open,
                    '#' => Tile::Elf,
                    c => unreachable!("unknown character {c}"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
        .try_into()
        .unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Open,
    Elf,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Open => write!(f, "."),
            Tile::Elf => write!(f, "#"),
        }
    }
}
