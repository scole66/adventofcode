//! # Solution for Advent of Code 2022 Day 19: Not Enough Minerals
//!
//! Ref: [Advent of Code 2022 Day 19](https://adventofcode.com/2022/day/19)
//!
use ahash::AHashMap;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u8,
    clay_robot_ore_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
    max_ore_cost: u8,
}
impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static BLUEPRINT_PATTERN: Lazy<Regex> = Lazy::new(|| {
            // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs
            // 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
            Regex::new(r"^Blueprint (?P<id>0|[1-9][0-9]*): Each ore robot costs (?P<OOC>0|[1-9][0-9]*) ore. Each clay robot costs (?P<COC>0|[1-9][0-9]*) ore. Each obsidian robot costs (?P<BOC>0|[1-9][0-9]*) ore and (?P<BCC>0|[1-9][0-9]*) clay. Each geode robot costs (?P<GOC>0|[1-9][0-9]*) ore and (?P<GBC>0|[1-9][0-9]*) obsidian.$").unwrap()
        });
        let caps = BLUEPRINT_PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Bad parse for blueprint \"{s}\""))?;
        let ore_robot_ore_cost = caps["OOC"].parse()?;
        let clay_robot_ore_cost = caps["COC"].parse()?;
        let obsidian_robot_ore_cost = caps["BOC"].parse()?;
        let obsidian_robot_clay_cost = caps["BCC"].parse()?;
        let geode_robot_ore_cost = caps["GOC"].parse()?;
        let geode_robot_obsidian_cost = caps["GBC"].parse()?;
        Ok(Blueprint {
            id: caps["id"].parse()?,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
            max_ore_cost: [
                ore_robot_ore_cost,
                clay_robot_ore_cost,
                obsidian_robot_ore_cost,
                geode_robot_ore_cost,
            ]
            .into_iter()
            .max()
            .unwrap(),
        })
    }
}

struct Blueprints(Vec<Blueprint>);
impl FromStr for Blueprints {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = Blueprints(s.lines().map(|s| s.parse()).collect::<anyhow::Result<_>>()?);
        if result.0.len() > u32::MAX as usize {
            anyhow::bail!("Too many blueprints");
        }
        if result.0.iter().enumerate().any(|(idx, bp)| idx as u32 + 1 != bp.id) {
            anyhow::bail!("Identifiers for blueprints are out of sequence");
        }
        Ok(result)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Inventory {
    ore: u8,
    clay: u8,
    geodes: u8,
    obsidian: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}
impl Inventory {
    fn collect(&self) -> Inventory {
        let mut i = *self;
        i.ore += i.ore_robots;
        i.clay += i.clay_robots;
        i.obsidian += i.obsidian_robots;
        i.geodes += i.geode_robots;
        i
    }
    fn build_ore_robot(&self, cost: u8) -> Inventory {
        let mut i = *self;
        i.ore -= cost;
        i.ore_robots += 1;
        i
    }
    fn build_clay_robot(&self, cost: u8) -> Inventory {
        let mut i = *self;
        i.ore -= cost;
        i.clay_robots += 1;
        i
    }
    fn build_obsidian_robot(&self, ore_cost: u8, clay_cost: u8) -> Inventory {
        let mut i = *self;
        i.ore -= ore_cost;
        i.clay -= clay_cost;
        i.obsidian_robots += 1;
        i
    }
    fn build_geode_robot(&self, ore_cost: u8, obsidian_cost: u8) -> Inventory {
        let mut i = *self;
        i.ore -= ore_cost;
        i.obsidian -= obsidian_cost;
        i.geode_robots += 1;
        i
    }
}

impl Blueprint {
    fn quality_level(&self) -> usize {
        self.id as usize * self.max_geode_production(24)
    }

    fn max_production_inner(
        &self,
        cache: &mut AHashMap<(u32, Inventory), usize>,
        inv: Inventory,
        time_left: u32,
    ) -> usize {
        if time_left == 0 {
            return inv.geodes as usize;
        }

        let cache_key = (time_left, inv);
        if let Some(&result) = cache.get(&cache_key) {
            return result;
        }

        // Figure out what we can build.
        let can_build_ore_robot = inv.ore >= self.ore_robot_ore_cost;
        let can_build_clay_robot = inv.ore >= self.clay_robot_ore_cost;
        let can_build_obsidian_robot =
            inv.ore >= self.obsidian_robot_ore_cost && inv.clay >= self.obsidian_robot_clay_cost;
        let can_build_geode_robot =
            inv.ore >= self.geode_robot_ore_cost && inv.obsidian >= self.geode_robot_obsidian_cost;

        // Collect resources
        let new_inventory = inv.collect();

        // Now try all the different things we can do.
        let mut m = 0;

        // Do nothing. Inventory is not further modified, but we lose a minute.
        // (Note: never do this if we can build a geode robot...)
        if !can_build_geode_robot {
            m = m.max(self.max_production_inner(cache, new_inventory, time_left - 1));
        }
        // Make an ore robot. Reduce the inventory by the robot's cost, and increase the number of robots.
        if can_build_ore_robot && !can_build_geode_robot && new_inventory.ore_robots < self.max_ore_cost {
            m = m.max(self.max_production_inner(
                cache,
                new_inventory.build_ore_robot(self.ore_robot_ore_cost),
                time_left - 1,
            ));
        }
        // Make a clay robot.
        if can_build_clay_robot && !can_build_geode_robot && new_inventory.clay_robots < self.obsidian_robot_clay_cost {
            m = m.max(self.max_production_inner(
                cache,
                new_inventory.build_clay_robot(self.clay_robot_ore_cost),
                time_left - 1,
            ));
        }
        // Make an obsidian robot.
        if can_build_obsidian_robot
            && !can_build_geode_robot
            && new_inventory.obsidian_robots < self.geode_robot_obsidian_cost
        {
            m = m.max(self.max_production_inner(
                cache,
                new_inventory.build_obsidian_robot(self.obsidian_robot_ore_cost, self.obsidian_robot_clay_cost),
                time_left - 1,
            ));
        }
        // Make a geode robot.
        if can_build_geode_robot {
            m = m.max(self.max_production_inner(
                cache,
                new_inventory.build_geode_robot(self.geode_robot_ore_cost, self.geode_robot_obsidian_cost),
                time_left - 1,
            ));
        }

        cache.insert(cache_key, m);
        m
    }
    fn max_geode_production(&self, time_left: u32) -> usize {
        let mut cache: AHashMap<(u32, Inventory), usize> = AHashMap::new();
        let result = self.max_production_inner(
            &mut cache,
            Inventory {
                ore: 0,
                clay: 0,
                geodes: 0,
                obsidian: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
            time_left,
        );
        println!("Cache grew to {} items", cache.len());
        result
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let prints = input.parse::<Blueprints>()?;
    Ok(prints.0.par_iter().map(|print| print.quality_level()).sum())
}
fn part2(input: &str) -> anyhow::Result<usize> {
    let prints = input.parse::<Blueprints>()?;
    let limit = 3.min(prints.0.len());
    let result = prints.0[0..limit]
        .par_iter()
        .map(|print| print.max_geode_production(32))
        .product::<usize>();
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    #[ignore] // takes too long on github
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 33);
    }

    #[test]
    #[ignore] // takes too long on github
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 62 * 56);
    }
}
