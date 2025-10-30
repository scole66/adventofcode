//! # Solution for Advent of Code 2015 Day 21:
//!
//! Ref: [Advent of Code 2015 Day 21](https://adventofcode.com/2015/day/21)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;
use std::sync::LazyLock as Lazy;

const EXPECT_RE: &str = "compiled patterns shouldn't fail";

struct Stats {
    hp: i64,
    damage: i64,
    armor: i64,
}

struct Input {
    opponent: Stats,
}

enum Line {
    HitPoints(i64),
    Damage(i64),
    Armor(i64),
}
impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        static PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(?<name>Hit Points|Damage|Armor): (?<value>\d+)$").expect(EXPECT_RE));
        let capture = PATTERN.captures(s).ok_or_else(|| anyhow!("bad input line"))?;
        let value = capture["value"].parse::<i64>()?;
        match &capture["name"] {
            "Hit Points" => Ok(Line::HitPoints(value)),
            "Damage" => Ok(Line::Damage(value)),
            "Armor" => Ok(Line::Armor(value)),
            _ => unreachable!(),
        }
    }
}
impl FromStr for Stats {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut hp = None;
        let mut damage = None;
        let mut armor = None;
        for line in s.lines() {
            match Line::from_str(line)? {
                Line::HitPoints(value) => hp = Some(value),
                Line::Damage(value) => damage = Some(value),
                Line::Armor(value) => armor = Some(value),
            }
        }
        Ok(Stats {
            hp: hp.ok_or_else(|| anyhow!("missing hit points"))?,
            damage: damage.ok_or_else(|| anyhow!("missing damage"))?,
            armor: armor.ok_or_else(|| anyhow!("missing armor"))?,
        })
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let opponent = Stats::from_str(s)?;
        Ok(Input { opponent })
    }
}

impl Stats {
    fn would_beat(&self, other: &Self) -> bool {
        let mut self_hp = self.hp;
        let mut other_hp = other.hp;
        while self_hp > 0 && other_hp > 0 {
            other_hp -= (self.damage - other.armor).max(1);
            if other_hp <= 0 {
                return true;
            }
            self_hp -= (other.damage - self.armor).max(1);
        }
        false
    }
}

// Weapons:    Cost  Damage  Armor
// Dagger        8     4       0
// Shortsword   10     5       0
// Warhammer    25     6       0
// Longsword    40     7       0
// Greataxe     74     8       0
//
// Armor:      Cost  Damage  Armor
// Leather      13     0       1
// Chainmail    31     0       2
// Splintmail   53     0       3
// Bandedmail   75     0       4
// Platemail   102     0       5
//
// Rings:      Cost  Damage  Armor
// Damage +1    25     1       0
// Damage +2    50     2       0
// Damage +3   100     3       0
// Defense +1   20     0       1
// Defense +2   40     0       2
// Defense +3   80     0       3

#[derive(Debug, Copy, Clone, PartialEq)]
enum Item {
    Weapon { cost: i64, damage: i64 },
    Armor { cost: i64, armor: i64 },
    Ring { cost: i64, damage: i64, armor: i64 },
}

impl Item {
    fn cost(&self) -> i64 {
        match self {
            Item::Weapon { cost, .. } | Item::Armor { cost, .. } | Item::Ring { cost, .. } => *cost,
        }
    }
    fn damage(&self) -> i64 {
        match self {
            Item::Armor { .. } => 0,
            Item::Weapon { damage, .. } | Item::Ring { damage, .. } => *damage,
        }
    }
    fn armor(&self) -> i64 {
        match self {
            Item::Weapon { .. } => 0,
            Item::Armor { armor, .. } | Item::Ring { armor, .. } => *armor,
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item::Weapon { cost: 8, damage: 4 },
    Item::Weapon { cost: 10, damage: 5 },
    Item::Weapon { cost: 25, damage: 6 },
    Item::Weapon { cost: 40, damage: 7 },
    Item::Weapon { cost: 74, damage: 8 },
];

const ARMOR: [Item; 6] = [
    Item::Armor { cost: 13, armor: 1 },
    Item::Armor { cost: 31, armor: 2 },
    Item::Armor { cost: 53, armor: 3 },
    Item::Armor { cost: 75, armor: 4 },
    Item::Armor { cost: 102, armor: 5 },
    Item::Armor { cost: 0, armor: 0 },
];

const RINGS: [Item; 7] = [
    Item::Ring {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item::Ring {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item::Ring {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item::Ring {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item::Ring {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item::Ring {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item::Ring {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

fn part1(input: &Input) -> i64 {
    // Choose one item from the weapons list, one item from the armor list, and two items from the rings list.
    // (If you choose two rings, you must choose two different ones.)
    let mut min_cost = i64::MAX;
    for weapon in WEAPONS {
        for armor in ARMOR {
            for ring1 in RINGS {
                for ring2 in RINGS {
                    if ring1 == ring2 && ring1.cost() > 0 {
                        continue;
                    }
                    let cost = weapon.cost() + armor.cost() + ring1.cost() + ring2.cost();
                    if cost >= min_cost {
                        continue;
                    }
                    let player = Stats {
                        hp: 100,
                        damage: weapon.damage() + ring1.damage() + ring2.damage(),
                        armor: armor.armor() + ring1.armor() + ring2.armor(),
                    };
                    if player.would_beat(&input.opponent) {
                        min_cost = cost;
                    }
                }
            }
        }
    }
    min_cost
}

fn part2(input: &Input) -> i64 {
    let mut max_cost = i64::MIN;
    for weapon in WEAPONS {
        for armor in ARMOR {
            for ring1 in RINGS {
                for ring2 in RINGS {
                    if ring1 == ring2 && ring1.cost() > 0 {
                        continue;
                    }
                    let cost = weapon.cost() + armor.cost() + ring1.cost() + ring2.cost();
                    if cost <= max_cost {
                        continue;
                    }
                    let player = Stats {
                        hp: 100,
                        damage: weapon.damage() + ring1.damage() + ring2.damage(),
                        armor: armor.armor() + ring1.armor() + ring2.armor(),
                    };
                    if !player.would_beat(&input.opponent) {
                        max_cost = cost;
                    }
                }
            }
        }
    }
    max_cost
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        Hit Points: 12
        Damage: 7
        Armor: 2
    "};

    #[test]
    fn battle() {
        let you = Stats {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let opponent = Stats {
            hp: 12,
            damage: 7,
            armor: 2,
        };
        assert!(you.would_beat(&opponent));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 8);
    }
}
