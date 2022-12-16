use std::collections::{HashMap, HashSet};

use lib::{common_startup::startup, floydwarshall};
use log::{debug, info, log_enabled, trace, Level};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    let valves: Vec<Valve> = input
        .lines()
        .map(|line| Valve::from_input_line(line))
        .collect();
    let id_to_valve: HashMap<String, &Valve> = valves
        .iter()
        .map(|valve| (valve.id.clone(), valve))
        .collect();

    let connections: HashMap<String, HashMap<String, i32>> = get_route_dists(&valves);

    info!("Part1: {}", part1(&connections, &id_to_valve));
    info!("Part2: {}", part2(&connections, &id_to_valve));
}

fn get_route_dists(valves: &[Valve]) -> HashMap<String, HashMap<String, i32>> {
    let valves_we_care_about: Vec<&Valve> = valves
        .iter()
        .filter(|valve| valve.flow_rate > 0 || valve.id == "AA")
        .collect();
    let mut connections_we_care_about = Vec::with_capacity(valves_we_care_about.len().pow(2));
    for v1 in valves_we_care_about.iter() {
        for v2 in valves_we_care_about.iter() {
            if v1 != v2 {
                connections_we_care_about.push((v1.id.clone(), v2.id.clone()));
            }
        }
    }
    let all_edges: HashMap<String, Vec<String>> = valves
        .iter()
        .map(|valve| (valve.id.clone(), valve.neighbours.clone()))
        .collect();
    let dists: Vec<Option<i32>> =
        floydwarshall::solve_uniform_edges(&all_edges, &connections_we_care_about);

    let mut connections: HashMap<String, HashMap<String, i32>> =
        HashMap::with_capacity(valves_we_care_about.len());
    for ((start, end), dist) in connections_we_care_about
        .iter()
        .zip(dists)
        .map(|(route, dist)| (route, dist.unwrap()))
    {
        connections
            .entry(start.clone())
            .or_default()
            .insert(end.clone(), dist);
    }

    debug!("{:?} - {}", connections, connections.len());

    connections
}

fn part1(
    connections: &HashMap<String, HashMap<String, i32>>,
    id_to_valve: &HashMap<String, &Valve>,
) -> i32 {
    let mut max_pressure_released = 0;
    let mut remaining_paths = vec![Path::new("AA", 30)];

    while let Some(current_path) = remaining_paths.pop() {
        for (next_valve, dist) in &connections[&current_path.current_valve] {
            if let Some(new_path) = current_path.with_extra_step(id_to_valve[next_valve], *dist) {
                max_pressure_released = new_path.pressure_released.max(max_pressure_released);
                remaining_paths.push(new_path)
            }
        }
    }

    max_pressure_released
}

