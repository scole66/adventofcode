//! # Solution for Advent of Code 2023 Day 9: Mirage Maintenance
//!
//! Ref: [Advent of Code 2023 Day 9](https://adventofcode.com/2023/day/9)
//!
use anyhow::{Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Sequence(Vec<i64>);
impl FromStr for Sequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_whitespace()
                .map(|numstr| numstr.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Sequence {
    fn deltas(&self) -> Vec<Vec<i64>> {
        let mut deltas = Vec::new();
        let mut work_vector = self.0.clone();
        while work_vector.iter().any(|&x| x != 0) {
            let new_work_vector = work_vector
                .windows(2)
                .map(|slice| slice[1] - slice[0])
                .collect::<Vec<_>>();
            deltas.push(work_vector);
            work_vector = new_work_vector;
        }

        deltas
    }

    fn extrapolate(&self) -> i64 {
        self.deltas().iter().map(|v| v.last().unwrap()).sum::<i64>()
    }

    fn pre_extrapolate(&self) -> i64 {
        // Math.
        // If deltas is indexed starting at zero,
        // value = sum(0<=n<inf; (-1)^n * D[n,0])
        self.deltas()
            .iter()
            .map(|v| *v.first().unwrap())
            .fold((1, 0), |(multiplier, acc), val| (-multiplier, acc + multiplier * val))
            .1
    }
}
struct Input(Vec<Sequence>);
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse::<Sequence>())
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

fn part1(input: &Input) -> i64 {
    input.0.iter().map(|seq| seq.extrapolate()).sum::<i64>()
}

fn part2(input: &Input) -> i64 {
    input.0.iter().map(|seq| seq.pre_extrapolate()).sum::<i64>()
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
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part1_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn part2_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part2(&input), 2);
    }
}
