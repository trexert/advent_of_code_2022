use std::collections::HashMap;
use RPSMove::{P, R, S};

type Int = u16;

#[derive(Clone, Copy, Debug)]
enum RPSMove {
    R,
    P,
    S,
}

struct Round {
    my_move: RPSMove,
    other_move: RPSMove,
}

impl Round {
    fn score(&self) -> Int {
        match (&self.my_move, &self.other_move) {
            (R, R) => 4,
            (R, P) => 1,
            (R, S) => 7,
            (P, R) => 8,
            (P, P) => 5,
            (P, S) => 2,
            (S, R) => 3,
            (S, P) => 9,
            (S, S) => 6,
        }
    }
}

fn main() {
    let strategy: Vec<(&str, &str)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut moves = line.split(" ");
            (moves.next().unwrap(), moves.next().unwrap())
        })
        .collect();

    println!("Part1: {}", part1(&strategy));
}

fn part1(strategy: &Vec<(&str, &str)>) -> Int {
    let mapping = HashMap::from_iter(
        [("A", R), ("B", P), ("C", S), ("X", R), ("Y", P), ("Z", S)].into_iter(),
    );
    score_strategy(strategy, &mapping)
}

fn score_strategy(strategy: &Vec<(&str, &str)>, mapping: &HashMap<&str, RPSMove>) -> Int {
    strategy
        .iter()
        .map(|(other_move_str, my_move_str)| {
            Round {
                my_move: mapping[my_move_str],
                other_move: mapping[other_move_str],
            }
            .score()
        })
        .sum()
}
