use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Index, IndexMut},
};

use lib::common_startup::startup;
use log::{debug, info, trace};

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    let encrypted: Vec<isize> = input.lines().map(|line| line.parse().unwrap()).collect();

    info!("Part1: {}", part1(&encrypted));
    info!("Part2: {}", part2(&encrypted));
}

fn part1(encrypted: &[isize]) -> isize {
    let mut zero_pos = None;
    let file_len = encrypted.len() as isize;
    let mut positions: Vec<isize> = (0..file_len).collect();
    let mut to_mix: RingBuffer<_> = encrypted
        .into_iter()
        .enumerate()
        .map(|(i, &val)| {
            if val == 0 {
                assert!(zero_pos.is_none());
                zero_pos = Some(i);
            }
            (val, i)
        })
        .collect();

    mix(&mut to_mix, &mut positions);

    debug!("{:?}", to_mix);
    let final_zero_pos = positions[zero_pos.unwrap()];
    to_mix[(final_zero_pos + 1000).rem_euclid(file_len)].0
        + to_mix[(final_zero_pos + 2000).rem_euclid(file_len)].0
        + to_mix[(final_zero_pos + 3000).rem_euclid(file_len)].0
}

fn part2(encrypted: &[isize]) -> isize {
    let mut zero_pos = None;
    let file_len = encrypted.len() as isize;
    let mut positions: Vec<isize> = (0..file_len).collect();
    let mut to_mix: RingBuffer<_> = encrypted
        .into_iter()
        .enumerate()
        .map(|(i, &val)| {
            if val == 0 {
                assert!(zero_pos.is_none());
                zero_pos = Some(i);
            }
            (val * 811589153, i)
        })
        .collect();

    for _ in 0..10 {
        mix(&mut to_mix, &mut positions);
    }

    debug!("{:?}", to_mix);
    let final_zero_pos = positions[zero_pos.unwrap()];
    to_mix[(final_zero_pos + 1000).rem_euclid(file_len)].0
        + to_mix[(final_zero_pos + 2000).rem_euclid(file_len)].0
        + to_mix[(final_zero_pos + 3000).rem_euclid(file_len)].0
}

fn mix(to_mix: &mut RingBuffer<(isize, usize)>, positions: &mut Vec<isize>) {
    let file_len = positions.len() as isize;
    for i in 0..file_len as usize {
        let start_position = positions[i];
        let to_move = to_mix[start_position];
        let end_position = start_position + to_move.0 % (file_len - 1);
        match start_position.cmp(&end_position) {
            Ordering::Equal => (),
            Ordering::Less => {
                for j in start_position..end_position {
                    assert!(
                        (positions[to_mix[j + 1].1] - j - 1).rem_euclid(file_len) == 0,
                        "{}, {}",
                        positions[to_mix[j + 1].1],
                        j + 1
                    );
                    to_mix[j] = to_mix[j + 1];
                    positions[to_mix[j].1] = j;
                }
                to_mix[end_position] = to_move;
                positions[to_move.1] = end_position;
            }
            Ordering::Greater => {
                for j in (end_position..start_position).rev() {
                    assert!(
                        (positions[to_mix[j].1] - j).rem_euclid(file_len) == 0,
                        "{}, {}",
                        positions[to_mix[j].1],
                        j
                    );
                    to_mix[j + 1] = to_mix[j];
                    positions[to_mix[j + 1].1] = j + 1;
                }
                to_mix[end_position] = to_move;
                positions[to_move.1] = end_position;
            }
        }
        trace!("{:?}\n{:?}", to_mix, positions);
    }
}

struct RingBuffer<T> {
    buff: Vec<T>,
}

impl<T: Debug> Debug for RingBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.buff)
    }
}

impl<T> Index<isize> for RingBuffer<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        &self.buff[index.rem_euclid(self.buff.len() as isize) as usize]
    }
}

impl<T> IndexMut<isize> for RingBuffer<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let index = index.rem_euclid(self.buff.len() as isize) as usize;
        &mut self.buff[index]
    }
}

impl<T> FromIterator<T> for RingBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        RingBuffer {
            buff: Vec::from_iter(iter),
        }
    }
}
