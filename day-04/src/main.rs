use advent_utils::files::read;
use anyhow::Result;
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    let input = read("day-04/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = get_pairs(input);

    pairs
        .iter()
        .filter(|(a, b)| (a.start() <= b.end() && a.end() >= b.start()))
        .count()
}

fn part_1(input: &str) -> usize {
    let pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = get_pairs(input);

    pairs
        .iter()
        .filter(|(a, b)| {
            a.start() <= b.start() && b.end() <= a.end()
                || a.start() >= b.start() && b.end() >= a.end()
        })
        .count()
}

fn get_pairs(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut ranges = line.split(',').map(|range| {
                let mut split = range.split('-');
                let a = split.next().unwrap().parse().unwrap();
                let b = split.next().unwrap().parse().unwrap();

                a..=b
            });

            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect()
}
