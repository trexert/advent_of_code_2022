use std::{error::Error, fmt::Display, str::FromStr};

use anyhow::Result;
use lib::common_startup::startup;
use log::{info, trace, Level};
use once_cell::sync::Lazy;
use regex::Regex;
use BotType::*;

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    let templates: Vec<Template> = input.lines().map(|line| line.parse().unwrap()).collect();

    info!("Part1: {}", part1(&templates));
    // info!("Part2: {}", part2());
}

fn part1(templates: &Vec<Template>) -> u8 {
    templates
        .into_iter()
        .map(|template| template.get_max_geodes())
        .max()
        .unwrap()
}

fn part2() {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MiningState {
    time: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
    bots: [u8; BOT_TYPES],
}

impl MiningState {
    fn new() -> Self {
        MiningState {
            time: 24,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            bots: [1, 0, 0, 0],
        }
    }

    fn with_extra_bot(&self, bot_recipe: Recipe) -> Option<MiningState> {
        if self.ore >= bot_recipe.ore
            && self.clay >= bot_recipe.clay
            && self.obsidian >= bot_recipe.obsidian
        {
            let mut bots = self.bots;
            bots[bot_recipe.bot_type as usize] += 1;
            Some(MiningState {
                time: self.time - 1,
                ore: self.ore - bot_recipe.ore + self.bots[Ore as usize],
                clay: self.clay - bot_recipe.clay + self.bots[Clay as usize],
                obsidian: self.obsidian - bot_recipe.obsidian + self.bots[Obsidian as usize],
                geodes: self.geodes + self.bots[Geode as usize],
                bots,
            })
        } else {
            None
        }
    }

    fn with_wait(&self) -> MiningState {
        MiningState {
            time: self.time - 1,
            ore: self.ore + self.bots[Ore as usize],
            clay: self.clay + self.bots[Clay as usize],
            obsidian: self.obsidian + self.bots[Clay as usize],
            geodes: self.geodes + self.bots[Geode as usize],
            bots: self.bots,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Template {
    id: u8,
    ore_bot: Recipe,
    clay_bot: Recipe,
    obsidian_bot: Recipe,
    geode_bot: Recipe,
}

impl Template {
    fn get_max_geodes(&self) -> u8 {
        let mut max_geodes = 0;
        let mut states = vec![MiningState::new()];
        let mut iters = 0;
        while let Some(current_state) = states.pop() {
            if current_state.time <= 0 {
                max_geodes = max_geodes.max(current_state.geodes)
            } else {
                let mut new_states: Vec<_> = ALL_TYPES
                    .iter()
                    .filter_map(|bot_type| current_state.with_extra_bot(self.recipe(*bot_type)))
                    .collect();

                if new_states.len() < BOT_TYPES {
                    new_states.push(current_state.with_wait());
                }

                states.append(&mut new_states);
            }
            if log::log_enabled!(Level::Trace) {
                if iters % 5_000_000 == 0 {
                    trace!(
                        "{}: {}\n{:?}",
                        iters,
                        states.len(),
                        &states[..10.min(states.len())]
                    );
                }
                iters += 1;
            }
        }

        max_geodes
    }

    fn recipe(&self, bot_type: BotType) -> Recipe {
        match bot_type {
            Ore => self.ore_bot,
            Clay => self.clay_bot,
            Obsidian => self.obsidian_bot,
            Geode => self.geode_bot,
        }
    }
}

impl FromStr for Template {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = TEMPLATE_REGEX.captures(s).ok_or(TemplateMatchError {})?;
        let id: u8 = captures
            .name("id")
            .ok_or(TemplateMatchError {})?
            .as_str()
            .parse()?;
        let oboc: u8 = captures
            .name("oboc")
            .ok_or(TemplateMatchError {})?
            .as_str()
            .parse()?;
        let cboc: u8 = captures
            .name("cboc")
            .ok_or(TemplateMatchError {})?
            .as_str()
            .parse()?;
        let obboc: u8 = captures
            .name("obboc")
            .ok_or(TemplateMatchError {})?
            .as_str()
            .parse()?;
        let obbcc: u8 = captures
            .name("obbcc")
            .ok_or(TemplateMatchError {})?
            .as_str()
            .parse()?;
        let gboc: u8 = captures
            .name("gboc")
            .ok_or(TemplateMatchError {})?
            .as_str()
            .parse()?;
        let gbobc: u8 = captures
            .name("gbobc")
            .ok_or(TemplateMatchError {})?
            .as_str()
            .parse()?;
        Ok(Template {
            id,
            ore_bot: Recipe {
                ore: oboc,
                clay: 0,
                obsidian: 0,
                bot_type: Ore,
            },
            clay_bot: Recipe {
                ore: cboc,
                clay: 0,
                obsidian: 0,
                bot_type: Clay,
            },
            obsidian_bot: Recipe {
                ore: obboc,
                clay: obbcc,
                obsidian: 0,
                bot_type: Obsidian,
            },
            geode_bot: Recipe {
                ore: gboc,
                clay: 0,
                obsidian: gbobc,
                bot_type: Geode,
            },
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Recipe {
    ore: u8,
    clay: u8,
    obsidian: u8,
    bot_type: BotType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const BOT_TYPES: usize = 4;
const ALL_TYPES: [BotType; BOT_TYPES] = [Ore, Clay, Obsidian, Geode];

static TEMPLATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"Blueprint (?P<id>\d*): Each ore robot costs (?P<oboc>\d*) ore\. Each clay robot costs (?P<cboc>\d*) ore\. Each obsidian robot costs (?P<obboc>\d*) ore and (?P<obbcc>\d*) clay\. Each geode robot costs (?P<gboc>\d*) ore and (?P<gbobc>\d*) obsidian.",
    ).unwrap()
});

#[derive(Debug, PartialEq, Eq)]
struct TemplateMatchError {}

impl Display for TemplateMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse template")
    }
}

impl Error for TemplateMatchError {}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn parse_template() {
        let expected = Template {
            id: 83,
            ore_bot: Recipe {
                ore: 4,
                clay: 0,
                obsidian: 0,
                bot_type: Ore,
            },
            clay_bot: Recipe {
                ore: 6,
                clay: 0,
                obsidian: 0,
                bot_type: Clay,
            },
            obsidian_bot: Recipe {
                ore: 2,
                clay: 13,
                obsidian: 0,
                bot_type: Obsidian,
            },
            geode_bot: Recipe {
                ore: 9,
                clay: 0,
                obsidian: 5,
                bot_type: Geode,
            },
        };
        let actual: Result<Template, _> = "Blueprint 83: Each ore robot costs 4 ore. Each clay robot costs 6 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 9 ore and 5 obsidian.".parse();

        assert_eq!(expected, actual.unwrap());
    }
}
