//! # Solution for Advent of Code 2015 Day 9: All in a Single Night
//!
//! Ref: [Advent of Code 2015 Day 9](https://adventofcode.com/2015/day/9)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashSet, AHashMap};
use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

struct DataPoint {
    location_a: String,
    location_b: String,
    distance: usize,
}

impl FromStr for DataPoint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<location_a>.*) to (?P<location_b>.*) = (?P<distance>0|[1-9][0-9]+)$")
                .expect("Hand-rolled regex is valid")
        });
        let caps = PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Bad distance specification: \"{s}\""))?;
        Ok(DataPoint {
            location_a: caps["location_a"].to_string(),
            location_b: caps["loaation_b"].to_string(),
            distance: caps["distance"]
                .parse::<usize>()
                .context("Error while parsing distance in \"{s}\"")?,
        })
    }
}

#[derive(Default)]
struct Data {
    locations: AHashSet<String>,
    distances: AHashMap<(String, String), usize>,
}

impl FromIterator<DataPoint> for Result<Data, anyhow::Error> {
    fn from_iter<T: IntoIterator<Item = DataPoint>>(iter: T) -> Self {
        // Collect up all the data
        let mut data = Data::default();
        for point in iter.into_iter() {
            data.distances.insert((point.location_a.clone(), point.location_b.clone()), point.distance);
            data.locations.insert(point.location_a);
            data.locations.insert(point.location_b);
        }
        // Do some validation
        // 1. All pairs must have a distance. (either (a,b) or (b,a))
        // 2. If both orders exist, they must have the same distance ((a,b) = (b,a))
        
        Ok(data)
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let data = input
        .lines()
        .map(|line| line.parse::<DataPoint>())
        .collect::<Result<Vec<DataPoint>, anyhow::Error>>()?;

    
}

fn part2(input: &str) -> anyhow::Result<usize> {
    todo!()
}

fn main() -> anyhow::Result<()> {
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
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 605);
    }

    #[test]
    #[should_panic]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 36);
    }
}
