//! # Solution for Advent of Code 2025 Day 5: Cafeteria
//!
//! Ref: [Advent of Code 2025 Day 5](https://adventofcode.com/2025/day/5)
//!
use ahash::{HashSet, HashSetExt};
use anyhow::{Error, Result, anyhow};
use std::cmp::{max, min};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    fresh: Vec<(i64, i64)>,
    inventory: Vec<i64>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (fresh_str, inventory_str) = s.split_once("\n\n").ok_or(anyhow!("Input is missing a blank line"))?;
        let fresh = fresh_str
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').ok_or(anyhow!(format!("Bad Range: {line}")))?;
                let start = start.parse::<i64>()?;
                let end = end.parse::<i64>()?;
                Ok((start, end))
            })
            .collect::<Result<Vec<_>>>()?;
        let inventory = inventory_str
            .lines()
            .map(|line| {
                let id = line.parse::<i64>()?;
                Ok(id)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Input { fresh, inventory })
    }
}

impl Input {
    fn is_fresh(&self, id: i64) -> bool {
        self.fresh.iter().any(|range| range.0 <= id && id <= range.1)
    }

    fn fresh_ingredients(&self) -> impl Iterator<Item = i64> {
        self.inventory.iter().filter(|id| self.is_fresh(**id)).copied()
    }
}

fn merge_unchecked(left: &(i64, i64), right: &(i64, i64)) -> (i64, i64) {
    // assumes left & right overlap or are adjacent
    (min(left.0, right.0), max(left.1, right.1))
}

fn ranges_joinable(left: &(i64, i64), right: &(i64, i64)) -> bool {
    let (first_end, second_start) = if left.0 < right.0 {
        (left.1, right.0)
    } else {
        (right.1, left.0)
    };
    // window between is 0 (or less, indicating overlap)
    second_start - first_end <= 1
}

fn normalize_ranges(range_list: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut result = HashSet::new();

    for r in range_list {
        let overlaps = result
            .iter()
            .filter(|existing| ranges_joinable(r, existing))
            .copied()
            .collect::<Vec<_>>();
        let mut merged = *r;
        for pair in overlaps {
            merged = merge_unchecked(&merged, &pair);
            result.remove(&pair);
        }
        result.insert(merged);
    }

    result.into_iter().collect()
}

fn part1(input: &Input) -> usize {
    input.fresh_ingredients().count()
}

fn part2(input: &Input) -> i64 {
    let normalized = normalize_ranges(&input.fresh);
    normalized.iter().map(|r| r.1 - r.0 + 1).sum()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = part2(&input);
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
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 3);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 14);
    }
}
