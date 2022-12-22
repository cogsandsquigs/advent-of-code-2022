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
fn part_1(input: &str) -> usize {
    let mut monkey_map = monkey_map(input);

    walk(&mut monkey_map);

    (monkey_map.at.x + 1) * 1000 + (monkey_map.at.y + 1) * 6 + monkey_map.facing
}

fn walk(map: &mut MonkeyMap) {
    for path in &map.directions {
        match path {
            Path::Turn(x) => match x {
                Turn::Right => map.facing = (map.facing + 1) % 4,
                Turn::Left => map.facing = if map.facing == 0 { 4 } else { map.facing - 1 },
            },
            Path::Walk(x) => {
                for _ in 0..*x {
                    let next_pt = match map.facing {
                        0 => Point::new(map.at.x + 1, map.at.y),
                        1 => Point::new(map.at.x, map.at.y + 1),
                        2 => Point::new(map.at.x - 1, map.at.y),
                        3 => Point::new(map.at.x, map.at.y - 1),
                        x => unreachable!("Cannot face in direction {x}"),
                    };

                    map.at = if next_pt.x >= map.map.width - 1
                        || next_pt.y >= map.map.height - 1
                        || next_pt.x == 0
                        || next_pt.y == 0
                    {
                        wrap_around(map, next_pt)
                    } else {
                        println!(
                            "{} {} {} {}",
                            map.map.width, map.map.height, next_pt.x, next_pt.y
                        );
                        match map.map[next_pt] {
                            // If it's a wall, just skip to the next direction
                            Tile::Wall => break,

                            // If it's ok, just go to the next point
                            Tile::Open => next_pt,

                            // Otherwise, wrap around
                            Tile::Empty => wrap_around(map, next_pt),
                        }
                    };
                }
            }
        }
    }
}

fn wrap_around(map: &MonkeyMap, point: Point<usize>) -> Point<usize> {
    match map.facing {
        0 => Point::new(
            point.x,
            map.map
                .clone()
                .into_iter()
                .nth(point.y)
                .unwrap()
                .into_iter()
                .find_position(|x| x == &Tile::Open)
                .unwrap()
                .0,
        ),
        1 => Point::new(
            map.map
                .clone()
                .into_iter()
                .find_position(|x| x[point.x] == Tile::Open)
                .unwrap()
                .0,
            point.y,
        ),
        2 => Point::new(
            point.x,
            map.map
                .clone()
                .into_iter()
                .nth(point.y)
                .unwrap()
                .into_iter()
                .rev()
                .enumerate()
                .find(|(_, x)| x == &Tile::Open)
                .unwrap()
                .0,
        ),
        3 => Point::new(
            map.map
                .clone()
                .into_iter()
                .enumerate()
                .rev()
                .find(|(_, x)| x[point.x] == Tile::Open)
                .unwrap()
                .0,
            point.y,
        ),
        x => unreachable!("Cannot face in direction {x}"),
    }
}

fn monkey_map(input: &str) -> MonkeyMap {
    let mut split = input.split("\n\n");
    let map = split.next().unwrap();
    let longest_line_len = map.lines().max_by_key(|line| line.len()).unwrap().len();
    let map = Grid::try_from(
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
    .unwrap();

    let at = Point::new(
        0,
        map.clone()
            .into_iter()
            .next()
            .unwrap()
            .into_iter()
            .find_position(|x| x == &Tile::Open)
            .unwrap()
            .0,
    );

    MonkeyMap {
        map,
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
        at,
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
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Open,
    Wall,
}
