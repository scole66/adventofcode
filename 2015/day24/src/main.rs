//! # Solution for Advent of Code 2015 Day 24: It Hangs in the Balance
//!
//! Ref: [Advent of Code 2015 Day 24](https://adventofcode.com/2015/day/24)
//!
use anyhow::{bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    numbers: Box<[i64]>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let numbers = s
            .lines()
            .map(|s| s.trim().parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        let sum = numbers.iter().sum::<i64>();
        if sum % 3 != 0 || sum % 4 != 0 {
            bail!("Numbers must add to a value divisible by both 3 and 4");
        }
        Ok(Self { numbers })
    }
}

fn find_combinations(nums: &[i64], target: i64) -> Vec<Vec<i64>> {
    fn backtrack(nums: &[i64], target: i64, start: usize, path: &mut Vec<i64>, result: &mut Vec<Vec<i64>>) {
        if target == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..nums.len() {
            if nums[i] > target {
                continue;
            }
            path.push(nums[i]);
            backtrack(nums, target - nums[i], i + 1, path, result);
            path.pop();
        }
    }
    let mut result = Vec::new();
    let mut path = Vec::new();
    backtrack(nums, target, 0, &mut path, &mut result);
    result
}

fn best_arrangement(input: &Input, bundle_count: i64) -> i64 {
    let bundle_size = input.numbers.iter().sum::<i64>() / bundle_count;
    let mut front_seat_minimum = (i64::MAX, usize::MAX);
    for bundle_a in find_combinations(&input.numbers, bundle_size) {
        let qe = bundle_a.iter().copied().product();
        let len = bundle_a.len();
        if len < front_seat_minimum.1 || (len == front_seat_minimum.1 && qe < front_seat_minimum.0) {
            front_seat_minimum = (qe, len);
        }
    }

    front_seat_minimum.0
}

fn part1(input: &Input) -> i64 {
    best_arrangement(input, 3)
}

fn part2(input: &Input) -> i64 {
    best_arrangement(input, 4)
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

    static SAMPLE: &str = indoc::indoc! {"
        1
        2
        3
        4
        5
        7
        8
        9
        10
        11
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 99);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 44);
    }
}
