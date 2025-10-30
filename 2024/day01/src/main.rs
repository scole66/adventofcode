//! # Solution for Advent of Code 2024 Day 1: Historian Hysteria
//!
//! Ref: [Advent of Code 2024 Day 1](https://adventofcode.com/2024/day/1)
//!
use anyhow::{anyhow, Context, Error, Result};
use counter::Counter;
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    data: Vec<(usize, usize)>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let context = format!("Failed to parse \"{line}\"");
                let left = iter
                    .next()
                    .ok_or_else(|| anyhow!("expected left number"))
                    .context(context.clone())?
                    .parse::<usize>()
                    .context("Left Number wasn't numberlike")
                    .context(context.clone())?;
                let right = iter
                    .next()
                    .ok_or_else(|| anyhow!("expected right number"))
                    .context(context.clone())?
                    .parse::<usize>()
                    .context("Right Number wasn't numberlike")
                    .context(context.clone())?;
                if iter.next().is_some() {
                    Err(anyhow!("unexpected extra input").context(context))?;
                }
                Ok((left, right))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Input { data })
    }
}

fn part1(input: &Input) -> usize {
    let mut left = input.data.iter().map(|(left, _)| *left).collect::<Vec<_>>();
    let mut right = input.data.iter().map(|(_, right)| *right).collect::<Vec<_>>();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum::<usize>()
}

fn part2(input: &Input) -> usize {
    let counted = input.data.iter().map(|(_, right)| *right).collect::<Counter<_>>();
    input.data.iter().map(|(left, _)| counted[left] * left).sum::<usize>()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = Input::from_str(&input)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn part1_input_parse() {
        let input = Input::from_str(SAMPLE).unwrap();
        assert_eq!(input.data.len(), 6);
        assert_eq!(input.data[0], (3, 4));
        assert_eq!(input.data[5], (3, 3));
    }

    #[test]
    fn part1_sample() {
        let input = Input::from_str(SAMPLE).unwrap();
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn part2_sample() {
        let input = Input::from_str(SAMPLE).unwrap();
        assert_eq!(part2(&input), 31);
    }
}
