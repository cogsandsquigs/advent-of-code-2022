use advent_utils::{files::read, heap::Heap, macros::solution};
use anyhow::Result;
use ndarray::{Array2, Array3};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{Display, Formatter},
    rc::Rc,
    sync::Arc,
};

fn main() -> Result<()> {
    let input = read("day-16/input.test.txt")?;

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

    dijkstra_two_actors(&valves, &good_valves, &distances)
}

/// Dijkstra search on the graph of distances between all valves. Returns the most water that can be released.
fn dijkstra_two_actors(
    all_valves: &HashMap<ValveID, Valve>,
    good_valves: &HashSet<ValveID>,
    distances: &HashMap<(ValveID, ValveID), usize>,
) -> usize {
    let mut queue = Heap::new();
    queue.push(State::new(30), 0);

    let mut max_released = 0;

    while let Some(mut state) = queue.pop() {
        max_released = max_released.max(
            state.person_released
                + state.person_rate * state.person_time_remaining
                + state.elephant_released
                + state.elephant_rate * state.elephant_time_remaining,
        );

        if state.person_time_remaining == 0 && state.elephant_time_remaining == 0 {
            continue;
        }

        // Greedily open valves if we can
        if !state.opened.contains(&state.person_valve)
            && all_valves.get(&state.person_valve).unwrap().rate > 0
            && state.person_time_remaining > 0
        {
            state.open_person(all_valves.get(&state.person_valve).unwrap());

            // Also  check if we can open the elephant valve
            if !state.opened.contains(&state.elephant_valve)
                && all_valves.get(&state.elephant_valve).unwrap().rate > 0
                && state.elephant_time_remaining > 0
            {
                state.open_elephant(all_valves.get(&state.elephant_valve).unwrap());
            }

            let released = state.person_released + state.elephant_released; // tmp variable to avoid borrow checker

            queue.push(state, released);
            continue;
        } else if !state.opened.contains(&state.elephant_valve)
            && all_valves.get(&state.elephant_valve).unwrap().rate > 0
            && state.elephant_time_remaining > 0
        {
            state.open_elephant(all_valves.get(&state.elephant_valve).unwrap());
            let released = state.person_released + state.elephant_released; // tmp variable to avoid borrow checker
            queue.push(state, released);
            continue;
        }

        for person_id in good_valves {
            if state.opened.contains(person_id) || state.person_time_remaining == 0 {
                continue;
            }

            for elephant_id in good_valves {
                if state.opened.contains(elephant_id)
                    || person_id == elephant_id
                    || state.elephant_time_remaining == 0
                {
                    continue;
                }

                let mut new_state = state.clone();

                let person_distance = distances.get(&(state.person_valve, *person_id)).unwrap();
                let elephant_distance = distances
                    .get(&(state.elephant_valve, *elephant_id))
                    .unwrap();

                // Travel to the valve and open it
                if state.person_time_remaining > *person_distance {
                    new_state.travel_person(person_id, *person_distance);
                    new_state.open_person(all_valves.get(person_id).unwrap());
                    // println!("person:   {} -> {}", person_id, *person_distance);
                }
                if state.elephant_time_remaining > *elephant_distance {
                    new_state.travel_elephant(elephant_id, *elephant_distance);
                    new_state.open_elephant(all_valves.get(elephant_id).unwrap());
                    // println!("elephant: {} -> {}", elephant_id, *elephant_distance);
                }

                let released = new_state.person_released + new_state.elephant_released; // tmp variable to avoid borrow checker
                queue.push(new_state, released);
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

    dijkstra_one_actor(&valves, &good_valves, &distances)
}

/// Dijkstra search on the graph of distances between all valves. Returns the most water that can be released.
fn dijkstra_one_actor(
    all_valves: &HashMap<ValveID, Valve>,
    good_valves: &HashSet<ValveID>,
    distances: &HashMap<(ValveID, ValveID), usize>,
) -> usize {
    let mut queue = Heap::new();
    queue.push(State::new(30), 0);

    let mut max_released = 0;

    while let Some(mut state) = queue.pop() {
        max_released = max_released
            .max(state.person_released + state.person_rate * state.person_time_remaining);

        if state.person_time_remaining == 0 {
            continue;
        }
        // Greedily open valve if we can
        else if !state.opened.contains(&state.person_valve)
            && all_valves.get(&state.person_valve).unwrap().rate > 0
        {
            state.open_person(all_valves.get(&state.person_valve).unwrap());
            let released = state.person_released; // tmp variable to avoid borrow checker
            queue.push(state, released);
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
                let released = new_state.person_released; // tmp variable to avoid borrow checker
                queue.push(new_state, released);
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
    let mut queue = Heap::new();
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
