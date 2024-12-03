//! # Solution for Advent of Code 2024 Day 2: Red-Nosed Reports
//!
//! Ref: [Advent of Code 2024 Day 2](https://adventofcode.com/2024/day/2)
//!
use anyhow::{Context, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Report {
    levels: Vec<i64>,
}

impl FromStr for Report {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .enumerate()
            .map(|(index, token)| {
                token
                    .parse::<i64>()
                    .context(format!("Bad parse for Level in column {}: {token}", index + 1))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Report { levels })
    }
}

struct Input {
    reports: Vec<Report>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .map(|(num, line)| {
                line.parse::<Report>()
                    .context(format!("Couldn't parse line {}: {line}", num + 1))
            })
            .collect::<Result<Vec<Report>>>()
            .map(|reports| Input { reports })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

impl Report {
    fn safety(&self) -> Safety {
        let deltas = self
            .levels
            .iter()
            .zip(self.levels.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();
        if (deltas.iter().all(|&delta| delta > 0) || deltas.iter().all(|&delta| delta < 0))
            && deltas.iter().all(|&delta| (1..=3).contains(&delta.abs()))
        {
            Safety::Safe
        } else {
            Safety::Unsafe
        }
    }

    fn dampened_safety(&self) -> Safety {
        if self.safety() == Safety::Safe {
            Safety::Safe
        } else {
            for removal_index in 0..self.levels.len() {
                let altered = Report {
                    levels: self
                        .levels
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(index, &level)| {
                                if index != removal_index {
                                    Some(level)
                                } else {
                                    None
                                }
                            },
                        )
                        .collect::<Vec<_>>(),
                };
                if altered.safety() == Safety::Safe {
                    return Safety::Safe;
                }
            }
            Safety::Unsafe
        }
    }
}

fn part1(input: &Input) -> usize {
    input
        .reports
        .iter()
        .filter(|report| report.safety() == Safety::Safe)
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .reports
        .iter()
        .filter(|report| report.dampened_safety() == Safety::Safe)
        .count()
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

    static SAMPLE: &str = indoc::indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn parse_input() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(input.reports.len(), 6);
        assert_eq!(input.reports[0].levels.len(), 5);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 2);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 4);
    }
}
