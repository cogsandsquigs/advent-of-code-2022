use anyhow::Result;
use std::ops::RangeInclusive;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-04/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &String) -> usize {
    let pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = get_pairs(input);

    pairs
        .into_iter()
        .filter(|(a, b)| (a.start() <= b.end() && a.end() >= b.start()))
        .count()
}

fn part_1(input: &String) -> usize {
    let pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = get_pairs(input);

    pairs
        .into_iter()
        .filter(|(a, b)| {
            a.start() <= b.start() && b.end() <= a.end()
                || a.start() >= b.start() && b.end() >= a.end()
        })
        .count()
}

fn get_pairs(input: &String) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
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
