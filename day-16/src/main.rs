use advent_utils::{files::read, macros::solution, queue::Queue};
use anyhow::Result;
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

fn main() -> Result<()> {
    let input = read("day-16/input.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "16", part = "2")]
fn part_2(input: &str) -> usize {
    let valves = valves(input);
    let good_valves = valves
        .iter()
        .filter(|(_, valve)| valve.rate > 0)
        .map(|(id, _)| *id)
        .collect::<HashSet<ValveID>>();

    let distances = floyd_warshall(&valves, &good_valves);

    bfs_two_actors(&valves, &good_valves, &distances)
}

/// BFS search on the graph of distances between all valves. Returns the most water that can be released.
fn bfs_two_actors(
    all_valves: &HashMap<ValveID, Valve>,
    good_valves: &HashSet<ValveID>,
    distances: &HashMap<(ValveID, ValveID), usize>,
) -> usize {
    let mut queue = Vec::new();
    queue.push(State::new(26));

    let mut max_released = 0;

    while let Some(mut state) = queue.pop() {
        max_released = max_released.max(state.release_all());

        if state.person_time_remaining < 2 && state.elephant_time_remaining < 2 {
            continue;
        }

        let person_valve = state.get_person_valve(all_valves).clone();
        let elephant_valve = state.get_elephant_valve(all_valves).clone();

        let mut did_open = false;

        // Greedily open valves if we can
        if !state.opened.contains(&state.person_valve)
            && person_valve.rate > 0
            && state.person_time_remaining >= 2
        {
            state.open_person(&person_valve);
            did_open = true;
        }

        // Greedily open valves if we can
        if !state.opened.contains(&state.elephant_valve)
            && elephant_valve.rate > 0
            && state.elephant_time_remaining >= 2
        {
            state.open_elephant(&elephant_valve);
            did_open = true;
        }

        if did_open {
            queue.push(state);
            continue;
        }

        for (person_id, elephant_id) in good_valves
            .iter()
            .combinations_with_replacement(2)
            .map(|v| (v[0], v[1]))
        {
            if state.person_time_remaining < 2
                || state.elephant_time_remaining < 2
                || person_id == elephant_id
                || state.opened.contains(person_id)
                || state.opened.contains(elephant_id)
            {
                continue;
            }

            let mut new_state = state.clone();

            let person_distance = distances.get(&(state.person_valve, *person_id)).unwrap();
            let elephant_distance = distances
                .get(&(state.elephant_valve, *elephant_id))
                .unwrap();

            let mut push_new = false;

            // Travel to the valve and open it
            if state.person_time_remaining > *person_distance {
                new_state.travel_person(person_id, *person_distance);
                new_state.open_person(all_valves.get(person_id).unwrap());
                push_new = true;
                println!("person:   {} -> {}", person_id, *person_distance);
            }

            if state.elephant_time_remaining > *elephant_distance {
                new_state.travel_elephant(elephant_id, *elephant_distance);
                new_state.open_elephant(all_valves.get(elephant_id).unwrap());
                push_new = true;
                println!("elephant: {} -> {}", elephant_id, *elephant_distance);
            }

            if push_new {
                queue.push(new_state);
                // queue.truncate(1000);
                println!("max_released: {}", max_released);
            }
        }
    }

    max_released
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

    bfs_one_actor(&valves, &good_valves, &distances)
}

/// BFS search on the graph of distances between all valves. Returns the most water that can be released.
fn bfs_one_actor(
    all_valves: &HashMap<ValveID, Valve>,
    good_valves: &HashSet<ValveID>,
    distances: &HashMap<(ValveID, ValveID), usize>,
) -> usize {
    let mut queue = Vec::new();
    queue.push(State::new(30));

    let mut max_released = 0;

    while let Some(mut state) = queue.pop() {
        max_released = max_released.max(state.release_all());

        if state.person_time_remaining == 0 {
            continue;
        }

        let valve = state.get_person_valve(all_valves).clone();
        // Greedily open valve if we can
        if !state.opened.contains(&state.person_valve) && valve.rate > 0 {
            state.open_person(&valve);
            queue.push(state);
            continue;
        }

        for id in good_valves {
            if state.opened.contains(id) {
                continue;
            }

            let distance = distances.get(&(state.person_valve, *id)).unwrap();

            // Travel to the valve and open it
            if state.person_time_remaining > *distance {
                let mut new_state = state.clone();
                new_state.travel_person(id, *distance);
                new_state.open_person(all_valves.get(id).unwrap());
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
    person_valve: ValveID,
    elephant_valve: ValveID,
    person_time_remaining: usize,
    elephant_time_remaining: usize,
    person_released: usize,
    elephant_released: usize,
    person_rate: usize,
    elephant_rate: usize,
    opened: HashSet<ValveID>,
}

impl State {
    fn new(time_remaining: usize) -> Self {
        Self {
            person_valve: ValveID::new("AA"),
            elephant_valve: ValveID::new("AA"),
            person_time_remaining: time_remaining,
            elephant_time_remaining: time_remaining,
            person_released: 0,
            elephant_released: 0,
            person_rate: 0,
            elephant_rate: 0,
            opened: HashSet::new(),
        }
    }

    fn release_all(&self) -> usize {
        self.person_released
            + self.person_rate * self.person_time_remaining
            + self.elephant_released
            + self.elephant_rate * self.elephant_time_remaining
    }

    fn get_person_valve<'a>(&'a self, all_valves: &'a HashMap<ValveID, Valve>) -> &Valve {
        all_valves.get(&self.person_valve).unwrap()
    }

    fn get_elephant_valve<'a>(&'a self, all_valves: &'a HashMap<ValveID, Valve>) -> &Valve {
        all_valves.get(&self.elephant_valve).unwrap()
    }

    fn open_person(&mut self, valve: &Valve) {
        // Takes 1 minute to open a valve
        self.person_time_remaining -= 1;
        // Release pressure from the valves that have been opened so far
        self.person_released += self.person_rate;

        // Add the rate from this valve to the total rate
        self.person_rate += valve.rate;

        // Tell the state that we've opened this valve
        self.opened.insert(valve.name);
    }

    fn open_elephant(&mut self, valve: &Valve) {
        // Takes 1 minute to open a valve
        self.elephant_time_remaining -= 1;
        // Release pressure from the valves that have been opened so far
        self.elephant_released += self.elephant_rate;

        // Add the rate from this valve to the total rate
        self.elephant_rate += valve.rate;

        // Tell the state that we've opened this valve
        self.opened.insert(valve.name);
    }

    fn travel_person(&mut self, valve: &ValveID, time: usize) {
        // Release pressure from the valves that have been opened so far
        // for the given amount of time
        self.person_released += self.person_rate * time;
        // Travel for the given amount of time
        self.person_time_remaining -= time;
        // Update the current valve
        self.person_valve = *valve;
    }

    fn travel_elephant(&mut self, valve: &ValveID, time: usize) {
        // Release pressure from the valves that have been opened so far
        // for the given amount of time
        self.elephant_released += self.elephant_rate * time;
        // Travel for the given amount of time
        self.elephant_time_remaining -= time;
        // Update the current valve
        self.elephant_valve = *valve;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
