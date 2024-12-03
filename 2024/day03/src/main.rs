//! # Solution for Advent of Code 2024 Day 3: Mull It Over
//!
//! Ref: [Advent of Code 2024 Day 3](https://adventofcode.com/2024/day/3)
//!
use anyhow::{bail, Context, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    s: String,
}
impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Input { s: s.to_string() })
    }
}

fn part1(input: &Input) -> Result<usize> {
    static PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").expect("compiled patterns shouldn't fail"));
    PATTERN
        .captures_iter(&input.s)
        .map(|cap| {
            let a = cap[1]
                .parse::<usize>()
                .context(format!("failed parsing first multiplicand ({})", &cap[1]))?;
            let b = cap[2]
                .parse::<usize>()
                .context(format!("failed parsing second multiplicand ({})", &cap[2]))?;
            Ok(a * b)
        })
        .sum::<Result<usize>>()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    Do,
    Dont,
    Mul(usize, usize),
}

fn part2(input: &Input) -> Result<usize> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+),(\d+)\))").expect("compiled patterns shouldn't fail")
    });
    let program = PATTERN
        .captures_iter(&input.s)
        .map(|cap| {
            if cap.get(1).is_some() {
                Ok(Op::Do)
            } else if cap.get(2).is_some() {
                Ok(Op::Dont)
            } else if cap.get(3).is_some() {
                let a = cap[4]
                    .parse::<usize>()
                    .context(format!("failed parsing first multiplicand ({})", &cap[4]))?;
                let b = cap[5]
                    .parse::<usize>()
                    .context(format!("failed parsing second multiplicand ({})", &cap[5]))?;
                Ok(Op::Mul(a, b))
            } else {
                bail!("failed to parse input")
            }
        })
        .collect::<Result<Vec<Op>>>()?;
    let mut sum = 0;
    let mut multiplication_allowed = true;
    for op in program {
        match op {
            Op::Do => multiplication_allowed = true,
            Op::Dont => multiplication_allowed = false,
            Op::Mul(a, b) => {
                if multiplication_allowed {
                    sum += a * b;
                }
            }
        }
    }

    Ok(sum)
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()).unwrap(), 161);
    }

    static SAMPLE2: &str = indoc::indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE2.parse::<Input>().unwrap()).unwrap(), 48);
    }
}
