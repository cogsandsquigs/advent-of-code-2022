use advent_utils::{macros::solution, point::Point};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

fn main() {
    part_1();
    part_2();
}

#[solution(day = "15", part = "2")]
fn part_2(input: &str) -> usize {
    let sensors = parse(input);
    let x_min = 0;
    let x_max = 4000000; // 20 for test, 4000000 for actual input

    for y_level in 0.. {
        let mut intervals = Vec::new();

        for sensor in sensors.iter() {
            if let Some(interval) = x_interval_at_y(*sensor, y_level) {
                intervals.push(interval);
            }
        }

        merge_intervals(&mut intervals);

        if intervals.len() > 1 {
            // Find gap between intervals
            for i in 0..intervals.len() - 1 {
                let (a, b) = intervals[i];
                let (c, d) = intervals[i + 1];

                if a < x_min && b < x_min {
                    continue;
                }

                if c > x_max && d > x_max {
                    continue;
                }

                let gap = c.abs_diff(d);

                if gap > 1 {
                    return (b + 1) * 4000000 + y_level;
                }
            }
        }
    }

    unreachable!("No point found")
}

#[solution(day = "15", part = "1")]
fn part_1(input: &str) -> usize {
    let sensors = parse(input);
    let y_level: usize = 2000000; // 10 for test, 2000000 for actual input
    let mut intervals = Vec::new();

    for sensor in sensors {
        if let Some(interval) = x_interval_at_y(sensor, y_level) {
            intervals.push(interval);
        }
    }

    merge_intervals(&mut intervals);

    intervals.iter().map(|(a, b)| a.abs_diff(*b)).sum()
}

fn merge_intervals(intervals: &mut Vec<(usize, usize)>) {
    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut index = 0;

    for i in 0..intervals.len() {
        if intervals[index].1 >= intervals[i].0 {
            intervals[index].1 = intervals[index].1.max(intervals[i].1);
        } else {
            index += 1;
            intervals[index] = intervals[i];
        }
    }

    intervals.truncate(index + 1);
}

/// Returns `None` if the y-level is not within range of the sensor
fn x_interval_at_y(sensor: Sensor, y: usize) -> Option<(usize, usize)> {
    // Get distance to beacon
    let beacon_distance = sensor.distance_to_beacon();
    // Get distance to y level
    let y_distance = y.abs_diff(sensor.position.y);

    // If y level is not within range of the sensor, return None
    if y_distance > beacon_distance {
        return None;
    }

    // Get max and min x
    let x_min = sensor.position.x - (beacon_distance - y_distance);
    let x_max = sensor.position.x + (beacon_distance - y_distance);

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
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<usize>()
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
    position: Point<usize>,
    closest_beacon: Point<usize>,
}

impl Sensor {
    fn new(position: Point<usize>, closest_beacon: Point<usize>) -> Self {
        Self {
            position,
            closest_beacon,
        }
    }

    fn distance_to_beacon(&self) -> usize {
        self.position.manhattan_distance(&self.closest_beacon)
    }
}
