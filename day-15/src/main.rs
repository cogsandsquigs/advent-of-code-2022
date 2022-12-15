use std::collections::HashSet;

use advent_utils::{files::read, point::Point};
use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

fn main() -> Result<()> {
    let input = read("day-15/input.test.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> i32 {
    // let sensors = parse(input);
    // let min_bound: i32 = 0;
    // let max_bound: i32 = 4000000; // 20 for test, 4000000 for actual input
    // let mut points_set: HashSet<Point> = HashSet::new();

    // // Prime the set
    // for y in min_bound..=max_bound {
    //     for x in min_bound..=max_bound {
    //         println!("inserting ({}, {})", x, y);
    //         points_set.insert(Point::new(x, y));
    //     }
    // }

    // for sensor in sensors {
    //     println!("{} points left", points_set.len());
    //     let points = sensor.points_within_range();
    //     points_set.retain(|point| !points.contains(point));
    // }

    // println!("{} points left", points_set.len());
    // println!("{:?}", points_set);

    // let point = points_set.iter().next().expect("No points left");

    // point.x * 4000000 + point.y

    todo!()
}

fn part_1(input: &str) -> i32 {
    let sensors = parse(input);
    let y_level: i32 = 10; // 10 for test, 2000000 for actual input
    let mut intervals = Vec::new();
    let mut beacon_count = 0;

    for sensor in sensors {
        if let Some(interval) = x_interval_at_y(sensor, y_level) {
            intervals.push(interval);

            if sensor.closest_beacon.y == y_level
                && sensor.closest_beacon.x >= interval.0
                && sensor.closest_beacon.x <= interval.1
            {
                beacon_count += 1;
            }
        }
    }

    println!("Intervals: {:?}", intervals);

    merge_intervals(&mut intervals);

    println!("Intervals: {:?}", intervals);

    intervals
        .iter()
        .map(|(a, b)| (b - a).abs() + 1)
        .sum::<i32>()
        - beacon_count
}

fn merge_intervals(intervals: &mut Vec<(i32, i32)>) {
    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut index = 0;

    for i in 0..intervals.len() {
        if intervals[index].1 >= intervals[i].0 {
            intervals[index].1 = intervals[i].1;
        } else {
            index += 1;
            intervals[index] = intervals[i];
        }
    }

    intervals.truncate(index + 1);
}

/// Returns `None` if the y-level is not within range of the sensor
fn x_interval_at_y(sensor: Sensor, y: i32) -> Option<(i32, i32)> {
    // Get distance to beacon
    let beacon_distance = sensor.distance_to_beacon() as i32;
    // Get distance to y level
    let y_distance = (y - sensor.position.y).abs();

    // If y level is not within range of the sensor, return None
    if y_distance > beacon_distance {
        return None;
    }

    // Get max and min x
    let x_min = sensor.position.x - (beacon_distance - y_distance);
    let x_max = sensor.position.x + (beacon_distance - y_distance);

    println!(
        "Sensor: {:?}, Beacon: {:?}",
        sensor.position, sensor.closest_beacon
    );
    println!("x_min: {}, x_max: {}", x_min, x_max);

    // Return the interval
    Some((x_min, x_max))
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

/// Returns in order: Sensor point, beacon point
fn parse_line(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<i32>()
    })(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<i32>()
    })(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<i32>()
    })(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<i32>()
    })(input)?;

    Ok((
        input,
        Sensor::new(
            Point::new(sensor_x, sensor_y),
            Point::new(beacon_x, beacon_y),
        ),
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn new(position: Point, closest_beacon: Point) -> Self {
        Self {
            position,
            closest_beacon,
        }
    }

    fn distance_to_beacon(&self) -> u32 {
        self.position.manhattan_distance(&self.closest_beacon)
    }

    // fn distance_to_point(&self, point: &Point) -> u32 {
    //     self.position.manhattan_distance(point)
    // }

    // fn point_within_beacon_range(&self, point: Point) -> bool {
    //     // Less than because we know there won't ever be a tie
    //     point.manhattan_distance(&self.position) <= self.distance_to_beacon()
    // }

    // fn points_within_range_at_y(&self, y: i32) -> Vec<Point> {
    //     let mut points = Vec::new();

    //     // Get distance to y level
    //     let y_distance = (y - self.position.y).abs();

    //     let x_min = self.position.x - (self.distance_to_beacon() as i32 - y_distance);
    //     let x_max = self.position.x + (self.distance_to_beacon() as i32 - y_distance);

    //     for x in x_min..=x_max {
    //         let point = Point::new(x, y);

    //         if self.point_within_beacon_range(point) {
    //             points.push(point);
    //         }
    //     }

    //     points
    // }

    // /// Computes all the possible points within range of the sensor
    // fn points_within_range(&self) -> Vec<Point> {
    //     let mut points = Vec::new();

    //     let x_min = self.position.x - self.distance_to_beacon() as i32;
    //     let x_max = self.position.x + self.distance_to_beacon() as i32;

    //     let y_min = self.position.y - self.distance_to_beacon() as i32;
    //     let y_max = self.position.y + self.distance_to_beacon() as i32;

    //     for x in x_min..=x_max {
    //         for y in y_min..=y_max {
    //             let point = Point::new(x, y);

    //             if self.point_within_beacon_range(point) {
    //                 points.push(point);
    //             }
    //         }
    //     }

    //     points
    // }
}
