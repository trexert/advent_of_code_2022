fn main() {
    let signal: Vec<char> = include_str!("input.txt").chars().collect();

    println!("Part1: {}", part1(&signal));
}

fn part1(signal: &Vec<char>) -> usize {
    signal
        .windows(SIZE)
        .enumerate()
        .filter(|&(_, packet)| check_for_marker(packet))
        .next()
        .unwrap()
        .0 + SIZE
}

fn check_for_marker(packet: &[char]) -> bool {
    assert!(packet.len() == SIZE);
    println!("{:?}", packet);
    (0..SIZE).all(|i| ((i + 1)..SIZE).all(|j| packet[i] != packet[j]))
}

const SIZE: usize = 4;
