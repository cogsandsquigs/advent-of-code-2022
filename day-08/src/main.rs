use anyhow::Result;
use itertools::Itertools;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-08/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let trees = parse(input);

    let mut max_score = 0;

    for (y, row) in trees.iter().enumerate() {
        for (x, _) in row.into_iter().enumerate() {
            let lines = get_edge_lines(&trees, (x, y));

            let score: usize = lines
                .into_iter()
                .map(|line| visible_trees(trees[y][x], line))
                .product();

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn visible_trees(height: u32, line: Vec<u32>) -> usize {
    let mut num_visible = 0;

    for tree in line {
        num_visible += 1;
        if tree >= height {
            break;
        }
    }

    num_visible
}

fn part_1(input: &str) -> usize {
    let trees = parse(input);

    let mut num_visible = 0;

    for (y, row) in trees.iter().enumerate() {
        for (x, _) in row.into_iter().enumerate() {
            let lines = get_edge_lines(&trees, (x, y));

            // If it's on an edge, then we can add one to the count and continue
            if lines.len() < 4 {
                num_visible += 1;
                continue;
            }

            for line in lines.clone() {
                let tree = trees[y][x];

                if line.iter().all(|&x| tree > x) {
                    num_visible += 1;
                    break;
                }
            }
        }
    }

    num_visible
}

/// Gets the lines from a tree (inclusive) to an edge. Assumes that the point is
/// inside `matrix`. Lines are in order from closest to tree first, to farthest from tree
/// last.
fn get_edge_lines(matrix: &Vec<Vec<u32>>, point: (usize, usize)) -> Vec<Vec<u32>> {
    let mut lines: Vec<Vec<u32>> = vec![];
    let height = matrix.len();
    let width = matrix[0].len();
    let (x, y) = point;

    // x direction
    if x < width - 1 {
        let mut line: Vec<u32> = vec![];
        for i in x + 1..width {
            line.push(matrix[y][i])
        }

        lines.push(line);
    }

    // y direction
    if y < height - 1 {
        let mut line: Vec<u32> = vec![];
        for i in y + 1..height {
            line.push(matrix[i][x])
        }

        lines.push(line);
    }

    // -x direction
    if x > 0 {
        let mut line: Vec<u32> = vec![];
        for i in (0..x).rev() {
            line.push(matrix[y][i])
        }

        lines.push(line);
    }

    // -y direction
    if y > 0 {
        let mut line: Vec<u32> = vec![];
        for i in (0..y).rev() {
            line.push(matrix[i][x])
        }

        lines.push(line);
    }

    lines
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}
