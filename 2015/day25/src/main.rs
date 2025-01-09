//! # Solution for Advent of Code 2015 Day 25:
//!
//! Ref: [Advent of Code 2015 Day 25](https://adventofcode.com/2015/day/25)

use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    row: i64,
    column: i64,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^To continue, please consult the code grid in the manual\.  Enter the code at row (?<row>[0-9]+), column (?<column>[0-9]+)\.\s*$").unwrap()
        });
        let caps = PATTERN.captures(s).ok_or_else(|| anyhow!("Bad input line"))?;
        let row = caps["row"].parse::<i64>()?;
        let column = caps["column"].parse::<i64>()?;
        Ok(Input { row, column })
    }
}

fn coords_to_sequence_number(row: i64, column: i64) -> i64 {
    let y = row - 1;
    let x = column - 1;

    x + (((y + x) * (y + x + 1)) >> 1)
}

const START: i64 = 20_151_125;
const FACTOR: i64 = 252_533;
const BOUND: i64 = 33_554_393;

fn part1(row: i64, column: i64) -> i64 {
    let mut seqnum = coords_to_sequence_number(row, column);
    let mut result = START;
    while seqnum > 0 {
        result = (result * FACTOR) % BOUND;
        seqnum -= 1;
    }

    result
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(input.row, input.column);
    let elapsed = start_time.elapsed();

    println!("Part1: {part1}");
    println!("Time: {elapsed:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 1 => 0; "topleft")]
    #[test_case(2, 1 => 1; "2, 1")]
    #[test_case(1, 2 => 2; "1, 2")]
    #[test_case(3, 1 => 3; "3, 1")]
    #[test_case(2, 2 => 4; "2, 2")]
    fn coords_to_seq(row: i64, col: i64) -> i64 {
        coords_to_sequence_number(row, col)
    }

    #[test_case(1, 1 => 20_151_125; "1, 1")]
    #[test_case(2, 1 => 31_916_031; "2, 1")]
    #[test_case(2, 2 => 21_629_792; "2, 2")]
    #[test_case(6, 6 => 27_995_004; "6, 6")]
    fn part1(row: i64, col: i64) -> i64 {
        super::part1(row, col)
    }
}
