use advent_utils::{files::read, point::Point};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = read("day-14/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let mut ceiling = ceiling_map(input);
    let mut sand_falls = 0;
    let barrier = ceiling.iter().map(|p| p.y).max().unwrap() + 2;
    loop {
        let mut lowest = get_lowest_point_with_barrier(&Point::new(500, 0), &ceiling, barrier);

        // Traverse down the ceiling until we reach the bottom.
        loop {
            if lowest.y == barrier - 1 {
                break;
            } else if let Some(point) = can_go_left(&lowest, &ceiling) {
                lowest = point;
            } else if let Some(point) = can_go_right(&lowest, &ceiling) {
                lowest = point;
            } else {
                break;
            }

            lowest = get_lowest_point_with_barrier(&lowest, &ceiling, barrier);
        }

        if lowest == Point::new(500, 0) {
            break;
        }

        ceiling.insert(lowest);
        sand_falls += 1;

        print_ceiling_map(&ceiling);
    }

    sand_falls + 1 // for the last sand to get to 500,0
}

/// Gets the lowest point beneath the given point that is on top of the lowest point beneath the given
/// point that is rock or sand. If the given point is on top of rock or sand, the given point is returned.
/// If there is no point beneath the given point, then the original x coordinate is returned, with y coordinate
/// of `barrier`.
fn get_lowest_point_with_barrier(point: &Point, ceiling: &HashSet<Point>, barrier: usize) -> Point {
    ceiling
        .iter()
        .filter(|p| p.x == point.x && p.y > point.y)
        .min_by_key(|p| p.y)
        .map(|p| Point::new(p.x, p.y - 1))
        .unwrap_or_else(|| Point::new(point.x, barrier - 1))
}

fn part_1(input: &str) -> usize {
    let mut ceiling = ceiling_map(input);
    let mut sand_falls = 0;

    'done: loop {
        let mut lowest = get_lowest_point(&Point::new(500, 0), &ceiling).unwrap();

        // Traverse down the ceiling until we reach the bottom.
        loop {
            if get_lowest_point(&lowest, &ceiling) != Some(lowest) {
                let Some(point) = get_lowest_point(&lowest, &ceiling) else {
                    break 'done;
                };

                lowest = point;
            } else if let Some(point) = can_go_left(&lowest, &ceiling) {
                lowest = point;
            } else if let Some(point) = can_go_right(&lowest, &ceiling) {
                lowest = point;
            } else {
                break;
            }
        }

        ceiling.insert(lowest);
        sand_falls += 1;
    }

    sand_falls
}

/// Checks if the given point can go one down and right. If so, returns the point one down and one
/// right. Otherwise, returns None.
fn can_go_right(point: &Point, ceiling: &HashSet<Point>) -> Option<Point> {
    let right = Point::new(point.x + 1, point.y + 1);

    if !ceiling.contains(&right) {
        Some(right)
    } else {
        None
    }
}

/// Checks if the given point can go one down and left. If so, returns the point one down and one
/// left. Otherwise, returns None.
fn can_go_left(point: &Point, ceiling: &HashSet<Point>) -> Option<Point> {
    let left = Point::new(point.x - 1, point.y + 1);

    if !ceiling.contains(&left) {
        Some(left)
    } else {
        None
    }
}

/// Gets the lowest point beneath the given point that is on top of the lowest point beneath the given
/// point that is rock or sand. If the given point is on top of rock or sand, the given point is returned.
/// If there is no point beneath the given point, then `None` is returned.
fn get_lowest_point(point: &Point, ceiling: &HashSet<Point>) -> Option<Point> {
    ceiling
        .iter()
        .filter(|p| p.x == point.x && p.y > point.y)
        .min_by_key(|p| p.y)
        .map(|p| Point::new(p.x, p.y - 1))
}

/// Prints the ceiling map, starting from x=494 to x=503 and y=0 to y=9.
fn print_ceiling_map(ceiling: &HashSet<Point>) {
    for y in 0..=11 {
        for x in 489..=511 {
            let point = Point::new(x, y);

            if ceiling.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn ceiling_map(input: &str) -> HashSet<Point> {
    let mut ceiling = HashSet::new();

    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut point = point.split(',');
                    let x = point.next().unwrap().parse().unwrap();
                    let y = point.next().unwrap().parse().unwrap();

                    Point::new(x, y)
                })
                .tuple_windows()
        })
        .for_each(|line| {
            line.for_each(|(start, end)| {
                start.line(&end).into_iter().for_each(|point| {
                    ceiling.insert(point);
                })
            })
        });

    ceiling
}
