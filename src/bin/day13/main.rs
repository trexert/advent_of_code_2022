use std::cmp::Ordering;

use crate::TreeNode::{Branch, Leaf};
use itertools::Itertools;

fn main() {
    let packets_str = include_str!("input.txt");

    println!("Part1: {}", part1(packets_str));
    println!("Part2: {}", part2(packets_str))
}

fn part1(packets_str: &str) -> usize {
    let packet_pairs: Vec<(TreeNode, TreeNode)> = packets_str
        .split("\n\n")
        .map(|pair_str| {
            pair_str
                .lines()
                .map(|line| TreeNode::from_str(&mut line.chars()).0)
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect();

    packet_pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, (lpacket, rpacket))| if lpacket < rpacket { Some(i + 1) } else { None })
        .sum()
}

fn part2(packets_str: &str) -> usize {
    let mut all_packets: Vec<TreeNode> = packets_str
        .lines()
        .filter_map(|line| {
            if line.len() > 0 {
                Some(TreeNode::from_str(&mut line.chars()).0)
            } else {
                None
            }
        })
        .collect();

    let markers = vec![
        TreeNode::from_str(&mut "[[2]]".chars()).0,
        TreeNode::from_str(&mut "[[6]]".chars()).0,
    ];

    all_packets.append(&mut markers.clone());

    all_packets.sort();

    let marker_positions: Vec<_> = all_packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if markers.contains(&packet) {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect();
    marker_positions.into_iter().product()
}

#[derive(Clone, Debug)]
enum TreeNode {
    Branch(Vec<TreeNode>),
    Leaf(u8),
}

impl TreeNode {
    fn from_str(input: &mut impl Iterator<Item = char>) -> (Self, bool) {
        let mut digits: Vec<char> = vec![];
        let mut children = vec![];
        while let Some(c) = input.next() {
            match c {
                '[' => loop {
                    let (child, more_to_come) = TreeNode::from_str(input);
                    children.push(child);
                    if !more_to_come {
                        match input.next() {
                            Some(',') => return (Branch(children), true),
                            Some(']') | None => return (Branch(children), false),
                            Some(c) => panic!("Unexpected char for situation: {}", c),
                        }
                    }
                },
                ']' => match (digits.is_empty(), children.is_empty()) {
                    (false, true) => {
                        return (
                            Leaf(digits.into_iter().collect::<String>().parse().unwrap()),
                            false,
                        )
                    }
                    (true, _) => return (Branch(children), false),
                    _ => panic!(
                        "Unexpected both children and digits: {:?}, {:?}",
                        digits, children
                    ),
                },
                ',' => match (digits.is_empty(), children.is_empty()) {
                    (false, true) => {
                        return (
                            Leaf(digits.into_iter().collect::<String>().parse().unwrap()),
                            true,
                        )
                    }
                    (true, _) => return (Branch(children), true),
                    _ => panic!(
                        "Unexpected both children and digits: {:?}, {:?}",
                        digits, children
                    ),
                },
                digit => digits.push(digit),
            }
        }
        panic!("Unexpected end of string");
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for TreeNode {}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Leaf(lval), Leaf(rval)) => lval.cmp(&rval),
            (Branch(_children), Leaf(val)) => self.cmp(&Branch(vec![Leaf(*val)])),
            (Leaf(val), Branch(_children)) => Branch(vec![Leaf(*val)]).cmp(other),
            (Branch(lchildren), Branch(rchildren)) => {
                for (lchild, rchild) in lchildren.into_iter().zip(rchildren) {
                    match lchild.cmp(&rchild) {
                        Ordering::Equal => (),
                        order => return order,
                    }
                }
                lchildren.len().cmp(&rchildren.len())
            }
        }
    }
}
