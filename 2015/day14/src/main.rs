//! # Solution for Advent of Code 2015 Day 14:
//!
//! Ref: [Advent of Code 2015 Day 14](https://adventofcode.com/2015/day/14)
//!
use ahash::AHashMap;
use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

struct Datum {
    flight_time: u64,
    fly_speed: u64,
    rest_time: u64,
    name: String,
}

impl FromStr for Datum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<name>[a-zA-Z]+) can fly (?P<fs>0|[1-9][0-9]*) km/s for (?P<ft>0|[1-9][0-9]*) seconds?, but then must rest for (?P<rt>0|[1-9][0-9]*) seconds?\.$").unwrap()
        });

        let caps = PATTERN.captures(s).ok_or_else(|| anyhow!("Bad input format: {s}"))?;
        let flight_time = caps["ft"].parse::<u64>()?;
        let fly_speed = caps["fs"].parse::<u64>()?;
        let rest_time = caps["rt"].parse::<u64>()?;
        let name = caps["name"].to_string();

        Ok(Datum {
            flight_time,
            fly_speed,
            rest_time,
            name,
        })
    }
}

impl Datum {
    fn distance_after(&self, seconds: u64) -> u64 {
        let period = self.flight_time + self.rest_time;
        let complete_cycle_part = seconds / period * self.fly_speed * self.flight_time;
        let remainder = self.fly_speed * self.flight_time.min(seconds % period);
        complete_cycle_part + remainder
    }

    // The speed of this reindeer during a particular second (0-based indexing)
    fn speed_at(&self, second: u64) -> u64 {
        let period = self.flight_time + self.rest_time;
        let position_within_cycle = second % period;
        if position_within_cycle < self.flight_time {
            self.fly_speed
        } else {
            0
        }
    }

    fn locations_by_time(&self, max_secs: u64) -> Vec<u64> {
        (0..max_secs)
            .map(|time| self.speed_at(time))
            .scan(0, |loc, speed| {
                let new_loc = *loc + speed;
                *loc = new_loc;
                Some(new_loc)
            })
            .collect()
    }
}

struct Data {
    reindeer: AHashMap<String, Datum>,
}

impl FromStr for Data {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.parse::<Datum>().map(|d| (d.name.clone(), d)))
            .collect::<Result<AHashMap<_, _>>>()?;
        Ok(Data { reindeer: map })
    }
}

impl Data {
    fn race(&self, seconds: u64) -> u64 {
        let mut points = AHashMap::new();
        let mut distance = AHashMap::new();
        for (_, d) in self.reindeer.iter() {
            distance.insert(d.name.clone(), d.locations_by_time(seconds));
        }
        for ts in 0..seconds as usize {
            let furthest_distance = distance.values().map(|locs| locs[ts]).max().expect("should be data");
            for (name, locs) in distance.iter() {
                if locs[ts] == furthest_distance {
                    points
                        .entry(name.clone())
                        .and_modify(|score| {
                            *score += 1;
                        })
                        .or_insert(1);
                }
            }
        }
        points.values().copied().max().expect("should be data")
    }
}

fn part1(input: &str) -> Result<u64> {
    let Data { reindeer: herd } = input.parse::<Data>()?;
    herd.values()
        .map(|datum| datum.distance_after(2503))
        .max()
        .ok_or_else(|| anyhow!("Empty input"))
}

fn part2(input: &str) -> Result<u64> {
    let herd = input.parse::<Data>()?;
    Ok(herd.race(2503))
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
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    "};

    #[test]
    fn part1_sample() {
        let Data { reindeer: herd } = SAMPLE.parse::<Data>().unwrap();

        assert_eq!(herd["Dancer"].distance_after(1000), 1056);
        assert_eq!(herd["Comet"].distance_after(1000), 1120);
    }

    #[test]
    fn part2_sample() {
        let herd = SAMPLE.parse::<Data>().unwrap();
        assert_eq!(herd.race(1000), 689);
    }
}
