use std::collections::HashSet;

use advent_utils::{files::read, grid::Grid, point::Point};
use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

fn main() -> Result<()> {
    let input = read("day-15/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> i32 {
    let sensors = parse(input);
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
    let mut points_set: HashSet<Point> = HashSet::new();

    for sensor in sensors {
        let points: Vec<Point> = sensor
            .points_within_range()
            .into_iter()
            .filter(|point| {
                point.x >= 0
                    && point.x <= 4000000
                    && point.y >= 0
                    && point.y <= 4000000
                    && sensor.distance_to_point(point) <= sensor.distance_to_beacon()
            })
            .collect();
        points_set.extend(points);
    }

    todo!()
}

fn part_1(input: &str) -> usize {
    let sensors = parse(input);
    let mut points_set: HashSet<Point> = HashSet::new();
    let y_level: i32 = 2000000; // 10 for test, 2000000 for actual input

    for sensor in sensors {
        // Check if the point is within y level range
        let y_diff = (sensor.position.y.abs() - y_level.abs()).abs();
        if sensor.distance_to_point(&Point::new(sensor.position.x, sensor.position.y + y_diff))
            > sensor.distance_to_beacon()
        {
            println!("{:?} is not within range of y level {}", sensor, y_level);
            continue;
        }

        let mut points = sensor.points_within_range_at_y(y_level);
        points.retain(|point| point != &sensor.closest_beacon); // Sanity check, not necessary b/c no point will be the same as the beacon
                                                                // As there is only one beacon within range per sensor
        points_set.extend(points);
    }

    points_set.len()
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

    fn distance_to_point(&self, point: &Point) -> u32 {
        self.position.manhattan_distance(point)
    }

    fn point_within_beacon_range(&self, point: Point) -> bool {
        // Less than because we know there won't ever be a tie
        point.manhattan_distance(&self.position) <= self.distance_to_beacon()
    }

    fn points_within_range_at_y(&self, y: i32) -> Vec<Point> {
        let mut points = Vec::new();

        // Get distance to y level
        let y_distance = (y - self.position.y).abs();

        let x_min = self.position.x - (self.distance_to_beacon() as i32 - y_distance);
        let x_max = self.position.x + (self.distance_to_beacon() as i32 - y_distance);

        for x in x_min..=x_max {
            let point = Point::new(x, y);

            if self.point_within_beacon_range(point) {
                points.push(point);
            }
        }

        points
    }

    /// Computes all the possible points within range of the sensor
    fn points_within_range(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let x_min = self.position.x - self.distance_to_beacon() as i32;
        let x_max = self.position.x + self.distance_to_beacon() as i32;

        let y_min = self.position.y - self.distance_to_beacon() as i32;
        let y_max = self.position.y + self.distance_to_beacon() as i32;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let point = Point::new(x, y);

                if self.point_within_beacon_range(point) {
                    points.push(point);
                }
            }
        }

        points
    }
}
