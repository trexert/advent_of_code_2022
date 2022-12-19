#![feature(type_alias_impl_trait)]

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use lib::common_startup::startup;
use log::{debug, info, trace};

type Cell = (i8, i8, i8);

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    let droplet: Droplet = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    info!("Part1: {}", part1(&droplet));
    info!("Part2: {}", part2(&droplet));
}

fn part1(droplet: &Droplet) -> usize {
    let mut faces = droplet.len() * 6;

    debug!("{:?}", droplet);

    for cell in droplet {
        for neighbour in cell.neighbours() {
            if droplet.contains(&neighbour) {
                faces -= 1
            }
            trace!("{:?}, {:?}, {}", cell, neighbour, faces);
        }
    }

    faces
}

fn part2(droplet: &Droplet) -> usize {
    let mut droplet_to_fill = droplet.clone();
    let mut faces = droplet_to_fill.len() * 6;

    debug!("{:?}", droplet_to_fill);

    for cell in droplet {
        for neighbour in cell.neighbours() {
            if neighbour.is_bounded_by(&mut droplet_to_fill) {
                faces -= 1
            }
            trace!("{:?}, {:?}, {}", cell, neighbour, faces);
        }
    }

    faces
}

#[derive(Clone, Debug)]
struct Droplet {
    cells: HashSet<Cell>,
    min: Cell,
    max: Cell,
}

impl Droplet {
    fn len(&self) -> usize {
        self.cells.len()
    }

    fn contains(&self, value: &Cell) -> bool {
        self.cells.contains(value)
    }
}

impl FromIterator<Cell> for Droplet {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut min = (i8::MAX, i8::MAX, i8::MAX);
        let mut max = (i8::MIN, i8::MIN, i8::MIN);
        let cells = iter
            .into_iter()
            .map(|cell| {
                min.0 = min.0.min(cell.0);
                min.1 = min.1.min(cell.1);
                min.2 = min.2.min(cell.2);
                max.0 = max.0.max(cell.0);
                max.1 = max.1.max(cell.1);
                max.2 = max.2.max(cell.2);
                cell
            })
            .collect();

        Droplet { cells, min, max }
    }
}

impl<'a> IntoIterator for &'a Droplet {
    type Item = &'a Cell;

    type IntoIter = impl Iterator<Item = &'a Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

trait CellInDroplet {
    fn neighbours(&self) -> [Self; 6]
    where
        Self: Sized;

    fn is_bounded_by(&self, droplet: &mut Droplet) -> bool;
}

impl CellInDroplet for Cell {
    fn neighbours(&self) -> [Cell; 6] {
        [
            (self.0 - 1, self.1, self.2),
            (self.0 + 1, self.1, self.2),
            (self.0, self.1 - 1, self.2),
            (self.0, self.1 + 1, self.2),
            (self.0, self.1, self.2 - 1),
            (self.0, self.1, self.2 + 1),
        ]
    }

    fn is_bounded_by(&self, droplet: &mut Droplet) -> bool {
        if droplet.contains(self) {
            return true;
        }

        let mut discovered = HashSet::from([*self]);
        let mut to_explore = VecDeque::from([*self]);

        while let Some(cell) = to_explore.pop_front() {
            for neighbour in cell.neighbours() {
                if neighbour.0 < droplet.min.0
                    || neighbour.1 < droplet.min.1
                    || neighbour.2 < droplet.min.2
                    || neighbour.0 > droplet.max.0
                    || neighbour.1 > droplet.max.1
                    || neighbour.2 > droplet.max.2
                {
                    return false;
                }

                if !droplet.contains(&neighbour) && !discovered.contains(&neighbour) {
                    to_explore.push_back(neighbour);
                    discovered.insert(neighbour);
                }
            }
        }

        droplet.cells.extend(discovered.into_iter());
        return true;
    }
}
