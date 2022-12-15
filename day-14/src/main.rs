use advent_utils::{files::read, grid::Grid, point::Point};
use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read("day-14/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

/// Width of the grid
const WIDTH: usize = 1000;
/// Height of the grid
const HEIGHT: usize = 200;

/// Adapted from PBearson's solution
fn part_2(input: &str) -> usize {
    let mut cave = parse_input(input);
    let mut i = 0;
    // This was found during my investigation of my specific input
    let max_y = 163;

    loop {
        let new = drop_sand_p2(&mut cave, Point::new(500, 0), max_y);

        if new.x == 500 && new.y == 0 {
            break;
        }

        i += 1;
    }

    i + 1
}

fn drop_sand_p2(grid: &mut Grid<Tile>, point: Point, max_y: i64) -> Point {
    let mut current = point;

    loop {
        let neighbors = current.neighbors();
        let possible_points = [neighbors[3], neighbors[6], neighbors[7]];

        let Some(found) = possible_points
            .iter()
            .find(|&&point| { grid[point] == Tile::Empty})
            .copied()
        else {
            break;
        };

        current = found;

        if current.y >= max_y - 1 {
            break;
        }
    }

    grid[current] = Tile::Sand;

    current
}

fn part_1(input: &str) -> usize {
    let mut cave = parse_input(input);
    let mut i = 0;

    loop {
        let new = drop_sand_p1(&mut cave, Point::new(500, 0));

        if new.y >= cave.height as i64 - 1 {
            break;
        }

        i += 1;
    }

    i
}

fn drop_sand_p1(grid: &mut Grid<Tile>, point: Point) -> Point {
    let mut current = point;

    loop {
        let neighbors = current.neighbors();
        let possible_points = [neighbors[3], neighbors[6], neighbors[7]];

        let Some(found) = possible_points
            .iter()
            .find(|&&point| { grid[point] == Tile::Empty})
            .copied()
        else {
            break;
        };

        current = found;

        if current.y >= grid.height as i64 - 1 {
            break;
        }
    }

    grid[current] = Tile::Sand;

    current
}

fn parse_input(input: &str) -> Grid<Tile> {
    let rocks: Vec<Vec<Point>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut parts = point.split(',');
                    let x = parts.next().unwrap().parse().unwrap();
                    let y = parts.next().unwrap().parse().unwrap();
                    Point::new(x, y)
                })
                .tuple_windows()
                .flat_map(|(a, b)| draw_line(&a, &b))
                .collect_vec()
        })
        .collect_vec();

    let mut grid: Grid<Tile> = Grid::new(WIDTH, HEIGHT);

    for rock in rocks {
        for point in rock {
            grid[point] = Tile::Rock;
        }
    }

    grid
}

fn draw_line(a: &Point, b: &Point) -> Vec<Point> {
    let mut points = vec![];

    if a.x != b.x {
        let (min, max) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };

        for x in min..=max {
            points.push(Point::new(x, a.y));
        }
    } else {
        let (min, max) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

        for y in min..=max {
            points.push(Point::new(a.x, y));
        }
    }

    points
}

/// The type of a grid tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Tile {
    /// An empty space
    #[default]
    Empty,
    /// A rock
    Rock,
    /// Sand
    Sand,
}
