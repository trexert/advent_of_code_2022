use lib::common_startup::startup;
use log::info;
use once_cell::sync::Lazy;
use regex::Regex;

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

struct Template {
    ore_bot: Recipe,
    clay_bot: Recipe,
    obsidian_bot: Recipe,
    geode_bot: Recipe,
}

impl Template {
    fn from_str(line: &str) -> Self {

    }
}

struct Recipe {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

static TEMPLATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Blueprint (?P<id>\d*): Each ore robot costs (?P<oboc>\d*) ore. Each clay robot costs (?P<cboc>\d*) ore."))
