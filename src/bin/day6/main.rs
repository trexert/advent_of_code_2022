#![feature(test)]

extern crate test;

use std::collections::{BTreeSet, HashSet};

use lib::LetterSet;

fn main() {
    let signal: Vec<char> = include_str!("input.txt").chars().collect();

    println!(
        "Part1: {}",
        search_for_marker(&signal, 4, check_for_marker_naive).unwrap()
    );
    println!(
        "Part2: {}",
        search_for_marker(&signal, 14, check_for_marker_naive).unwrap()
    );
}

fn search_for_marker<F>(signal: &[char], marker_size: usize, f: F) -> Option<usize>
where
    F: Fn(&[char]) -> bool,
{
    signal
        .windows(marker_size)
        .position(|packet| f(packet))
        .map(|i| i + marker_size)
}

fn check_for_marker_naive(packet: &[char]) -> bool {
    (0..packet.len()).all(|i| ((i + 1)..packet.len()).all(|j| packet[i] != packet[j]))
}

#[allow(dead_code)]
fn check_for_marker_hashset(packet: &[char]) -> bool {
    packet.iter().collect::<HashSet<_>>().len() == packet.len()
}

#[allow(dead_code)]
fn check_for_marker_letterset(packet: &[char]) -> bool {
    LetterSet::from(packet).len() == packet.len()
}

#[allow(dead_code)]
fn check_for_marker_btree(packet: &[char]) -> bool {
    packet.iter().collect::<BTreeSet<_>>().len() == packet.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test::bench::black_box;
    use test::Bencher;

    #[bench]
    fn bench4_check_for_marker_naive(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                4,
                check_for_marker_naive,
            ))
        });
    }

    #[bench]
    fn bench4_check_for_marker_hashset(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                4,
                check_for_marker_hashset,
            ))
        });
    }

    #[bench]
    fn bench4_check_for_marker_letterset(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                4,
                check_for_marker_btree,
            ))
        });
    }

    #[bench]
    fn bench4_check_for_marker_btree(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                4,
                check_for_marker_letterset,
            ))
        });
    }

    #[bench]
    fn bench14_check_for_marker_naive(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                14,
                check_for_marker_naive,
            ))
        });
    }

    #[bench]
    fn bench14_check_for_marker_hashset(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                14,
                check_for_marker_hashset,
            ))
        });
    }

    #[bench]
    fn bench14_check_for_marker_letterset(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                14,
                check_for_marker_letterset,
            ))
        });
    }

    #[bench]
    fn bench14_check_for_marker_btree(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                14,
                check_for_marker_btree,
            ))
        });
    }

    #[bench]
    fn bench100_check_for_marker_naive(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                100,
                check_for_marker_naive,
            ))
        });
    }

    #[bench]
    fn bench100_check_for_marker_letterset(b: &mut Bencher) {
        let signal: Vec<char> = include_str!("input.txt").chars().collect();
        b.iter(|| {
            black_box(search_for_marker(
                black_box(&signal),
                100,
                check_for_marker_letterset,
            ))
        });
    }
}
