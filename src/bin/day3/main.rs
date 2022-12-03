use std::collections::HashSet;

type Int = u16;

fn main() {
    let rucksacks: Vec<_> = include_str!("input.txt").lines().collect();

    println!("Part1: {}", part1(&rucksacks));
    println!("Part2: {}", part2(&rucksacks));
}

fn part1(rucksacks: &Vec<&str>) -> Int {
    let pockets = rucksacks.iter().map(|rucksack| {
        let mut pockets = (
            Vec::with_capacity(rucksack.len() / 2 + 1),
            Vec::with_capacity(rucksack.len() / 2 + 1),
        );
        for (i, char) in rucksack.chars().enumerate() {
            if i < rucksack.len() / 2 {
                pockets.0.push(char);
            } else {
                pockets.1.push(char);
            }
        }
        pockets
    });
    let mut result = 0;
    for (p0, p1) in pockets {
        let (p0set, p1set): (HashSet<_>, HashSet<_>) =
            (HashSet::from_iter(p0.iter()), HashSet::from_iter(p1.iter()));
        let intersection: Vec<_> = p0set.intersection(&p1set).collect();
        assert!(intersection.len() == 1);
        result += priority(**intersection[0]);
    }

    result
}

fn part2(rucksacks: &Vec<&str>) -> Int {
    let mut result = 0;

    for group in rucksacks.chunks(3) {
        let sets: Vec<HashSet<char>> = group
            .iter()
            .map(|rucksack| HashSet::from_iter(rucksack.chars()))
            .collect();

        let intersection: Vec<_> = sets[0]
            .iter()
            .filter(|c| sets.as_slice()[1..].iter().all(|s| s.contains(c)))
            .collect();

        assert!(intersection.len() == 1);

        result += priority(*intersection[0]);
    }

    result
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(item: char) -> Int {
    (ALPHABET.find(item).unwrap() + 1) as Int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('d'), 4);
        assert_eq!(priority('Z'), 52);
    }
}
