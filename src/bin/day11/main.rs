use std::{cmp::Reverse, collections::VecDeque, ops};

use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let monkeys_str = include_str!("input.txt");

    println!("Part1: {}", part1(monkeys_str));
    println!("Part2: {}", part2(monkeys_str));
}

fn part1(monkeys_str: &str) -> u64 {
    let mut monkeys: Vec<_> = monkeys_str
        .split("\n\n")
        .map(|monkey_str| Monkey::from_str(monkey_str))
        .collect();

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            for (new_monkey, worry) in monkeys[monkey_id].inspect_items(|old_worry| old_worry / 3) {
                monkeys[new_monkey].items.push_back(worry);
            }
        }
    }
    monkeys.sort_by_key(|monkey| Reverse(monkey.total_inspections));
    monkeys[0].total_inspections * monkeys[1].total_inspections
}

fn part2(monkeys_str: &str) -> u64 {
    let mut monkeys: Vec<_> = monkeys_str
        .split("\n\n")
        .map(|monkey_str| Monkey::from_str(monkey_str))
        .collect();

    let big_mod = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test_number);

    for _round in 0..10_000 {
        for monkey_id in 0..monkeys.len() {
            for (new_monkey, worry) in
                monkeys[monkey_id].inspect_items(|old_worry| old_worry % big_mod)
            {
                monkeys[new_monkey].items.push_back(worry);
            }
        }
    }
    monkeys.sort_by_key(|monkey| Reverse(monkey.total_inspections));
    monkeys[0].total_inspections * monkeys[1].total_inspections
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test_number: u64,
    true_monkey: usize,
    false_monkey: usize,
    total_inspections: u64,
}

impl Monkey {
    fn from_str(str_in: &str) -> Self {
        let mut lines = str_in.lines();
        assert!(MONKEY_REGEX.is_match(lines.next().unwrap()));
        Monkey {
            items: Self::parse_items(lines.next().unwrap()),
            operation: Self::parse_operation(lines.next().unwrap()),
            test_number: Self::parse_test(lines.next().unwrap()),
            true_monkey: Self::parse_handover(lines.next().unwrap()),
            false_monkey: Self::parse_handover(lines.next().unwrap()),
            total_inspections: 0,
        }
    }

    fn inspect_items<F>(&mut self, worry_decreaser: F) -> Vec<(usize, u64)>
    where
        F: Fn(u64) -> u64,
    {
        self.items
            .drain(..)
            .map(|item| {
                self.total_inspections += 1;
                let worry = worry_decreaser((self.operation)(item));
                let new_monkey = if worry % self.test_number == 0 {
                    self.true_monkey
                } else {
                    self.false_monkey
                };
                (new_monkey, worry)
            })
            .collect()
    }

    fn parse_items(items_line: &str) -> VecDeque<u64> {
        let items_match = ITEMS_REGEX
            .captures(items_line)
            .unwrap()
            .name("nums")
            .unwrap()
            .as_str();
        items_match
            .split(",")
            .map(|item| item.trim().parse().unwrap())
            .collect()
    }

    fn parse_operation(op_line: &str) -> Box<dyn Fn(u64) -> u64> {
        let captures = OP_REGEX.captures(op_line).unwrap();
        let op = match captures.name("op").unwrap().as_str() {
            "*" => ops::Mul::mul,
            "+" => ops::Add::add,
            not_operator => panic!("Unexpected operator {}", not_operator),
        };

        let arg_str = captures.name("arg").unwrap().as_str();
        let arg: Result<u64, _> = arg_str.parse();

        if let Ok(num) = arg {
            Box::new(move |old| op(old, num))
        } else {
            assert!(arg_str == "old");
            Box::new(move |old| op(old, old))
        }
    }

    fn parse_test(test_line: &str) -> u64 {
        TEST_REGEX
            .captures(test_line)
            .unwrap()
            .name("num")
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    }

    fn parse_handover(handover_line: &str) -> usize {
        HANDOVER_REGEX
            .captures(handover_line)
            .unwrap()
            .name("num")
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    }
}

static MONKEY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Monkey \d:$").unwrap());
static ITEMS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^  Starting items: (?P<nums>[\d, ]*)$").unwrap());
static OP_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^  Operation: new = old (?P<op>[*+]) (?P<arg>.*)$").unwrap());
static TEST_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^  Test: divisible by (?P<num>\d*)").unwrap());
static HANDOVER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^    If (true|false): throw to monkey (?P<num>\d*)").unwrap());
