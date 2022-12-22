use std::fmt::Display;

use advent_utils::{grid::Grid, macros::solution};
use itertools::Itertools;

fn main() {
    part_1();
    part_2();
}

#[solution(day = "22", part = "2")]
fn part_2(input: &str) -> i64 {
    todo!()
}

#[solution(day = "22", part = "1")]
fn part_1(input: &str) -> i64 {
    let monkey_map = monkey_map(input);

    todo!()
}

fn monkey_map(input: &str) -> MonkeyMap {
    let mut split = input.split("\n\n");
    let map = split.next().unwrap();
    let directions = split.next().unwrap();
    let longest_line_len = map.lines().max_by_key(|line| line.len()).unwrap().len();

    println!("{}", longest_line_len);

    MonkeyMap {
        map: Grid::try_from(
            map.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            ' ' => Tile::Empty,
                            '.' => Tile::Open,
                            '#' => Tile::Wall,
                            c => unreachable!("Unknown character {c}"),
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
        .unwrap(),
        directions: todo!(),
        facing: 0,
    }
}

#[derive(Debug, Clone)]
struct MonkeyMap {
    map: Grid<Tile>,
    directions: Vec<Path>,
    facing: usize,
}

impl Display for MonkeyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map.clone().into_iter() {
            for val in line {
                write!(
                    f,
                    "{}",
                    match val {
                        Tile::Empty => " ",
                        Tile::Open => ".",
                        Tile::Wall => "#",
                    }
                )?;
            }
        }

        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    Turn(Turn),
    Walk(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Right = -1,
    Left = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Open,
    Wall,
}
