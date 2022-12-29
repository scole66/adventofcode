//! # Solution for Advent of Code 2015 Day 10:
//!
//! Ref: [Advent of Code 2015 Day 10](https://adventofcode.com/2015/day/10)
//!

use anyhow::{anyhow, Error, Result};
use std::fmt::Display;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone)]
struct Digits {
    digits: Vec<u8>,
}

impl FromStr for Digits {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Digits {
            digits: s
                .trim()
                .chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .map(|val| val as u8)
                        .ok_or_else(|| anyhow!("Not a digit: {ch}"))
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in self.digits.iter().map(|d| (*d + b'0') as char) {
            write!(f, "{ch}")?;
        }
        Ok(())
    }
}

impl Digits {
    fn look_and_say(&self) -> Digits {
        let mut result = vec![];
        let mut run_length = 0;
        let mut run_value = None;
        for digit in self.digits.iter().copied() {
            if let Some(val) = run_value {
                if val == digit {
                    run_length += 1;
                } else {
                    result.push(run_length);
                    result.push(val);
                    run_length = 1;
                    run_value = Some(digit);
                }
            } else {
                run_length = 1;
                run_value = Some(digit);
            }
        }
        if let Some(value) = run_value {
            result.push(run_length);
            result.push(value);
        }
        Digits { digits: result }
    }

    fn len(&self) -> usize {
        self.digits.len()
    }

    fn len_after_iterations(&self, num: usize) -> usize {
        let mut digits = self.clone();
        for _ in 0..num {
            digits = digits.look_and_say();
        }
        digits.len()
    }
}

fn part1(input: &str) -> Result<usize> {
    let digits = input.parse::<Digits>()?;
    Ok(digits.len_after_iterations(40))
}

fn part2(input: &str) -> Result<usize> {
    // This is probably not the desired method, but my laptop is big enough to hold it, and fast enough to
    // figure out the answer. Computers in 2015 were not as powerful as in 2022.
    let digits = input.parse::<Digits>()?;
    Ok(digits.len_after_iterations(50))
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
    use test_case::test_case;

    #[test_case("1" => "11")]
    #[test_case("11" => "21")]
    #[test_case("21" => "1211")]
    #[test_case("1211" => "111221")]
    #[test_case("11121" => "311211")]
    fn look_and_say(inp: &str) -> String {
        let before = inp.parse::<Digits>().unwrap();
        let after = before.look_and_say();
        format!("{after}")
    }
}
