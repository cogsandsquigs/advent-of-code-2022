use std::fmt::Display;

use advent_utils::{grid::Grid, macros::solution, point::Point};
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

    println!("{}", monkey_map);

    todo!()
}

fn walk(map: &mut MonkeyMap) {
    for path in &map.directions {
        match path {
            Path::Turn(x) => match x {
                Turn::Right => map.facing = (map.facing + 1) % 4,
                Turn::Left => map.facing = if map.facing == 0 { 4 } else { map.facing - 1 },
            },
            Path::Walk(x) => {
                for i in 0..=*x {
                    todo!()
                }
            }
        }
    }
}

fn monkey_map(input: &str) -> MonkeyMap {
    let mut split = input.split("\n\n");
    let map = split.next().unwrap();
    let longest_line_len = map.lines().max_by_key(|line| line.len()).unwrap().len();

    MonkeyMap {
        map: Grid::try_from(
            map.lines()
                .map(|line| {
                    let mut v = line
                        .chars()
                        .map(|c| match c {
                            ' ' => Tile::Empty,
                            '.' => Tile::Open,
                            '#' => Tile::Wall,
                            c => unreachable!("Unknown character {c}"),
                        })
                        .collect_vec();

                    if v.len() != longest_line_len {
                        v.append(&mut vec![Tile::Empty; longest_line_len - v.len()])
                    }

                    v
                })
                .collect_vec(),
        )
        .unwrap(),
        directions: split
            .next()
            .unwrap()
            .split_inclusive(|c| c == 'L' || c == 'R')
            .map(|part| {
                let last = part.chars().nth(part.len() - 1).unwrap();
                if last == 'L' || last == 'R' {
                    vec![
                        Path::Walk(part[0..part.len() - 1].parse().unwrap()),
                        if last == 'L' {
                            Path::Turn(Turn::Left)
                        } else {
                            Path::Turn(Turn::Right)
                        },
                    ]
                } else {
                    vec![Path::Walk(part.parse().unwrap())]
                }
            })
            .collect::<Vec<Vec<Path>>>()
            .concat(),
        facing: 0,
        at: todo!(),
    }
}

#[derive(Debug, Clone)]
struct MonkeyMap {
    map: Grid<Tile>,
    directions: Vec<Path>,
    facing: usize,
    at: Point<usize>,
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

            writeln!(f, "")?;
        }

        writeln!(f, "")?;

        for path in &self.directions {
            write!(
                f,
                "{}",
                match path {
                    Path::Turn(t) => match t {
                        Turn::Left => "L",
                        Turn::Right => "R",
                    }
                    .to_string(),

                    Path::Walk(x) => x.to_string(),
                }
            )?;
        }

        writeln!(f, "")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    Turn(Turn),
    Walk(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Right = 1,
    Left = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Open,
    Wall,
}
