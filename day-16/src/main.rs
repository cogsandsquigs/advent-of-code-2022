use advent_utils::{macros::solution, queue::Queue};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};

fn main() {
    part_1();
    part_2();
}

#[solution(day = "16", part = "2")]
fn part_2(input: &str) -> usize {
    let all_valves = valves(input);
    let good_valves = all_valves
        .iter()
        .filter(|(_, valve)| valve.rate > 0)
        .map(|(id, _)| id)
        .copied()
        .collect();
    let distances = floyd_warshall(&all_valves, &good_valves);

    (1..good_valves.len() - 1)
        .flat_map(|k| good_valves.iter().combinations_with_replacement(k))
        .map(|v| HashSet::<ValveID>::from_iter(v.into_iter().copied()))
        .tuple_combinations()
        .filter(|(a, b)| a.is_disjoint(b))
        .map(|(a, b)| bfs(&all_valves, &a, &distances, 26) + bfs(&all_valves, &b, &distances, 26))
        .max()
        .expect("could not find max")
}

#[solution(day = "16", part = "1")]
fn part_1(input: &str) -> usize {
    let valves = valves(input);
    let good_valves = valves
        .iter()
        .filter(|(_, valve)| valve.rate > 0)
        .map(|(id, _)| *id)
        .collect::<HashSet<ValveID>>();

    let distances = floyd_warshall(&valves, &good_valves);

    bfs(&valves, &good_valves, &distances, 30)
}

/// BFS search on the graph of distances between all valves. Returns the most water that can be released.
fn bfs(
    all_valves: &HashMap<ValveID, Valve>,
    good_valves: &HashSet<ValveID>,
    distances: &HashMap<(ValveID, ValveID), usize>,
    initial_time: usize,
) -> usize {
    let mut queue = Vec::new();
    queue.push(State::new(initial_time));

    let mut max_released = 0;

    while let Some(mut state) = queue.pop() {
        max_released = max_released.max(state.release_all());

        if state.time_remaining == 0 {
            continue;
        }

        let Some(valve) = state.get_valve(all_valves).cloned() else {
            continue;
        };

        // Greedily open valve if we can
        if !state.opened.contains(&state.valve) && valve.rate > 0 {
            state.open(&valve);
            queue.push(state);
            continue;
        }

        for id in good_valves {
            if state.opened.contains(id) {
                continue;
            }

            let distance = distances.get(&(state.valve, *id)).unwrap();

            // Travel to the valve and open it
            if state.time_remaining > *distance {
                let mut new_state = state.clone();
                new_state.travel(id, *distance);
                new_state.open(all_valves.get(id).unwrap());
                queue.push(new_state);
            }
        }
    }

    max_released
}

fn floyd_warshall(
    all_valves: &HashMap<ValveID, Valve>,
    good_valves: &HashSet<ValveID>,
) -> HashMap<(ValveID, ValveID), usize> {
    let mut distances = HashMap::new();

    for i in all_valves.keys() {
        for j in good_valves {
            distances.insert((*i, *j), dist_between_valves(all_valves, i, j));
        }
    }

    distances
}

fn dist_between_valves(
    all_valves: &HashMap<ValveID, Valve>,
    start: &ValveID,
    end: &ValveID,
) -> usize {
    let mut queue = Queue::new();
    queue.push(start, Reverse(0));

    let mut visited = HashSet::new();

    while let Some((current, Reverse(distance))) = queue.pop_with_priority() {
        if current == end {
            return distance;
        }

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        let current_valve = all_valves.get(current).unwrap();

        for id in all_valves.keys() {
            if id == current {
                continue;
            }

            if current_valve.neighbors.contains(id) {
                queue.push(id, Reverse(distance + 1));
            }
        }
    }

    panic!("No path found between {} and {}", start, end);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    valve: ValveID,
    time_remaining: usize,
    released: usize,
    rate: usize,
    opened: HashSet<ValveID>,
}

impl State {
    fn new(time_remaining: usize) -> Self {
        Self {
            valve: ValveID::new("AA"),
            time_remaining,
            released: 0,
            rate: 0,
            opened: HashSet::new(),
        }
    }

    fn release_all(&self) -> usize {
        self.released + self.rate * self.time_remaining
    }

    fn get_valve<'a>(&'a self, all_valves: &'a HashMap<ValveID, Valve>) -> Option<&Valve> {
        all_valves.get(&self.valve)
    }

    fn open(&mut self, valve: &Valve) {
        // Takes 1 minute to open a valve
        self.time_remaining -= 1;
        // Release pressure from the valves that have been opened so far
        self.released += self.rate;

        // Add the rate from this valve to the total rate
        self.rate += valve.rate;

        // Tell the state that we've opened this valve
        self.opened.insert(valve.name);
    }

    fn travel(&mut self, valve: &ValveID, time: usize) {
        // Release pressure from the valves that have been opened so far
        // for the given amount of time
        self.released += self.rate * time;
        // Travel for the given amount of time
        self.time_remaining -= time;
        // Update the current valve
        self.valve = *valve;
    }
}

fn valves(input: &str) -> HashMap<ValveID, Valve> {
    let mut valves = HashMap::new();

    input
        .lines()
        .map(|line| valve(line).unwrap().1)
        .for_each(|valve| {
            valves.insert(valve.name, valve);
        });

    valves
}

fn valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = valve_id(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, rate) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    let (input, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(input)?;
    let (input, neighbors) = separated_list1(tag(", "), valve_id)(input)?;

    Ok((
        input,
        Valve {
            name,
            rate,
            neighbors: neighbors.into_iter().collect(),
        },
    ))
}

fn valve_id(input: &str) -> IResult<&str, ValveID> {
    let (input, name) = take(2usize)(input)?;
    let name = ValveID::new(name);
    Ok((input, name))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Valve {
    name: ValveID,
    rate: usize,
    neighbors: HashSet<ValveID>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ValveID {
    name: usize,
}

impl ValveID {
    fn new(string: &str) -> Self {
        let char1 = string.chars().next().unwrap();
        let char2 = string.chars().nth(1).unwrap();
        let name = (char1 as usize - 'A' as usize) * 26 + (char2 as usize - 'A' as usize);
        Self { name }
    }
}

impl Display for ValveID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char1 = (self.name / 26) as u8 + b'A';
        let char2 = (self.name % 26) as u8 + b'A';
        write!(f, "{}{}", char1 as char, char2 as char)
    }
}
