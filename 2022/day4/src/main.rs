//! # Solution for Advent of Code 2022 Day 4: Camp Cleanup
//!
//! Ref: [Advent of Code 2022 Day 4](https://adventofcode.com/2022/day/4)
//!

use anyhow::{self, Context};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};
use std::ops::Range;

struct RangePair {
    left: Range<u32>,
    right: Range<u32>,
}

impl TryFrom<&str> for RangePair {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<left_start>[1-9][0-9]*)-(?P<left_end>[1-9][0-9]*),(?P<right_start>[1-9][0-9]*)-(?P<right_end>[1-9][0-9]*)$").expect("Hand-rolled regex is valid")
        });
        let captures = PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Bad format for range pair"))?;
        let mut citer = ["left_start", "left_end", "right_start", "right_end"]
            .iter()
            .map(|&name| {
                captures
                    .name(name)
                    .expect("Regex guarantees a match")
                    .as_str()
                    .parse::<u32>()
                    .map_err(anyhow::Error::from)
            });
        let mut next_capture = || citer.next().expect("Regex guarantees 4 matches");
        let left_start = next_capture()?;
        let left_end = next_capture()? + 1;
        let right_start = next_capture()?;
        let right_end = next_capture()? + 1;

        Ok(RangePair {
            left: Range {
                start: left_start,
                end: left_end,
            },
            right: Range {
                start: right_start,
                end: right_end,
            },
        })
    }
}

fn contained_within<T>(left: &Range<T>, right: &Range<T>) -> bool
where
    T: PartialOrd,
{
    // Returns true if left is contained (completely) within right
    left.start < left.end && left.start >= right.start && left.end <= right.end
}

fn overlap<T>(left: &Range<T>, right: &Range<T>) -> bool
where
    T: PartialOrd,
{
    // Returns true if the ranges overlap
    left.start < left.end
        && right.start < right.end
        && (right.start >= left.start && right.start < left.end || left.start >= right.start && left.start < right.end)
}

fn part1(input: &[String]) -> Result<u32, anyhow::Error> {
    let mut count = 0;
    for line in input.iter() {
        let RangePair { left, right } = RangePair::try_from(line.as_str())?;
        if contained_within(&left, &right) || contained_within(&right, &left) {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(input: &[String]) -> Result<u32, anyhow::Error> {
    let mut count = 0;
    for line in input.iter() {
        let RangePair { left, right } = RangePair::try_from(line.as_str())?;
        if overlap(&left, &right) {
            count += 1;
        }
    }
    Ok(count)
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(|line_result| line_result.map_err(anyhow::Error::from))
        .collect::<Result<Vec<String>, anyhow::Error>>()
        .context("Failed to parse puzzle input from stdin")?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn sample_part1() {
        let input = SAMPLE.lines().map(String::from).collect::<Vec<_>>();

        let p1 = part1(&input).unwrap();
        assert_eq!(p1, 2);
    }

    #[test]
    fn sample_part2() {
        let input = SAMPLE.lines().map(String::from).collect::<Vec<_>>();

        let p2 = part2(&input).unwrap();
        assert_eq!(p2, 4);
    }
}
