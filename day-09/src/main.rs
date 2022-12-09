use std::collections::HashSet;

use anyhow::Result;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-09/input.test.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let motions = motions(input);

    simulate_10_knots(motions)
}

fn simulate_10_knots(motions: Vec<(Direction, isize)>) -> usize {
    let mut head_pos = (0, 0);

    // Buffer of 10 "knots", as we have 9 knots in front of the tail, and the tail itself.
    let mut knots: Vec<(isize, isize)> = vec![(0, 0); 10];

    let mut unique_visits: HashSet<(isize, isize)> = HashSet::new();

    for (direction, steps) in motions {
        for _ in 0..steps {
            (head_pos, knots) = step_10_knots_once(head_pos, &knots, direction);

            unique_visits.insert(knots[0]);
        }
    }

    unique_visits.len()
}

/// Takes in head pos, tail pos, and dir, returns new head pos and tail pos
fn step_10_knots_once(
    head_pos: (isize, isize),
    knots: &[(isize, isize)],
    direction: Direction,
) -> ((isize, isize), Vec<(isize, isize)>) {
    let mut new_knots = knots.to_vec();
    let new_head_pos: (isize, isize);

    (new_head_pos, new_knots[9]) = step_once(head_pos, knots[9], direction);

    // Reversed b/c we want to move all the knots starting from front (9) to back (0)
    for i in (0..9).rev() {
        new_knots[i] = step_knot(new_knots[i + 1], knots[i]);
    }

    (new_head_pos, new_knots)
}

fn step_knot(new_head_pos: (isize, isize), tail_pos: (isize, isize)) -> (isize, isize) {
    let mut new_tail_pos = tail_pos;

    // We continuously replace our tail position with our previous head position, unless the head position is
    // within sqrt(2) distance of the tail position.
    if euclidean_distance(&new_head_pos, &tail_pos) > 2_f64.sqrt() {
        new_tail_pos = *neighbors(&new_head_pos)
            .iter()
            .min_by(|pos1, pos2| {
                euclidean_distance(&new_head_pos, pos1)
                    .partial_cmp(&euclidean_distance(&new_head_pos, pos2))
                    .unwrap()
            })
            .unwrap();
    }

    new_tail_pos
}

fn neighbors(pos: &(isize, isize)) -> [(isize, isize); 8] {
    let &(x, y) = pos;

    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
    ]
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

fn part_1(input: &str) -> usize {
    let motions = motions(input);

    simulate_1_knot(motions)
}

fn simulate_1_knot(motions: Vec<(Direction, isize)>) -> usize {
    let (mut head_pos, mut tail_pos) = ((0, 0), (0, 0));
    let mut unique_visits: HashSet<(isize, isize)> = HashSet::new();

    // Insert starting position, as tail is initially at the same position as the head
    unique_visits.insert(tail_pos);

    for (direction, steps) in motions {
        for _ in 0..steps {
            (head_pos, tail_pos) = step_once(head_pos, tail_pos, direction);

            unique_visits.insert(tail_pos);
        }
    }

    unique_visits.len()
}

/// Takes in head pos, tail pos, and dir, returns new head pos and tail pos
fn step_once(
    head_pos: (isize, isize),
    tail_pos: (isize, isize),
    direction: Direction,
) -> ((isize, isize), (isize, isize)) {
    let prev_head_pos = head_pos;
    let new_head_pos = match direction {
        Direction::Up => (head_pos.0, head_pos.1 + 1),
        Direction::Down => (head_pos.0, head_pos.1 - 1),
        Direction::Left => (head_pos.0 - 1, head_pos.1),
        Direction::Right => (head_pos.0 + 1, head_pos.1),
    };

    let mut new_tail_pos = tail_pos;

    // We continuously replace our tail position with our previous head position, unless the head position is
    // within sqrt(2) distance of the tail position.
    if euclidean_distance(&new_head_pos, &tail_pos) > 2_f64.sqrt() {
        new_tail_pos = prev_head_pos;
    }

    (new_head_pos, new_tail_pos)
}

fn euclidean_distance(from: &(isize, isize), to: &(isize, isize)) -> f64 {
    let (x1, y1) = from;
    let (x2, y2) = to;

    ((x1 - x2).pow(2) as f64 + (y1 - y2).pow(2) as f64).sqrt()
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
