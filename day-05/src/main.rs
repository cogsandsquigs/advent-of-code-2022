use anyhow::Result;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-04/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &String) -> usize {
    todo!()
}

fn part_1(input: &String) -> usize {
    todo!()
}
