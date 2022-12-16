use advent_utils::{files::read, macros::solution};
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
fn part_2(input: &str) -> i64 {
    todo!()
}

#[solution(day = "16", part = "1")]
fn part_1(input: &str) -> usize {
    let mut valves = valves(input);

    for (id, valve) in &valves {
        println!(
            "{} leads to {}",
            id,
            valve
                .leads_to
                .iter()
                .map(|(id, dist)| format!("{} with distance {}", id, dist))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    println!("---");

    collapse_valves(&mut valves);

    for (id, valve) in &valves {
        println!(
            "{} leads to {}",
            id,
            valve
                .leads_to
                .iter()
                .map(|(id, dist)| format!("{} with distance {}", id, dist))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    let num_valves = valves.len();
    let path_dists = floyd_warshall(&valves);

    todo!()
}

// /// Dynamic-programming algorithm to find the next best valve to open. Returns the total flow rate
// /// released.
// fn open_valves(
//     all_valves: &HashMap<ValveID, Valve>,
//     visited_valves: &mut HashSet<ValveID>,
//     current_valve: ValveID,
//     // The remaining time in minutes
//     remaining_time: usize,
//     // The total flow rate composed of all the valves we have opened
//     total_flow: usize,
//     // The total flow rate released
//     released: usize,
// ) -> usize {
//     let valve = all_valves.get(&current_valve).unwrap();
//     // If we have 1 minute left, we can't open any more valves
//     if remaining_time <= 1 {
//         return released + total_flow * remaining_time;
//     }
//     // 2 minutes left allows us to open the valve we are currently at
//     else if remaining_time == 2 {
//         return released + total_flow * remaining_time + valve.flow_rate;
//     }
//     // If we have visited all the valves or all except the current one, we can't open any more valves
//     else if visited_valves.len() >= all_valves.len() - 1 {
//         return released
//             + total_flow * remaining_time
//             + valve.flow_rate * (remaining_time - 1);
//     } else {
//         let best_total_flow = usize::MIN;

//         for neighbor_id in valve.leads_to {
//             let neighbor = all_valves.get(&current_valve).unwrap();

//             visited.insert(neighbor);

//             if best_total_flow < open_valves(all_valves, visited)
//         }
//     }

//     todo!()
// }

/// Floyd-warshall algorithm
fn floyd_warshall(valves: &HashMap<ValveID, Valve>) -> HashMap<(ValveID, ValveID), usize> {
    let mut dist: HashMap<(ValveID, ValveID), usize> = HashMap::new();

    for (valve_id, valve) in valves.iter() {
        for (leads_to, distance) in valve.leads_to.iter() {
            dist.insert((*valve_id, *leads_to), *distance);
        }
    }

    for &k in valves.keys() {
        for &i in valves.keys() {
            for &j in valves.keys() {
                let dist_ij = dist.get(&(i, j)).cloned().unwrap_or(usize::MAX);
                let dist_ik = dist.get(&(i, k)).cloned().unwrap_or(usize::MAX);
                let dist_kj = dist.get(&(k, j)).cloned().unwrap_or(usize::MAX);
                let dist_ik_kj = dist_ik.checked_add(dist_kj).unwrap_or(usize::MAX);
                dist.insert((i, j), dist_ij.min(dist_ik_kj));
            }
        }
    }

    dist
}

/// Gets rid of every valve with a flow rate of 0, by replacing them with +1 distance for paths of adjacent valves
fn collapse_valves(valves: &mut HashMap<ValveID, Valve>) {
    let mut to_remove: HashSet<ValveID> = HashSet::new();
    let mut removed_id_neighbors: HashMap<ValveID, Vec<ValveID>> = HashMap::new();

    for (id, valve) in valves.iter() {
        // Don't get rid of the starting point valve
        if valve.flow_rate == 0 && valve.id != ValveID::new("AA") {
            to_remove.insert(*id);
            removed_id_neighbors.insert(*id, valve.leads_to.keys().copied().collect());
        }
    }

    for id in &to_remove {
        let valve = valves.remove(id).unwrap();

        for neighbor_id in valve.leads_to.keys() {
            // If the neighbor is also being removed, we need to add its neighbors to the current valve
            if to_remove.contains(neighbor_id) {
                for neighbor_neighbor_id in removed_id_neighbors.get(neighbor_id).unwrap() {
                    let neighbor = valves.get_mut(neighbor_neighbor_id).unwrap();
                    neighbor.leads_to.insert(*id, 1);
                }
            } else {
                let neighbor = valves.get_mut(neighbor_id).unwrap();
                neighbor.leads_to.insert(*id, 1);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapItem<T>(Reverse<usize>, T)
where
    T: PartialEq + Eq;

impl<T: Eq> PartialOrd for HeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<T: Eq> Ord for HeapItem<T> {
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
            leads_to: leads_to.into_iter().map(|id| (id, 1)).collect(),
        },
    ))
}

fn valve_id(input: &str) -> IResult<&str, ValveID> {
    let (input, id) = take(2usize)(input)?;
    let id = ValveID::new(id);
    Ok((input, id))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Valve {
    id: ValveID,
    flow_rate: usize,
    leads_to: HashMap<ValveID, usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
