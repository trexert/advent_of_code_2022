use std::{collections::HashSet, str::FromStr};

use lib::common_startup::startup;
use lib::interval::IntervalSet;
use log::{debug, info, trace};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let cli = startup();
    let (input, row, max_index) = if cli.sample {
        (include_str!("sample_input.txt"), 10, 20)
    } else {
        (include_str!("input.txt"), 2_000_000, 4_000_000)
    };

    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| Sensor::from_input_line(line))
        .collect();

    info!("Part1: {}", part1(&sensors, row));
    info!("Part2: {}", part2(&sensors, max_index));
}

fn part1(sensors: &[Sensor], row: i32) -> usize {
    let mut blocked_spaces = HashSet::new();

    for sensor in sensors {
        let dist_to_row = sensor.pos.1.abs_diff(row);
        let remaining_dist = if sensor.beacon_dist >= dist_to_row {
            sensor.beacon_dist + 1 - dist_to_row
        } else {
            0
        } as i32;

        for x in sensor.pos.0 - (remaining_dist - 1)..=sensor.pos.0 + (remaining_dist - 1) {
            blocked_spaces.insert(x);
        }
    }

    for sensor in sensors {
        if sensor.nearest_beacon.1 == row {
            blocked_spaces.remove(&sensor.nearest_beacon.0);
        }
    }

    blocked_spaces.len()
}

fn part2(sensors: &[Sensor], max_index: i32) -> i64 {
    let row_space = IntervalSet::from_interval(0, max_index);
    for row in 0..=max_index {
        if row % 1000 == 0 {
            debug!("{}", row);
        }
        let blocked_space = sensors
            .iter()
            .fold(IntervalSet::new(), |acc: IntervalSet<i32>, sensor| {
                acc.union(&sensor.blocked_interval(row))
            })
            .intersection(&row_space);
        trace!("{:?}", blocked_space);
        if blocked_space != row_space {
            let available_space = row_space.difference(&blocked_space);
            debug!("{:?}, {:?}", blocked_space, available_space);
            let x = available_space.get_intervals()[0].0;
            return x as i64 * 4_000_000 + row as i64;
        }
    }
    panic!()
}

struct Sensor {
    pos: (i32, i32),
    nearest_beacon: (i32, i32),
    beacon_dist: u32,
}

impl Sensor {
    fn from_input_line(line: &str) -> Self {
        let captures = LINE_REGEX.captures(line).unwrap();
        let sx: i32 = captures
            .name("sx")
            .and_then(|cap| cap.as_str().parse().ok())
            .unwrap();
        let sy: i32 = captures
            .name("sy")
            .and_then(|cap| cap.as_str().parse().ok())
            .unwrap();
        let bx: i32 = captures
            .name("bx")
            .and_then(|cap| cap.as_str().parse().ok())
            .unwrap();
        let by: i32 = captures
            .name("by")
            .and_then(|cap| cap.as_str().parse().ok())
            .unwrap();
        Sensor {
            pos: (sx, sy),
            nearest_beacon: (bx, by),
            beacon_dist: dist((sx, sy), (bx, by)),
        }
    }

    fn blocked_interval(&self, row: i32) -> IntervalSet<i32> {
        let dist_to_row = row.abs_diff(self.pos.1);
        let remaining_dist = if self.beacon_dist >= dist_to_row {
            self.beacon_dist + 1 - dist_to_row
        } else {
            0
        } as i32;
        IntervalSet::from_interval(
            self.pos.0 - remaining_dist + 1,
            self.pos.0 + remaining_dist - 1,
        )
    }
}

fn dist((px1, py1): (i32, i32), (px2, py2): (i32, i32)) -> u32 {
    px1.abs_diff(px2) + py1.abs_diff(py2)
}

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::from_str(r"Sensor at x=(?P<sx>-?\d*), y=(?P<sy>-?\d*): closest beacon is at x=(?P<bx>-?\d*), y=(?P<by>-?\d*)").unwrap()
});
