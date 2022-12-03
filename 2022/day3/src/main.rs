//! # Solution for Advent of Code 2022 Day 3: Rucksack Reorganization
//!
//! Ref: [Advent of Code 2022 Day 3](https://adventofcode.com/2022/day/3)
//!

use ahash::AHashSet;
use anyhow::{self, Context};
use std::io::{self, BufRead};

fn validate(line: &str) -> anyhow::Result<()> {
    (line.len() & 1 == 0)
        .then_some(())
        .ok_or_else(|| anyhow::anyhow!("Rucksack entry had an odd length"))?;
    line.chars()
        .all(|ch| ch.is_ascii_alphabetic())
        .then_some(())
        .ok_or_else(|| anyhow::anyhow!("Rucksack entry had illegal chars"))
}

fn priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        'A'..='Z' => item as i32 - 'A' as i32 + 27,
        _ => unreachable!(),
    }
}

fn common_item(rucksack: &str) -> anyhow::Result<char> {
    let size = rucksack.len();
    let left_compartment = AHashSet::from_iter(rucksack[0..(size / 2)].chars());
    let right_compartment = AHashSet::from_iter(rucksack[(size / 2)..size].chars());

    let items_in_common = left_compartment.intersection(&right_compartment).collect::<Vec<_>>();

    (items_in_common.len() == 1)
        .then(|| *items_in_common[0])
        .ok_or_else(|| anyhow::anyhow!("Expected one item in common, found {}", items_in_common.len()))
}

fn part1(rucksacks: &[String]) -> anyhow::Result<i32> {
    let priorities = rucksacks
        .iter()
        .map(|rucksack| {
            let item = common_item(rucksack.as_str())?;
            Ok(priority(item))
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;
    Ok(priorities.iter().sum())
}

fn partition(rucksacks: &[String]) -> anyhow::Result<Vec<[String; 3]>> {
    (rucksacks.len() % 3 == 0).then_some(()).ok_or_else(|| {
        anyhow::anyhow!(
            "The number of rucksacks must be divisible by 3. (Found {}.)",
            rucksacks.len()
        )
    })?;
    let mut results = Vec::with_capacity(rucksacks.len() / 3);
    let mut group = Vec::with_capacity(3);
    for sack in rucksacks {
        group.push(sack);
        if group.len() == 3 {
            results.push([group[0].clone(), group[1].clone(), group[2].clone()]);
            group.clear();
        }
    }

    Ok(results)
}

fn common_group_item(group: &[String; 3]) -> anyhow::Result<char> {
    let group1_items = AHashSet::from_iter(group[0].chars());
    let group2_items = AHashSet::from_iter(group[1].chars());
    let group3_items = AHashSet::from_iter(group[2].chars());

    let early_commons = group1_items
        .intersection(&group2_items)
        .copied()
        .collect::<AHashSet<_>>();
    let common = early_commons.intersection(&group3_items).collect::<Vec<_>>();

    (common.len() == 1)
        .then(|| *common[0])
        .ok_or_else(|| anyhow::anyhow!("Expected one common item, found {}", common.len()))
}

fn part2(rucksacks: &[String]) -> anyhow::Result<i32> {
    let groups = partition(rucksacks)?;
    let priorities = groups
        .iter()
        .map(|group| {
            let item = common_group_item(group)?;
            Ok(priority(item))
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;
    Ok(priorities.iter().sum())
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let rucksacks = stdin
        .lock()
        .lines()
        .map(|result| {
            result.map_err(anyhow::Error::from).and_then(|line| {
                validate(&line)?;
                Ok(line)
            })
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()
        .context("Failed to parse puzzle input from stdin")?;

    println!("Part1: {}", part1(&rucksacks)?);
    println!("Part2: {}", part2(&rucksacks)?);

    Ok(())
}

#[cfg(test)]

static SAMPLE: &str = indoc::indoc! {"
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
"};

#[test]
fn part1_sample() {
    let rucksacks = SAMPLE.lines().map(String::from).collect::<Vec<_>>();

    let p1 = part1(&rucksacks).unwrap();
    assert_eq!(p1, 157);
}

#[test]
fn part2_sample() {
    let rucksacks = SAMPLE.lines().map(String::from).collect::<Vec<_>>();

    let p2 = part2(&rucksacks).unwrap();
    assert_eq!(p2, 70);
}
