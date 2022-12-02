type Int = u16;

fn main() {
    let strategy: Vec<(&str, &str)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut moves = line.split(" ");
            (moves.next().unwrap(), moves.next().unwrap())
        })
        .collect();

    println!("Part1: {}", part1(&strategy));
    println!("Part2: {}", part2(&strategy));
}

fn part1(strategy: &Vec<(&str, &str)>) -> Int {
    score_strategy(strategy, |round| match round {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        _ => panic!("Unexpected input"),
    })
}

fn part2(strategy: &Vec<(&str, &str)>) -> Int {
    score_strategy(strategy, |round| match round {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,
        _ => panic!("Unexpected input"),
    })
}

fn score_strategy<F: Fn((&str, &str)) -> Int>(strategy: &Vec<(&str, &str)>, scoring: F) -> Int {
    strategy.iter().map(|&round| scoring(round)).sum()
}
