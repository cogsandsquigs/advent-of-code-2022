use anyhow::Result;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-01/input.txt")?;

    // Get the elves' calorie counts
    let mut elves: Vec<Vec<usize>> = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|count| count.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    // Sort the elves from highest to lowest
    elves.sort_by(|a, b| {
        let a = a.into_iter().fold(0, |acc, x| acc + x);
        let b = b.into_iter().fold(0, |acc, x| acc + x);

        // Flip from regular `a.cmp(b)` to sort highest to lowest/descending, meaning
        // we get the elf w/ highest calorie count by indexing at 0.
        b.cmp(&a)
    });

    println!(
        "Puzzle 1 answer: {}",
        elves[0].clone().into_iter().fold(0, |acc, x| acc + x)
    );

    println!(
        "Puzzle 2 answer: {}",
        elves
            .clone()
            .into_iter()
            .take(3)
            .fold(0, |acc, x| acc + x.into_iter().fold(0, |acc, x| acc + x))
    );

    Ok(())
}
