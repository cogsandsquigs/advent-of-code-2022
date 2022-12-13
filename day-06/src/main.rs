use advent_utils::files::read;
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-06/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    for (idx, window) in chars.windows(14).enumerate() {
        if all_unique(window) {
            return idx + 14;
        }
    }

    unreachable!("There should have been a stop before")
}

fn part_1(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    for (idx, window) in chars.windows(4).enumerate() {
        if all_unique(window) {
            return idx + 4;
        }
    }

    unreachable!("There should have been a stop before")
}

fn all_unique(s: &[char]) -> bool {
    s.iter().enumerate().all(|(i, &c)| {
        s.iter()
            .enumerate()
            .skip(i + 1)
            .all(|(_, &other)| c != other)
    })
}