fn part2(
    connections: &HashMap<String, HashMap<String, i32>>,
    id_to_valve: &HashMap<String, &Valve>,
) -> i32 {
    let max_cost = connections
        .values()
        .flat_map(|neighbours| neighbours.values())
        .max()
        .unwrap()
        + 1;

    let mut max_pressure_released = 0;
    let mut remaining_paths = vec![(Path::new("AA", 26), Path::new("AA", 26))];
    let all_valves: HashSet<String> = connections.keys().map(|valve| valve.clone()).collect();

    let mut i = 0;
    while let Some((p1, p2)) = remaining_paths.pop() {
        if p1.time_remaining > max_cost && p2.time_remaining > max_cost {
            let remaining_valves: HashSet<_> = all_valves.difference(&p1.visited).collect();
            for &v1 in remaining_valves.iter() {
                for &v2 in remaining_valves.iter() {
                    if v1 > v2 {
                        let p1v1: i32 = connections[&p1.current_valve][v1];
                        let p1v2: i32 = connections[&p1.current_valve][v2];
                        let p2v1: i32 = connections[&p2.current_valve][v1];
                        let p2v2: i32 = connections[&p2.current_valve][v2];
                        let (new_p1, new_p2) = if p1v1 + p2v2 < p1v2 + p2v1 {
                            (
                                p1.with_extra_step(id_to_valve[v1], p1v1)
                                    .unwrap()
                                    .with_updated_visited(v2),
                                p2.with_extra_step(id_to_valve[v2], p2v2)
                                    .unwrap()
                                    .with_updated_visited(v1),
                            )
                        } else {
                            (
                                p1.with_extra_step(id_to_valve[v2], p1v2)
                                    .unwrap()
                                    .with_updated_visited(v1),
                                p2.with_extra_step(id_to_valve[v1], p2v1)
                                    .unwrap()
                                    .with_updated_visited(v2),
                            )
                        };
                        max_pressure_released = max_pressure_released
                            .max(new_p1.pressure_released + new_p2.pressure_released);
                        remaining_paths.push((new_p1, new_p2));
                    }
                }
            }
        } else {
            for (moving, waiting) in [(&p1, &p2), (&p2, &p1)] {
                for (next_valve, dist) in &connections[&moving.current_valve] {
                    if let Some(new_path) = moving.with_extra_step(id_to_valve[next_valve], *dist) {
                        max_pressure_released = max_pressure_released
                            .max(new_path.pressure_released + waiting.pressure_released);
                        remaining_paths.push((new_path, waiting.with_updated_visited(next_valve)));
                    }
                }
            }
        }

        if log_enabled!(Level::Trace) {
            i += 1;
            if i % 100_000 == 0 {
                trace!(
                    "{:?}, {}",
                    remaining_paths
                        .iter()
                        .position(|(p1, _p2)| p1.visited.len() > 3),
                    remaining_paths.len(),
                );
            }
        }
    }

    max_pressure_released
}

#[derive(Clone)]
struct Path {
    visited: HashSet<String>,
    current_valve: String,
    time_remaining: i32,
    pressure_released: i32,
}

impl Path {
    fn new(starting_location: &str, total_time: i32) -> Self {
        let visited: HashSet<String> = HashSet::from([starting_location.to_owned()]);
        let current_valve = starting_location.to_owned();
        let time_remaining = total_time;
        let pressure_released = 0;
        Path {
            visited,
            current_valve,
            time_remaining,
            pressure_released,
        }
    }

    fn with_extra_step(&self, next_valve: &Valve, dist: i32) -> Option<Self> {
        let time_remaining = self.time_remaining - dist - 1;
        if !self.visited.contains(&next_valve.id) && time_remaining > 0 {
            let mut visited = self.visited.clone();
            visited.insert(next_valve.id.to_owned());
            let pressure_released =
                self.pressure_released + next_valve.release_pressure(time_remaining);
            return Some(Path {
                visited,
                current_valve: next_valve.id.to_owned(),
                time_remaining,
                pressure_released,
            });
        } else {
            return None;
        }
    }
    fn with_updated_visited(&self, extra_valve: &str) -> Self {
        assert!(!self.visited.contains(extra_valve));
        let mut visited = self.visited.clone();
        visited.insert(extra_valve.to_owned());
        Path {
            visited,
            ..self.clone()
        }
    }
}

#[derive(PartialEq, Eq)]
struct Valve {
    id: String,
    flow_rate: i32,
    neighbours: Vec<String>,
}

impl Valve {
    fn from_input_line(line: &str) -> Self {
        trace!("{}", line);
        let captures = LINE_REGEX.captures(line).unwrap();
        Valve {
            id: captures.name("id").unwrap().as_str().to_string(),
            flow_rate: captures.name("fr").unwrap().as_str().parse().unwrap(),
            neighbours: captures
                .name("neighbours")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|neighbour| neighbour.to_string())
                .collect(),
        }
    }

    fn release_pressure(&self, remaining_time: i32) -> i32 {
        remaining_time * self.flow_rate
    }
}

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Valve (?P<id>[A-Z]{2}) has flow rate=(?P<fr>\d*); tunnels? leads? to valves? (?P<neighbours>.*)").unwrap()
});
