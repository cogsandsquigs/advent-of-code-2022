use advent_utils::{files::read, point::Point};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = read("day-14/input.test.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

/// Adapted from PBearson's solution
fn part_2(input: &str) -> usize {
    let (min_x, max_x, max_y) = dimensions(input);
    println!("Dimensions: {} {} {}", min_x, max_x, max_y);
    let ceiling = ceiling_grid(input, (min_x, max_x, max_y));

    count_sand(ceiling)
}

fn count_sand(mut grid: Vec<Vec<bool>>) -> usize {
    let mut count = 0;

    for (i, row) in grid.clone().into_iter().enumerate() {
        let mut rowcount = 1 + (i * 2) - row.iter().filter(|&&x| x).count();

        // Check top 3 rows
        if i > 0 {
            for j in 1..row.len() - 1 {
                let p1 = grid[i - 1][j - 1];
                let p2 = grid[i - 1][j];
                let p3 = grid[i - 1][j + 1];

                if !grid[i][j] && p1 && p2 && p3 {
                    grid[i][j] = true;
                    rowcount -= 1;
                }
            }
        }

        count += rowcount
    }

    count
}

fn ceiling_grid(input: &str, (min_x, max_x, max_y): (usize, usize, usize)) -> Vec<Vec<bool>> {
    let mut ceiling: HashMap<usize, Vec<Point>> = HashMap::new();

    input.lines().for_each(|line| {
        line.split(" -> ")
            .map(|point| {
                let mut parts = point.split(',');

                let x = parts.next().unwrap();
                let y = parts.next().unwrap();

                let x = x.parse::<usize>().expect("Failed to parse x coordinate");
                let y = y.parse::<usize>().expect("Failed to parse y coordinate");

                Point::new(x, y)
            })
            .tuple_windows()
            .flat_map(|(a, b)| a.line(&b))
            .for_each(|point| {
                ceiling.entry(point.x).or_insert_with(Vec::new).push(point);
            });
    });

    let mut grid = vec![vec![false; max_x - min_x + 1]; max_y];

    for (_, points) in ceiling {
        for point in points {
            grid[point.y][point.x - min_x] = true;
        }
    }

    grid
}

// Gets  min x, max x,max y
fn dimensions(input: &str) -> (usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    input.lines().for_each(|line| {
        line.split(" -> ")
            .map(|point| {
                let mut parts = point.split(',');

                let x = parts.next().unwrap();
                let y = parts.next().unwrap();

                let x = x.parse::<usize>().expect("Failed to parse x coordinate");
                let y = y.parse::<usize>().expect("Failed to parse y coordinate");

                Point::new(x, y)
            })
            .for_each(|point| {
                if point.x < min_x {
                    min_x = point.x;
                }

                if point.x > max_x {
                    max_x = point.x;
                }

                if point.y > max_y {
                    max_y = point.y;
                }
            });
    });

    (min_x, max_x, max_y + 2)
}

fn part_1(input: &str) -> usize {
    let mut ceiling = ceiling_map(input);
    let mut sand_falls = 0;

    'done: loop {
        let mut lowest = lowest_point(&Point::new(500, 0), &ceiling);

        loop {
            if lowest.is_none() {
                break 'done;
            } else if let Some(point) = can_go_left(&lowest.unwrap(), &ceiling) {
                lowest = lowest_point(&point, &ceiling);
            } else if let Some(point) = can_go_right(&lowest.unwrap(), &ceiling) {
                lowest = lowest_point(&point, &ceiling);
            } else {
                break;
            }
        }

        let v = ceiling.entry(lowest.unwrap().x).or_insert_with(Vec::new);
        v.push(lowest.unwrap());
        v.sort_by_key(|p| p.y);
        sand_falls += 1;
    }

    sand_falls
}

fn can_go_right(point: &Point, ceiling: &HashMap<usize, Vec<Point>>) -> Option<Point> {
    let right = Point::new(point.x + 1, point.y + 1);

    if ceiling.contains_key(&right.x) && ceiling.get(&right.x).unwrap().contains(&right) {
        None
    } else {
        Some(right)
    }
}

fn can_go_left(point: &Point, ceiling: &HashMap<usize, Vec<Point>>) -> Option<Point> {
    let left = Point::new(point.x - 1, point.y + 1);

    if ceiling.contains_key(&left.x) && ceiling.get(&left.x).unwrap().contains(&left) {
        None
    } else {
        Some(left)
    }
}

fn lowest_point(point: &Point, ceiling: &HashMap<usize, Vec<Point>>) -> Option<Point> {
    ceiling
        .get(&point.x)
        .and_then(|points| points.iter().filter(|p| p.y > point.y).min_by_key(|p| p.y))
        .map(|p| Point::new(p.x, p.y - 1))
        .or(None)
}

/// Prints the ceiling map, starting from x=494 to x=503 and y=0 to y=9.
fn print_ceiling_map(ceiling: &HashMap<usize, Vec<Point>>) {
    for y in 0..=11 {
        for x in 489..=511 {
            let point = Point::new(x, y);

            if let Some(points) = ceiling.get(&point.x) {
                if points.contains(&point) {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn ceiling_map(input: &str) -> HashMap<usize, Vec<Point>> {
    let mut ceiling: HashMap<usize, Vec<Point>> = HashMap::new();

    input.lines().for_each(|line| {
        line.split(" -> ")
            .map(|point| {
                let mut parts = point.split(',');

                let x = parts.next().unwrap();
                let y = parts.next().unwrap();

                let x = x.parse::<usize>().expect("Failed to parse x coordinate");
                let y = y.parse::<usize>().expect("Failed to parse y coordinate");

                Point::new(x, y)
            })
            .tuple_windows()
            .flat_map(|(a, b)| a.line(&b))
            .for_each(|point| {
                ceiling.entry(point.x).or_insert_with(Vec::new).push(point);
            });
    });

    ceiling
}
