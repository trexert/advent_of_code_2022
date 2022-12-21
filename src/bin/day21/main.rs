use std::collections::HashMap;

use lib::common_startup::startup;
use lib::op_wrapper::Op;
use log::info;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    let monkeys: HashMap<&'static str, MonkeyNum> = input
        .lines()
        .map(|line| {
            let num = MonkeyNum::from_str(line);
            (num.id, num)
        })
        .collect();

    info!("Part1: {}", part1(&monkeys));
    // info!("Part2: {}", part2());
}

fn part1(monkeys: &HashMap<&str, MonkeyNum>) -> f64 {
    let mut monkeys = monkeys.clone();
    let mut to_calc = vec!["root"];
    while let Some(current_id) = to_calc.pop() {
        match &monkeys[current_id].kind {
            MonkeyNumKind::Val(_) => (),
            MonkeyNumKind::Op(left, right, op) => match (&monkeys[left].kind, &monkeys[right].kind)
            {
                (MonkeyNumKind::Val(left_val), MonkeyNumKind::Val(right_val)) => {
                    monkeys.get_mut(current_id).unwrap().kind =
                        MonkeyNumKind::Val(op.call(*left_val, *right_val))
                }
                _ => to_calc.append(&mut vec![current_id, left, right]),
            },
        }
    }
    monkeys["root"].val()
}

fn part2(monkeys: &HashMap<&str, MonkeyNum>) -> (f64, f64) {
    let root = monkeys["root"].op();
    let left = root.0;
    let right = root.1;
}

#[derive(Clone, Debug)]
struct MonkeyNum {
    id: &'static str,
    kind: MonkeyNumKind,
}

#[derive(Clone, Debug)]
enum MonkeyNumKind {
    Op(&'static str, &'static str, Op),
    Val(f64),
}

impl MonkeyNum {
    fn from_str(line: &'static str) -> Self {
        let caps = MONKEY_REGEX.captures(line).unwrap();
        let id = caps.name("id").unwrap().as_str();
        if let Some(num) = caps.name("num") {
            Self {
                id,
                kind: MonkeyNumKind::Val(num.as_str().parse().unwrap()),
            }
        } else {
            let left = caps.name("left").unwrap().as_str();
            let right = caps.name("right").unwrap().as_str();
            let op = caps.name("op").unwrap().as_str().parse().unwrap();
            Self {
                id,
                kind: MonkeyNumKind::Op(left, right, op),
            }
        }
    }

    fn val(&self) -> f64 {
        if let MonkeyNumKind::Val(val) = self.kind {
            val
        } else {
            panic!("Tried to get val from non-val {:?}", self)
        }
    }

    fn op(&self) -> (&str, &str, Op) {
        if let MonkeyNumKind::Op(left, right, op) = self.kind {
            (left, right, op)
        } else {
            panic!("Tried to get op from non-op {:?}", self)
        }
    }
}

static MONKEY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<id>[a-z]{4}): (?:(?P<left>[a-z]{4}) (?P<op>[\+\-\*/]) (?P<right>[a-z]{4})|(?P<num>\d*))").unwrap()
});
