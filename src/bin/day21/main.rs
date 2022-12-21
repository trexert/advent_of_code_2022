use std::collections::HashMap;

use lib::common_startup::startup;
use lib::op_wrapper::Op;
use log::{debug, info, trace, Level};
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
    info!("Part2: {:?}", part2(&monkeys));
}

fn part1(monkeys: &HashMap<&str, MonkeyNum>) -> f64 {
    let mut monkeys = monkeys.clone();
    calculate_monkey("root", &mut monkeys, None)
}

fn part2(monkeys: &HashMap<&str, MonkeyNum>) -> f64 {
    let root = monkeys["root"].op();
    let mut monkeys0 = monkeys.clone();
    let left0 = calculate_monkey(root.0, &mut monkeys0, Some(0.0));
    let right0 = calculate_monkey(root.1, &mut monkeys0, Some(0.0));
    let mut monkeys1 = monkeys.clone();
    let left1 = calculate_monkey(root.0, &mut monkeys1, Some(1_000_000_000_000.0));
    let right1 = calculate_monkey(root.1, &mut monkeys1, Some(1_000_000_000_000.0));
    let diff0 = right0 - left0;
    let diff1 = right1 - left1;
    let humn = diff0 / ((diff0 - diff1) / 1_000_000_000_000.0);
    if log::log_enabled!(Level::Debug) {
        let mut monkeys2 = monkeys.clone();
        let left2 = calculate_monkey(root.0, &mut monkeys2, Some(humn));
        let right2 = calculate_monkey(root.1, &mut monkeys2, Some(humn));
        debug!("{}, {}", left2, right2);
    }
    humn
}

fn calculate_monkey(
    monkey_id: &str,
    monkeys: &mut HashMap<&str, MonkeyNum>,
    humn: Option<f64>,
) -> f64 {
    let mut to_calc = vec![monkey_id];
    if let Some(val) = humn {
        monkeys.get_mut("humn").unwrap().kind = MonkeyNumKind::Val(val);
    }
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
    monkeys[monkey_id].val()
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

    fn op(&self) -> (&'static str, &'static str, Op) {
        if let MonkeyNumKind::Op(left, right, op) = self.kind.clone() {
            (left, right, op)
        } else {
            panic!("Tried to get op from non-op {:?}", self)
        }
    }
}

static MONKEY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<id>[a-z]{4}): (?:(?P<left>[a-z]{4}) (?P<op>[\+\-\*/]) (?P<right>[a-z]{4})|(?P<num>[-\d\.]*))").unwrap()
});
