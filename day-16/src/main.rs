use advent_utils::files::read;
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-16/input.test.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn part_1(input: &str) -> usize {
    todo!()
}
