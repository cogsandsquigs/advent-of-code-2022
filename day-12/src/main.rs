use std::collections::{HashMap, HashSet};

use anyhow::Result;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-12/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let (heightmap, _, ending_point) = heightmap(input);

    let starting_points: Vec<(usize, usize)> = heightmap
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, &c)| if c == 1 { Some((x, y)) } else { None })
        })
        .collect();

    let mut shortest = usize::MAX;

    for starting_point in starting_points {
        let Ok(steps) = dijkstra_search_steps(&heightmap, starting_point, ending_point) else {
            continue;
        };

        if steps < shortest {
            shortest = steps;
        }
    }

    shortest
}

fn part_1(input: &str) -> usize {
    let (heightmap, starting_point, ending_point) = heightmap(input);

    dijkstra_search_steps(&heightmap, starting_point, ending_point).unwrap()
}

fn dijkstra_search_steps(
    heightmap: &Vec<Vec<usize>>,
    starting_point: (usize, usize),
    ending_point: (usize, usize),
) -> Result<usize, String> {
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: HashSet<(usize, usize)> = HashSet::new();

    distances.insert(starting_point, 0);
    queue.insert(starting_point);

    while !queue.is_empty() {
        let current_point = shortest_point(&distances, &queue);
        queue.remove(&current_point);

        if current_point == ending_point {
            return Ok(distances[&current_point]);
        }

        let neighbors = orthogonal_neighbors_walkable(heightmap, current_point);

        for neighbor in neighbors {
            let new_distance = distances[&current_point] + 1;

            if !distances.contains_key(&neighbor) || new_distance < distances[&neighbor] {
                distances.insert(neighbor, new_distance);
                queue.insert(neighbor);
            }
        }
    }

    Err(String::from("No path found!"))
}

fn shortest_point(
    distances: &HashMap<(usize, usize), usize>,
    points: &HashSet<(usize, usize)>,
) -> (usize, usize) {
    *points
        .iter()
        .min_by_key(|point| distances[point])
        .expect("No shortest point found")
}

fn orthogonal_neighbors_walkable(
    heightmap: &Vec<Vec<usize>>,
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    orthogonal_neighbors(heightmap, (x, y))
        .into_iter()
        .filter(|(p_x, p_y)| heightmap[*p_y][*p_x] <= heightmap[y][x] + 1)
        .collect()
}

fn orthogonal_neighbors(
    heightmap: &Vec<Vec<usize>>,
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];

    if x > 0 {
        neighbors.push((x - 1, y));
    }

    if x < heightmap[0].len() - 1 {
        neighbors.push((x + 1, y));
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }

    if y < heightmap.len() - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn heightmap(input: &str) -> (Vec<Vec<usize>>, (usize, usize), (usize, usize)) {
    let starting_point = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .enumerate()
                .find_map(|(x, c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .expect("No starting point found");

    let ending_point = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .enumerate()
                .find_map(|(x, c)| if c == 'E' { Some((x, y)) } else { None })
        })
        .expect("No ending point found");

    let heightmap = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'a'..='z' => c as usize - 'a' as usize + 1, // Height starts at 1, so we can subtract 1 safely
                    'E' => 26, // Highest point, same elevation as 'z'
                    'S' => 1,  // Lowest point, same elevation as 'a'
                    _ => unreachable!("Invalid character '{c}'"),
                })
                .collect()
        })
        .collect();

    (heightmap, starting_point, ending_point)
}
