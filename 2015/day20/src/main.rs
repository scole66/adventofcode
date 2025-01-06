//! # Solution for Advent of Code 2015 Day 20:
//!
//! Ref: [Advent of Code 2015 Day 20](https://adventofcode.com/2015/day/20)
//!
use anyhow::{Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    number: i64,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Input> {
        let number = s.trim().parse::<i64>()?;
        Ok(Input { number })
    }
}

fn sigma(n: i64) -> i64 {
    let mut result = 0;
    let limit = (n as f64).sqrt().ceil() as i64;
    for i in 1..=limit {
        if n % i == 0 {
            result += i;
            if i * i != n {
                result += n / i;
            }
        }
    }
    result
}

fn altered_sigma(n: i64) -> i64 {
    let mut result = 0;
    let limit = (n as f64).sqrt().ceil() as i64;
    for i in 1..=limit {
        if n % i == 0 {
            if n / i <= 50 {
                result += i;
            }
            if i <= 50 {
                result += n / i;
            }
        }
    }
    result
}

fn part1(input: &Input) -> i64 {
    for house_number in 1.. {
        let sigma = sigma(house_number);
        if sigma * 10 >= input.number {
            return house_number;
        }
    }
    unreachable!()
}

fn part2(input: &Input) -> i64 {
    for house_number in 1.. {
        let sigma = altered_sigma(house_number);
        if sigma * 11 >= input.number {
            return house_number;
        }
    }
    unreachable!()
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
        70
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 4);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 4);
    }
}
