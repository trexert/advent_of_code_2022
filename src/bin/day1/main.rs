use std::cmp::Reverse;

type Int = u32;

fn main() {
    let inventories = include_str!("input.txt")
        .split("\n\n")
        .map(|inventory_str| {
            inventory_str
                .lines()
                .map(|calories_str| calories_str.parse::<Int>().unwrap())
                .collect()
        })
        .collect();

    println!("Part1: {}", part1(&inventories));
    println!("Part2: {}", part2(&inventories));
}

fn part1(inventories: &Vec<Vec<Int>>) -> Int {
    inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .unwrap()
}

fn part2(inventories: &Vec<Vec<Int>>) -> Int {
    let mut inventory_sums: Vec<Int> = inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();

    inventory_sums.sort_unstable_by_key(|&inventory_sum| Reverse(inventory_sum));

    inventory_sums.iter().take(3).sum()
}
