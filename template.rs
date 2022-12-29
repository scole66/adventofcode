//! # Solution for Advent of Code 2022 Day XXX:
//!
//! Ref: [Advent of Code 2022 Day XXX](https://adventofcode.com/2022/day/XXX)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

fn part1(input: &str) -> Result<usize> {
    todo!()
}

fn part2(input: &str) -> Result<usize> {
    todo!()
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
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 13);
    }

    #[test]
    #[should_panic]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 36);
    }
}
