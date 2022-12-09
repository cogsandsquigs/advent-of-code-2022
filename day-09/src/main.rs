use std::collections::HashSet;

use anyhow::Result;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-09/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let motions = motions(input);

    simulate_n_knots(motions, 10)
}

fn part_1(input: &str) -> usize {
    let motions = motions(input);

    simulate_n_knots(motions, 2)
}

fn simulate_n_knots(motions: Vec<(Direction, isize)>, n: usize) -> usize {
    let mut knots = vec![(0, 0); n];
    let mut unique_visits: HashSet<(isize, isize)> = HashSet::new();

    for (direction, distance) in motions {
        for _ in 0..distance {
            // Step the head knot forward
            knots[n - 1] = head_direction(knots[n - 1], direction);

            // Step the tail knots forward
            for i in (0..n - 1).rev() {
                knots[i] = step_knot(knots[i + 1], knots[i]);
            }

            // Insert last knot into set
            unique_visits.insert(knots[0]);
        }
    }

    unique_visits.len()
}

fn step_knot(new_head_pos: (isize, isize), tail_pos: (isize, isize)) -> (isize, isize) {
    let mut new_tail_pos = tail_pos;

    // If the head is adjacent to the tail, use the original tail position
    if chebyshev_distance(&new_head_pos, &new_tail_pos) == 1 {
        return new_tail_pos;
    }
    // If it is in the same row or column, use the orthogonal neighbors
    else if new_head_pos.0 == new_tail_pos.0 || new_head_pos.1 == new_tail_pos.1 {
        for neighbor in neighbors_orthogonal(&new_tail_pos) {
            if chebyshev_distance(&new_head_pos, &neighbor)
                < chebyshev_distance(&new_head_pos, &new_tail_pos)
            {
                new_tail_pos = neighbor;
            }
        }
    }
    // Otherwise, use the diagonal neighbors
    else {
        for neighbor in neighbors_diagonal(&new_tail_pos) {
            if chebyshev_distance(&new_head_pos, &neighbor)
                < chebyshev_distance(&new_head_pos, &new_tail_pos)
            {
                new_tail_pos = neighbor;
            }
        }
    }

    new_tail_pos
}

fn head_direction(pos: (isize, isize), direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn chebyshev_distance(a: &(isize, isize), b: &(isize, isize)) -> isize {
    let &(x1, y1) = a;
    let &(x2, y2) = b;

    (x1 - x2).abs().max((y1 - y2).abs())
}

fn neighbors_diagonal(pos: &(isize, isize)) -> [(isize, isize); 4] {
    let &(x, y) = pos;

    [
        (x - 1, y - 1),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
    ]
}

fn neighbors_orthogonal(pos: &(isize, isize)) -> [(isize, isize); 4] {
    let &(x, y) = pos;

    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn motions(input: &str) -> Vec<(Direction, isize)> {
    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| {
            let direction = match line[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                direction => unreachable!("Invalid direction '{direction}'"),
            };

            let distance = line[1].parse::<isize>().unwrap();

            (direction, distance)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
