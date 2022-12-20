use lib::common_startup::startup;
use log::info;

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    // info!("Part1: {}", part1());
    // info!("Part2: {}", part2());
}

fn part1() {}

fn part2() {}
