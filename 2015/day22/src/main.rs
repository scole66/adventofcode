//! # Solution for Advent of Code 2015 Day 22:
//!
//! Ref: [Advent of Code 2015 Day 22](https://adventofcode.com/2015/day/22)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    hit_points: i64,
    damage: i64
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let (Some(hit_points), Some(damage)) = s
            .lines()
            .try_fold((None, None), |acc, line| {
                let (id, value) = line.split_once(": ").ok_or_else(|| anyhow!("Bad line {line}"))?;
                let value = value.parse::<i64>()?;
                if id == "Hit Points" {
                    Ok((Some(value), acc.1))
                } else if id == "Damage" {
                    Ok((acc.0, Some(value)))
                } else {
                    bail!("Bad value id {id}")
                }
            })? {
                Ok(Input{hit_points, damage})
            }
        else {
            bail!("Need both Hit Points and Damage")
        }
    }
}

struct World {
    player_hit_points: i64,
    player_armor: i64,
    player_mana: i64,
    boss_hit_points: i64,
    boss_damage: i64,
    poison_timer: i64,
    shield_timer: i64,
    recharge_timer: i64,
    loud: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
    Nothing,
}

macro_rules! diagnostic {
    ($self:ident, $($arg:tt)*) => {
        if $self.loud {
            println!($($arg)*);
        }
    };
}

impl World {
    fn new(boss: &Input, player_hit_points: i64, player_mana: i64) -> Self {
        Self {
            player_hit_points,
            player_armor: 0,
            player_mana,
            boss_hit_points: boss.hit_points,
            boss_damage: boss.damage,
            poison_timer: 0,
            shield_timer: 0,
            recharge_timer: 0,
            loud: true,
        }
    }

    fn make_loud(&mut self) {
        self.loud = true;
    }
    fn make_quiet(&mut self) {        
        self.loud = false;
    }

    fn print_combantant_status(&self) {
        diagnostic!(self, "- Player has {} hit points, {} armor, {} mana", self.player_hit_points, self.player_armor, self.player_mana);
        diagnostic!(self, "- Boss has {} hit points", self.boss_hit_points);
    }

    fn hurt_the_boss(&mut self, amt: i64) -> Option<()> {
        self.boss_hit_points -= amt;
        if self.boss_hit_points <= 0 {
            diagnostic!(self, "The boss has been killed; the player wins.");
            return None
        }
        Some(())
    }

    fn advance_effects(&mut self) -> Option<()> {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
            diagnostic!(self, "Shield's timer is now {}.", self.shield_timer);
            if self.shield_timer == 0 {
                self.player_armor -= 7;
                diagnostic!(self, "Shield wears off, decreasing armor by 7.");
            }
        }
        if self.poison_timer > 0 {
            self.poison_timer -= 1;
            diagnostic!(self, "Poison deals 3 damage; its timer is now {}.", self.poison_timer);
            if self.poison_timer == 0 {
                diagnostic!(self, "Poison wears off.")
            }
            self.hurt_the_boss(3)?;
        }
        if self.recharge_timer > 0 {
            self.player_mana += 101;
            self.recharge_timer -= 1;
            diagnostic!(self, "Recharge provides 101 mana; its timer is now {}.", self.recharge_timer);
            if self.recharge_timer == 0 {
                diagnostic!(self, "Recharge wears off.");
            }
        }

        Some(())
    }

    fn cast_spell(&mut self, spell: Spell) -> Option<()> {
        match spell {
            Spell::MagicMissile => {
                diagnostic!(self, "Player casts Magic Milssile, dealing 4 damage.");
                self.player_mana -= 53;
                self.hurt_the_boss(4)?;
            }
            Spell::Drain => {
                diagnostic!(self, "Player casts Drain, dealing 2 damage, and healing 2 hp.");
                self.player_mana -= 73;
                self.hurt_the_boss(2)?;
                self.player_hit_points += 2;
            },
            Spell::Shield => {
                diagnostic!(self, "Player casts Shield, increasing armor by 7.");
                self.player_mana -= 113;
                self.player_armor += 7;
                self.shield_timer = 6;
            },
            Spell::Poison => {
                diagnostic!(self, "Player casts Poison.");
                self.player_mana -= 173;
                self.poison_timer = 6;
            },
            Spell::Recharge => {
                diagnostic!(self, "Player casts Recharge.");
                self.player_mana -= 229;
                self.recharge_timer = 5;
            }
            Spell::Nothing => {}
        }
        Some(())
    }

    fn turn(&mut self, spell: Spell) -> Option<()> {
        diagnostic!(self, "-- Player turn --");
        self.print_combantant_status();

        self.advance_effects()?;
        self.cast_spell(spell);
        diagnostic!(self, "");

        diagnostic!(self, "-- Boss turn --");
        self.print_combantant_status();
        self.advance_effects()?;

        diagnostic!(self, "Boss attacks for {} damage!", self.boss_damage);
        self.player_hit_points -= (self.boss_damage - self.player_armor).max(1);
        if self.player_hit_points <= 0 {
            diagnostic!(self, "Player dies. Boss wins.");
            return None;
        }
        diagnostic!(self, "");

        Some(())
    }

    fn castable_spells(&self) -> Vec<Spell> {
        let mut possible = Vec::new();
        if self.player_mana >= 53 {
            possible.push(Spell::MagicMissile);
        }
        if self.player_mana >= 73 {
            possible.push(Spell::Drain);
        }
        if self.player_mana >= 113 && self.shield_timer <= 1 {
            possible.push(Spell::Drain);
        }
        if self.player_mana >= 173 && self.poison_timer <= 1 {
            possible.push(Spell::Poison);
        }
        if self.player_mana >= 229 && self.recharge_timer <= 1 {
            possible.push(Spell::Recharge);
        }
        possible
    }
}

fn part1(input: &Input) -> i64 {
    let mut w = World::new(input, 50, 500);

    
}

fn part2(input: &Input) -> i64 {
    todo!()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = 0; //part2(&input);
    let elapsed = start_time.elapsed();

    println!("Part1: {part1}");
    println!("Part2: {part2}");
    println!("Time: {elapsed:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 13);
    }

    #[test]
    #[should_panic]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 36);
    }
}
