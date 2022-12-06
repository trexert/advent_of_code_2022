fn main() {
    let signal: Vec<char> = include_str!("input.txt").chars().collect();

    println!("Part1: {}", search_for_marker(&signal, 4));
    println!("Part2: {}", search_for_marker(&signal, 14));
}

fn search_for_marker(signal: &Vec<char>, marker_size: usize) -> usize {
    signal
        .windows(marker_size)
        .enumerate()
        .filter(|&(_, packet)| check_for_marker(packet))
        .next()
        .unwrap()
        .0
        + marker_size
}

fn check_for_marker(packet: &[char]) -> bool {
    println!("{:?}", packet);
    (0..packet.len()).all(|i| ((i + 1)..packet.len()).all(|j| packet[i] != packet[j]))
}
