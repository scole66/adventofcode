//! # Solution for Advent of Code 2015 Day 16: Aunt Sue
//!
//! Ref: [Advent of Code 2015 Day 16](https://adventofcode.com/2015/day/16)
//!
use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

struct MatchInput {
    children: i32,
    cats: i32,
    samoyeds: i32,
    pomeranians: i32,
    akitas: i32,
    vizslas: i32,
    goldfish: i32,
    trees: i32,
    cars: i32,
    perfumes: i32,
}

const TICKER_TAPE: MatchInput = MatchInput {
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1,
};

#[derive(Debug, Default)]
struct Sue {
    id: i32,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}
impl FromStr for Sue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static SUE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^Sue (?P<id>0|[1-9][0-9]*)$").unwrap());
        static ITEM_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^(?P<item>children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): (?P<amount>0|[1-9][0-9]*)$").unwrap()
        });
        let (label, data) = s.split_once(": ").ok_or_else(|| anyhow!("Mangled input: {s}"))?;
        let caps = SUE_PATTERN
            .captures(label)
            .ok_or_else(|| anyhow!("Bad label: \"{label}\""))?;
        let id = caps["id"].parse::<i32>()?;
        let mut sue = Sue { id, ..Default::default() };
        for item in data.split(", ") {
            let caps = ITEM_PATTERN
                .captures(item)
                .ok_or_else(|| anyhow!("Bad item: \"{item}\""))?;
            let amount = Some(caps["amount"].parse::<i32>()?);
            match &caps["item"] {
                "children" => sue.children = amount,
                "cats" => sue.cats = amount,
                "samoyeds" => sue.samoyeds = amount,
                "pomeranians" => sue.pomeranians = amount,
                "akitas" => sue.akitas = amount,
                "vizslas" => sue.vizslas = amount,
                "goldfish" => sue.goldfish = amount,
                "trees" => sue.trees = amount,
                "cars" => sue.cars = amount,
                "perfumes" => sue.perfumes = amount,
                _ => unreachable!(),
            };
        }
        Ok(sue)
    }
}

struct Family {
    aunts: Vec<Sue>,
}

impl FromStr for Family {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Family { aunts: s.lines().map(|line| line.parse::<Sue>()).collect::<Result<Vec<_>>>()? })
    }
}

impl Family {
    fn search(&self, needle: &MatchInput) -> Vec<i32> {
        self.aunts
            .iter()
            .filter(|&sue| {
                sue.children.map_or(true, |amt| amt == needle.children)
                    && sue.cats.map_or(true, |amt| amt == needle.cats)
                    && sue.samoyeds.map_or(true, |amt| amt == needle.samoyeds)
                    && sue.pomeranians.map_or(true, |amt| amt == needle.pomeranians)
                    && sue.akitas.map_or(true, |amt| amt == needle.akitas)
                    && sue.vizslas.map_or(true, |amt| amt == needle.vizslas)
                    && sue.goldfish.map_or(true, |amt| amt == needle.goldfish)
                    && sue.trees.map_or(true, |amt| amt == needle.trees)
                    && sue.cars.map_or(true, |amt| amt == needle.cars)
                    && sue.perfumes.map_or(true, |amt| amt == needle.perfumes)
            })
            .map(|sue| sue.id)
            .collect::<Vec<_>>()
    }

    fn best_match(&self, needle: &MatchInput) -> i32 {
        let matches = self.search(needle);
        matches[0]
    }

    fn search_ranges(&self, needle: &MatchInput) -> Vec<i32> {
        self.aunts
            .iter()
            .filter(|&sue| {
                sue.children.map_or(true, |amt| amt == needle.children)
                    && sue.cats.map_or(true, |amt| amt > needle.cats)
                    && sue.samoyeds.map_or(true, |amt| amt == needle.samoyeds)
                    && sue.pomeranians.map_or(true, |amt| amt < needle.pomeranians)
                    && sue.akitas.map_or(true, |amt| amt == needle.akitas)
                    && sue.vizslas.map_or(true, |amt| amt == needle.vizslas)
                    && sue.goldfish.map_or(true, |amt| amt < needle.goldfish)
                    && sue.trees.map_or(true, |amt| amt > needle.trees)
                    && sue.cars.map_or(true, |amt| amt == needle.cars)
                    && sue.perfumes.map_or(true, |amt| amt == needle.perfumes)
            })
            .map(|sue| sue.id)
            .collect::<Vec<_>>()
    }
}

fn part1(input: &str) -> Result<i32> {
    let family = input.parse::<Family>()?;
    let id = family.best_match(&TICKER_TAPE);

    Ok(id)
}

fn part2(input: &str) -> Result<i32> {
    let family = input.parse::<Family>()?;
    Ok(family.search_ranges(&TICKER_TAPE)[0])
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}
