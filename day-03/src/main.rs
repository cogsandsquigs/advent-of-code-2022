use anyhow::Result;
use itertools::Itertools;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-03/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let rucksacks: Vec<&str> = input.lines().collect();

    rucksacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let (a, b, c) = (
                group.next().unwrap().chars().collect::<Vec<char>>(),
                group.next().unwrap().chars().collect::<Vec<char>>(),
                group.next().unwrap().chars().collect::<Vec<char>>(),
            );

            common_between_3(&a, &b, &c)
        })
        .map(get_priority)
        .sum()
}

fn common_between_3(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>) -> char {
    for char in a {
        if b.contains(&char) && c.contains(char) {
            return *char;
        }
    }

    unreachable!("No common character found")
}

fn part_1(input: &str) -> usize {
    let rucksacks: Vec<&str> = input.lines().collect();

    rucksacks
        .into_iter()
        .map(|sack| {
            let (a, b) = (
                &sack[0..sack.len() / 2].chars().collect::<Vec<char>>(),
                &sack[sack.len() / 2..].chars().collect::<Vec<char>>(),
            );
            common_between_2(a, b)
        })
        .map(get_priority)
        .sum()
}

// Gets the priority of a character
fn get_priority(char: char) -> usize {
    match char {
        'a'..='z' => char as usize - 'a' as usize + 1,
        'A'..='Z' => char as usize - 'A' as usize + 27,
        _ => unreachable!("Invalid character"),
    }
}

fn common_between_2(a: &Vec<char>, b: &Vec<char>) -> char {
    for char in a {
        if b.contains(&char) {
            return *char;
        }
    }

    unreachable!("No common character found")
}
