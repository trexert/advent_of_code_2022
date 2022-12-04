use std::cmp::Ordering;

type Int = u8;

fn main() {
    let paired_assignments: Vec<((Int, Int), (Int, Int))> = include_str!("input.txt")
        .lines()
        .map(|line| assemble_pair(line))
        .collect();

    println!("Part1: {}", part1(&paired_assignments));
    println!("Part2: {}", part2(&paired_assignments));
}

fn part1(paired_assignments: &Vec<((Int, Int), (Int, Int))>) -> usize {
    paired_assignments
        .iter()
        .filter(|(p0, p1)| p1.1 <= p0.1)
        .count() as usize
}

fn part2(paired_assignments: &Vec<((Int, Int), (Int, Int))>) -> usize {
    paired_assignments
        .iter()
        .filter(|(p0, p1)| p1.0 <= p0.1)
        .count() as usize
}

fn assemble_pair(line: &str) -> ((Int, Int), (Int, Int)) {
    let mut elf_pair_iter = line.split(",").map(|range| {
        let mut range_ends = range.split("-").map(|end| end.parse::<Int>().unwrap());
        (range_ends.next().unwrap(), range_ends.next().unwrap())
    });
    let elf_pair = (elf_pair_iter.next().unwrap(), elf_pair_iter.next().unwrap());
    match elf_pair.0 .0.cmp(&elf_pair.1 .0) {
        Ordering::Less => elf_pair,
        Ordering::Greater => (elf_pair.1, elf_pair.0),
        Ordering::Equal => {
            if elf_pair.0 .1 < elf_pair.1 .1 {
                (elf_pair.1, elf_pair.0)
            } else {
                elf_pair
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_pairs() {
        assert_eq!(assemble_pair("2-4,6-8"), ((2, 4), (6, 8)));
        assert_eq!(assemble_pair("6-6,4-6"), ((4, 6), (6, 6)));
        assert_eq!(assemble_pair("4-6,4-7"), ((4, 7), (4, 6)));
    }
}
