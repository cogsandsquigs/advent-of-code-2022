use advent_utils::{files::read, macros::solution};
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-19/input.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "19", part = "2")]
fn part_2(input: &str) -> usize {
    todo!()
}

#[solution(day = "19", part = "1")]
fn part_1(input: &str) -> usize {
    let blueprints = blueprints(input);

    todo!()
}

fn blueprints(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let word = line.split_whitespace().collect::<Vec<_>>();

            Blueprint {
                id: word[1][..word[1].len() - 1].parse().unwrap(),
                ore_cost: word[6].parse().unwrap(),
                clay_cost: word[12].parse().unwrap(),
                obsidian_cost_ore: word[18].parse().unwrap(),
                obsidian_cost_clay: word[21].parse().unwrap(),
                geode_cost_ore: word[27].parse().unwrap(),
                geode_cost_obsidian: word[30].parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Blueprint {
    // The ID of the blueprint.
    id: usize,

    // How much it costs to build an ore robot, in ore.
    ore_cost: usize,

    // How much it costs to build a clay robot, in ore.
    clay_cost: usize,

    // How much it costs to build an obsidian robot in ore.
    obsidian_cost_ore: usize,

    // How much it costs to build an obsidian robot in clay.
    obsidian_cost_clay: usize,

    // How much it costs to build a geode robot in ore.
    geode_cost_ore: usize,

    // How much it costs to build a geode robot in obsidian.
    geode_cost_obsidian: usize,
}

impl Blueprint {
    /// Calculates the range of ore one can make given the time limit `t` minutes. Note that we start out
    /// with 1 ore robot.
    fn ore_ranges(&self, t: usize) -> Vec<usize> {
        todo!()
    }
}
