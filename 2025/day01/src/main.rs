//! # Solution for Advent of Code 2025 Day 1:
//!
//! Ref: [Advent of Code 2025 Day 1](https://adventofcode.com/2025/day/1)
//!
use anyhow::{Error, Result, anyhow, bail};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    instructions: Vec<i64>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Input {
            instructions: s
                .lines()
                .map(|line| {
                    let (direction_str, amount_str) = line
                        .split_at_checked(1)
                        .ok_or(anyhow!("Invalid instruction: {line}"))?;
                    let amount = amount_str.parse::<i64>()?;
                    match direction_str {
                        "L" => Ok(-amount),
                        "R" => Ok(amount),
                        _ => bail!("Invalid direction: {direction_str}"),
                    }
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

fn part1(input: &Input) -> i64 {
    let mut position = 50;
    let mut zeros = 0;
    for &inst in &input.instructions {
        position = (position + inst).rem_euclid(100);
        if position == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn part2(input: &Input) -> i64 {
    let mut position = 50;
    let mut zeros = 0;
    for &inst in &input.instructions {
        let complete_cycles = (inst / 100).abs();
        zeros += complete_cycles;
        let new_position = position + inst % 100;
        if position != 0 && !(0..=100).contains(&new_position) {
            zeros += 1;
        }
        position = new_position.rem_euclid(100);
        if position == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = part2(&input);
    let elapsed = start_time.elapsed();

    println!("Part1: {part1}");
    println!("Part2: {part2}");
    println!("Time: {elapsed:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 3);
    }

    #[test_case(SAMPLE => 6; "Sample input")]
    #[test_case("R1000" => 10; "Right 1000")]
    #[test_case("L51" => 1; "Left 51")]
    #[test_case("R51" => 1; "Right 51")]
    #[test_case("L151" => 2; "Left 151")]
    #[test_case("L50" => 1; "Left 50")]
    #[test_case("R50" => 1; "Right 50")]
    #[test_case("L51\nR1" => 2; "Left 51 then Right 1")]
    #[test_case("R51\nL1" => 2; "Right 51 then Left 1")]
    fn part2(instr: &str) -> i64 {
        super::part2(&instr.parse::<Input>().unwrap())
    }
}
