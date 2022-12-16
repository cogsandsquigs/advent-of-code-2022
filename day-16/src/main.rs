use advent_utils::{files::read, macros::solution};
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{Display, Formatter},
};

fn main() -> Result<()> {
    let input = read("day-16/input.test.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "16", part = "2")]
fn part_2(input: &str) -> i64 {
    todo!()
}

#[solution(day = "16", part = "1")]
fn part_1(input: &str) -> usize {
    let all_valves = valves(input);
    let mut visited_valves: HashSet<ValveID> = HashSet::new();
    let mut remaining_time = 30;
    let mut current_valve = ValveID::new("AA");
    let mut total_flow = 0;
    let mut total_released = 0;

    // Visit as many valves as possible
    while visited_valves.len() < all_valves.len() {
        let (next_valve, time_to_valve) =
            next_best_valve(&all_valves, &visited_valves, &current_valve, remaining_time);
        let next_valve = all_valves.get(&next_valve).unwrap();

        // Update the total released WITHOUT including the current valve.
        total_released += total_flow * (time_to_valve + 1);

        // Update the total flow.
        total_flow += next_valve.flow_rate;

        // Update the remaining time. +1 for the time to release the current valve.
        remaining_time = remaining_time.saturating_sub(time_to_valve + 1);

        // Update the current valve.
        current_valve = next_valve.id;

        // Mark the valve as visited if we released from it.s
        visited_valves.insert(current_valve);

        println!(
            "Visited {} with a new total flow of {} and a total released of {}. Time spent: {}, time remaining: {}",
            current_valve, total_flow, total_released, time_to_valve + 1, remaining_time
        );

        // If we've run out of time, stop.
        if remaining_time == 0 {
            break;
        }
    }

    total_released
}

// Gets the next-best valve to visit, and returns the id of the valve, as well as the
// time it would take to get there.
fn next_best_valve(
    all_valves: &HashMap<ValveID, Valve>,
    visited_valves: &HashSet<ValveID>,
    current_valve: &ValveID,
    remaining_time: usize,
) -> (ValveID, usize) {
    let mut best_valve = all_valves.get(current_valve).unwrap().clone(); // Set best valve to current valve
    let mut best_score = usize::MIN;
    let mut best_time = usize::MAX;

    for (id, valve) in all_valves {
        // Skip any valves we've already visited.
        if visited_valves.contains(id) {
            continue;
        }

        let time_to_valve = bfs(all_valves, current_valve, id);

        // Skip any valves that we can't reach in time.
        if time_to_valve > remaining_time {
            continue;
        }

        // Get the score for this valve, which is the benefit of visiting it.
        let valve_score = valve.flow_rate * (remaining_time - time_to_valve);

        if valve_score >= best_score {
            best_valve = valve.clone();
            best_score = valve_score;
            best_time = time_to_valve;
        }
    }

    (best_valve.id, best_time)
}

// Finds the shortest route between two valves, and returns the time it would take to traverse that route.
fn bfs(valves: &HashMap<ValveID, Valve>, start_id: &ValveID, end_id: &ValveID) -> usize {
    let mut queue = Vec::new();
    let mut distances = HashMap::new();

    queue.push((start_id, 0));
    distances.insert(start_id, 0);

    while let Some((current_id, current_distance)) = queue.pop() {
        let current_valve = valves.get(current_id).unwrap();

        for next_id in &current_valve.leads_to {
            let next_distance = current_distance + 1;

            if next_id == end_id {
                return next_distance;
            }

            if let Some(&existing_distance) = distances.get(&next_id) {
                if next_distance < existing_distance {
                    distances.insert(next_id, next_distance);
                    queue.push((next_id, next_distance));
                }
            } else {
                distances.insert(next_id, next_distance);
                queue.push((next_id, next_distance));
            }
        }
    }

    unreachable!("No route found between {} and {}", start_id, end_id);
}

// Finds the most optimal route between two valves, and returns the total pressure released by that route.
fn find_optimal_score(
    valves: &HashMap<ValveID, Valve>,
    total
    start_id: &ValveID,
    end_id: &ValveID,

) -> usize {
    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();

    queue.push(ValveIDHeapItem(Reverse(0), *start_id));
    distances.insert(start_id, 0);

    while let Some(ValveIDHeapItem(Reverse(current_score), current_id)) = queue.pop() {
        let current_valve = valves.get(&current_id).unwrap();

        for next_id in &current_valve.leads_to {
            let next_score = current_score - current_valve.flow_rate as i32;

            if next_id == end_id {
                println!("Found route with score {}", next_score);
                return next_score as usize;
            }

            if let Some(&existing_score) = distances.get(&next_id) {
                if next_score > existing_score {
                    distances.insert(next_id, next_score);
                    queue.push(ValveIDHeapItem(Reverse(next_score), *next_id));
                }
            } else {
                distances.insert(next_id, next_score);
                queue.push(ValveIDHeapItem(Reverse(next_score), *next_id));
            }
        }
    }

    unreachable!("No route found between {} and {}", start_id, end_id);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ValveIDHeapItem(Reverse<i32>, ValveID);

impl PartialOrd for ValveIDHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for ValveIDHeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

fn valves(input: &str) -> HashMap<ValveID, Valve> {
    let mut valves = HashMap::new();

    input
        .lines()
        .map(|line| valve(line).unwrap().1)
        .for_each(|valve| {
            valves.insert(valve.id, valve);
        });

    valves
}

fn valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, id) = valve_id(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    let (input, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(input)?;
    let (input, leads_to) = separated_list1(tag(", "), valve_id)(input)?;

    Ok((
        input,
        Valve {
            id,
            flow_rate,
            leads_to,
        },
    ))
}

fn valve_id(input: &str) -> IResult<&str, ValveID> {
    let (input, id) = take(2usize)(input)?;
    let id = ValveID::new(id);
    Ok((input, id))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Valve {
    id: ValveID,
    flow_rate: usize,
    leads_to: Vec<ValveID>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveID {
    id: usize,
}

impl ValveID {
    fn new(string: &str) -> Self {
        let char1 = string.chars().next().unwrap();
        let char2 = string.chars().nth(1).unwrap();
        let id = (char1 as usize - 'A' as usize) * 26 + (char2 as usize - 'A' as usize);
        Self { id }
    }
}

impl Display for ValveID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char1 = (self.id / 26) as u8 + b'A';
        let char2 = (self.id % 26) as u8 + b'A';
        write!(f, "{}{}", char1 as char, char2 as char)
    }
}
