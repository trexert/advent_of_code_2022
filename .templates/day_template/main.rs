use lib::common_startup::startup;
use log::info;

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    // info!("Part1: {}", part1(&connections, &id_to_valve));
    // info!("Part2: {}", part2(&connections, &id_to_valve));
}

fn part1() {}

fn part2() {}
