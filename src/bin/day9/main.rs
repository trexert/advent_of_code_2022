use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let directions: Vec<(char, u8)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut split_line = line.split(" ");
            (
                split_line.next().and_then(|s| s.chars().next()).unwrap(),
                split_line.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    println!("Part1: {}", part1(&directions));
    println!("Part2: {}", part2(&directions));
}

fn part1(directions: &[(char, u8)]) -> usize {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut t_visited: HashSet<(i32, i32)> = HashSet::new();

    for (direction, count) in directions {
        for _ in 0..*count {
            match direction {
                'U' => {
                    h.1 += 1;
                }
                'D' => {
                    h.1 -= 1;
                }
                'R' => {
                    h.0 += 1;
                }
                'L' => {
                    h.0 -= 1;
                }
                _ => panic!("Unexpected direction"),
            }
            t = move_tail(h, t);
            t_visited.insert(t);
            // println!("{:?}", t_visited);
        }
    }

    t_visited.len()
}

fn part2(directions: &[(char, u8)]) -> usize {
    let mut rope = [(0, 0); 10];
    let mut t_visited: HashSet<(i32, i32)> = HashSet::new();

    for (direction, count) in directions {
        for _ in 0..*count {
            match direction {
                'U' => {
                    rope[0].1 += 1;
                }
                'D' => {
                    rope[0].1 -= 1;
                }
                'R' => {
                    rope[0].0 += 1;
                }
                'L' => {
                    rope[0].0 -= 1;
                }
                _ => panic!("Unexpected direction"),
            }
            for knot in 0..(rope.len() - 1) {
                rope[knot + 1] = move_tail(rope[knot], rope[knot + 1]);
            }
            t_visited.insert(rope[9]);
        }
    }

    t_visited.len()
}

fn move_tail(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    let mut new_t = t;
    if h.0.abs_diff(t.0) > 1 {
        match h.0.cmp(&t.0) {
            Ordering::Greater => new_t.0 += 1,
            Ordering::Less => new_t.0 -= 1,
            Ordering::Equal => panic!("Unexpected equal cmp"),
        };
        match h.1.cmp(&t.1) {
            Ordering::Greater => new_t.1 += 1,
            Ordering::Less => new_t.1 -= 1,
            Ordering::Equal => (),
        };
    } else if h.1.abs_diff(t.1) > 1 {
        match h.1.cmp(&t.1) {
            Ordering::Greater => new_t.1 += 1,
            Ordering::Less => new_t.1 -= 1,
            Ordering::Equal => panic!("Unexpected equal cmp"),
        };
        match h.0.cmp(&t.0) {
            Ordering::Greater => new_t.0 += 1,
            Ordering::Less => new_t.0 -= 1,
            Ordering::Equal => (),
        };
    }

    new_t
}
