use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use lib::common_startup::startup;
use log::{debug, info};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let cli = startup();
    let (row, input) = if cli.sample {
        (10, include_str!("sample_input.txt"))
    } else {
        (2_000_000, include_str!("input.txt"))
    };

    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| Sensor::from_input_line(line))
        .collect();

    info!("Part1: {}", part1(&sensors, row));
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

    debug!("blocked_spaces: {:?}", {
        let mut bsv = blocked_spaces.iter().collect_vec();
        bsv.sort();
        bsv
    });
    blocked_spaces.len()
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
}

fn dist((px1, py1): (i32, i32), (px2, py2): (i32, i32)) -> u32 {
    px1.abs_diff(px2) + py1.abs_diff(py2)
}

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::from_str(r"Sensor at x=(?P<sx>-?\d*), y=(?P<sy>-?\d*): closest beacon is at x=(?P<bx>-?\d*), y=(?P<by>-?\d*)").unwrap()
});
