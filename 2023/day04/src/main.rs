//! # Solution for Advent of Code 2023 Day 4:
//!
//! Ref: [Advent of Code 2023 Day 4](https://adventofcode.com/2023/day/4)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    inventory: Vec<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static ROW_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^Card +(?<id>[1-9][0-9]*):(?<winners>(?: +[1-9][0-9]*)+) *\|(?<inventory>(?: +[1-9][0-9]*)+)$")
                .unwrap()
        });
        let caps = ROW_PATTERN
            .captures(s)
            .ok_or_else(|| anyhow!("Bad format for card line {s}"))?;
        let id = caps
            .name("id")
            .expect("id should always be matched")
            .as_str()
            .parse::<u32>()?;
        let mut winners = caps
            .name("winners")
            .expect("winners should always be matched")
            .as_str()
            .split_whitespace()
            .map(|ss| ss.parse::<u32>().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        let mut inventory = caps
            .name("inventory")
            .expect("inventory should always be matched")
            .as_str()
            .split_whitespace()
            .map(|ss| ss.parse::<u32>().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;

        winners.sort();
        inventory.sort();
        Ok(Card { id, winners, inventory })
    }
}

impl Card {
    fn num_matches(&self) -> usize {
        self.inventory
            .iter()
            .filter(|&probe| self.winners.contains(probe))
            .collect::<Vec<_>>()
            .len()
    }
    fn points(&self) -> usize {
        let num = self.num_matches();
        if num == 0 {
            0
        } else {
            1 << (num - 1)
        }
    }
    fn id(&self) -> u32 {
        self.id
    }
}

fn part1(input: &str) -> Result<usize> {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>())
        .collect::<Result<Vec<_>>>()?;
    Ok(cards.iter().map(Card::points).sum::<usize>())
}

fn part2(input: &str) -> Result<usize> {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>())
        .collect::<Result<Vec<_>>>()?;
    let mut collection = cards
        .into_iter()
        .map(|card| (card.id(), (1, card)))
        .collect::<AHashMap<_, _>>();
    let mut keys = collection.keys().copied().collect::<Vec<_>>();
    keys.sort();
    for k in keys {
        let (count_ref, card) = collection.get(&k).unwrap();
        let count = *count_ref;
        let winner_num = card.num_matches();
        let key = k as usize;
        if winner_num > 0 {
            for extra in key + 1..=(key + winner_num) {
                if let Some(v) = collection.get_mut(&(extra as u32)) {
                    v.0 += count;
                }
            }
        }
    }
    Ok(collection.values().map(|val| val.0).sum::<usize>())
}

fn main() -> Result<()> {
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
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 13);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 30);
    }
}
