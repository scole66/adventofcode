//! # Solution for Advent of Code 2023 Day 6: Wait For It
//!
//! Ref: [Advent of Code 2023 Day 6](https://adventofcode.com/2023/day/6)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::io::{self, Read};
use std::str::FromStr;

// dist travelled = hold_time (ms) * (1 mm/ms / ms) * (race_time (ms) - hold_time(ms)) = H*R - H^2

#[derive(Debug)]
struct Input {
    times: Vec<i64>,
    distances: Vec<i64>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let line_of_times = lines
            .next()
            .ok_or_else(|| anyhow!("Line of times missing from input"))?;
        let mut times_words = line_of_times.split_whitespace();
        let time_head = times_words.next().ok_or_else(|| anyhow!("Line of times is empty"))?;
        if time_head != "Time:" {
            bail!("Line of times missing correct identifer; found {time_head}");
        }
        let times = times_words
            .map(|word| word.parse::<i64>().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        let line_of_results = lines
            .next()
            .ok_or_else(|| anyhow!("Line of distances missing from input"))?;
        let mut results_words = line_of_results.split_whitespace();
        let result_head = results_words
            .next()
            .ok_or_else(|| anyhow!("Line of distances is empty"))?;
        if result_head != "Distance:" {
            bail!("Line of distances missing correct identifier; found {result_head}")
        }
        let distances = results_words
            .map(|word| word.parse::<i64>().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        if times.len() != distances.len() {
            bail!(
                "Number of times values {} must match number of distance values {}",
                times.len(),
                distances.len()
            );
        }
        Ok(Input { times, distances })
    }
}

fn ways_to_beat_record(race_time: i64, current_record: i64) -> i64 {
    // We want all the H that cause H*R - H^2 to be greater than D
    // Or, -h^2 + rh - d > 0
    // that's a downward facing parabola; the zeros are the min & max of our range for H.
    //   (-b +/- sqrt(b^2 -4ac)) / 2a

    let descriminant = race_time * race_time - 4 * current_record;
    assert!(descriminant > 0);
    let desc_root = (descriminant as f64).sqrt();
    let s1 = (-(race_time as f64) - desc_root) * (-0.5);
    let s2 = (-(race_time as f64) + desc_root) * (-0.5);
    let lower = ((s1.min(s2) + 1.0).floor()) as i64;
    let higher = ((s1.max(s2) - 1.0).ceil()) as i64;

    higher - lower + 1
}

fn part1(input: &Input) -> i64 {
    input
        .times
        .iter()
        .zip(input.distances.iter())
        .map(|(time, distance)| ways_to_beat_record(*time, *distance))
        .product()
}

fn part2(input: &Input) -> i64 {
    let time = input.times.iter().join("").parse::<i64>().unwrap();
    let distance = input.distances.iter().join("").parse::<i64>().unwrap();
    ways_to_beat_record(time, distance)
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
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test_case(7, 9 => 4)]
    #[test_case(15, 40 => 8)]
    #[test_case(30, 200 => 9)]
    fn ways_to_beat_record(time: i64, record: i64) -> i64 {
        super::ways_to_beat_record(time, record)
    }

    #[test]
    fn part1_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn part2_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part2(&input), 71503);
    }
}
