use advent_utils::{files::read, macros::solution};
use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = read("day-18/input.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "18", part = "2")]
fn part_2(input: &str) -> i64 {
    let points = points(input);
    let mins = points.iter().fold((0, 0, 0), |acc, (x, y, z)| {
        (acc.0.min(*x) - 1, acc.1.min(*y) - 1, acc.2.min(*z) - 1)
    });
    let maxs = points.iter().fold((0, 0, 0), |acc, (x, y, z)| {
        (acc.0.max(*x) + 1, acc.1.max(*y) + 1, acc.2.max(*z) + 1)
    });
    let mut total = 0;
    let mut visited = points.clone();
    let mut queue = vec![mins];

    while let Some(point) = queue.pop() {
        for neighbor in neighbors(point) {
            if neighbor.0 < mins.0
                || neighbor.0 > maxs.0
                || neighbor.1 < mins.1
                || neighbor.1 > maxs.1
                || neighbor.2 < mins.2
                || neighbor.2 > maxs.2
            {
                continue;
            }

            total += i64::from(points.contains(&neighbor));

            // If we haven't seen this point before, add it to the queue.
            if visited.insert(neighbor) {
                queue.push(neighbor);
            }
        }
    }

    total
}

#[solution(day = "18", part = "1")]
fn part_1(input: &str) -> i64 {
    let points = points(input);
    let mut inserted = HashSet::new();
    let mut total = 0;

    for point in points {
        let rm = neighbors(point)
            .iter()
            .filter(|&point| inserted.contains(point))
            .count() as i64;
        inserted.insert(point);
        total += 6 - rm * 2;
    }

    total
}

fn neighbors((x, y, z): (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn points(input: &str) -> HashSet<(i64, i64, i64)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect()
}
