#![feature(iter_intersperse)]
use std::{collections::HashMap, fmt::Debug, thread, time::Duration};

use itertools::Itertools;
use CaveCell::{Air, Rock, Sand, Source};

fn main() {
    let cave = dbg!(Cave::from_input(include_str!("input.txt")));

    println!("Part1: {}", part1(&cave));
}

fn part1(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let mut grains = 0;
    while cave.drop_sand() {
        if grains % 5 == 0 {
            thread::sleep(Duration::from_millis(100));
            println!("{:?}", cave);
        }
        grains += 1;
    }
    grains
}

#[derive(Clone)]
struct Cave {
    layout: HashMap<(u16, u16), CaveCell>,
    minx: u16,
    maxx: u16,
    maxy: u16,
}

impl Cave {
    fn from_input(rock_structures: &str) -> Self {
        let mut layout = HashMap::new();

        for line in rock_structures.lines() {
            for ((sx, sy), (ex, ey)) in line
                .split("->")
                .map(|point_str| {
                    point_str
                        .trim()
                        .split(",")
                        .map(|xory| xory.parse().unwrap())
                        .collect_tuple::<(u16, u16)>()
                        .unwrap()
                })
                .tuple_windows()
            {
                match (sx != ex, sy != ey) {
                    (true, false) => {
                        layout.extend((sx.min(ex)..=sx.max(ex)).map(|x| ((x, sy), Rock)))
                    }
                    (false, true) => {
                        layout.extend((sy.min(ey)..=sy.max(ey)).map(|y| ((sx, y), Rock)))
                    }
                    _ => panic!(
                        "Non horizontal/vertical line found: {:?} -> {:?}",
                        (sx, sy),
                        (ex, ey)
                    ),
                }
            }
        }

        layout.insert(SOURCE, Source);

        let minx = layout.keys().min_by_key(|(x, _y)| x).unwrap().0;
        let maxx = layout.keys().max_by_key(|(x, _y)| x).unwrap().0;
        let maxy = layout.keys().max_by_key(|(_x, y)| y).unwrap().1;

        Cave {
            layout,
            minx,
            maxx,
            maxy,
        }
    }

    fn get_cell(&self, x: u16, y: u16) -> CaveCell {
        *self.layout.get(&(x, y)).unwrap_or(&Air)
    }

    // Returns whether the sand grain was stopped by the cave
    fn drop_sand(&mut self) -> bool {
        let mut current_pos = SOURCE;
        while let Some((nx, ny)) = self.next_space(current_pos) {
            if nx < self.minx || nx > self.maxx || ny > self.maxy {
                return false;
            }
            current_pos = (nx, ny);
        }
        self.layout.insert(current_pos, Sand);
        true
    }

    fn next_space(&self, (cx, cy): (u16, u16)) -> Option<(u16, u16)> {
        if self.get_cell(cx, cy + 1) == Air {
            Some((cx, cy + 1))
        } else if self.get_cell(cx - 1, cy + 1) == Air {
            Some((cx - 1, cy + 1))
        } else if self.get_cell(cx + 1, cy + 1) == Air {
            Some((cx + 1, cy + 1))
        } else {
            None
        }
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output =
            vec![vec![Air; (self.maxx - self.minx + 1) as usize]; (self.maxy + 1) as usize];

        for (&(x, y), &cell) in &self.layout {
            output[y as usize][(x - self.minx) as usize] = cell;
        }

        write!(
            f,
            "\n{}",
            Iterator::intersperse(
                output.into_iter().map(|line| {
                    line.into_iter()
                        .map(|cell| cell.to_string())
                        .collect::<String>()
                }),
                "\n".to_string(),
            )
            .collect::<String>(),
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CaveCell {
    Air,
    Rock,
    Sand,
    Source,
}

impl CaveCell {
    fn to_string(&self) -> String {
        match self {
            Air => ".",
            Rock => "#",
            Sand => "o",
            Source => "+",
        }
        .to_string()
    }
}

impl Debug for CaveCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

const SOURCE: (u16, u16) = (500, 0);
