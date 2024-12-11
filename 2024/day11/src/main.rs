//! # Solution for Advent of Code 2024 Day 11: Plutonian Pebbles
//!
//! Ref: [Advent of Code 2024 Day 11](https://adventofcode.com/2024/day/11)
//!
//! This module implements a solution for transforming numbers according to specific rules:
//! - If the number is 0, it becomes 1
//! - If the number has an odd number of digits, it splits into two parts
//! - Otherwise, the number is multiplied by 2024
//!
//! The solution tracks how many chunks result after applying these transformations
//! repeatedly for a given number of steps.

use ahash::AHashMap;
use anyhow::{Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

/// Represents the parsed input containing initial numbers to transform
struct Input {
    /// Vector of integers to process
    nums: Vec<i64>,
}
impl FromStr for Input {
    type Err = Error;

    /// Parses space-separated numbers from a string into the Input struct
    fn from_str(s: &str) -> Result<Self> {
        let nums = s.split_whitespace().map(str::parse).collect::<Result<Vec<i64>, _>>()?;
        Ok(Input { nums })
    }
}

/// Applies one step of the transformation rules to a single number
///
/// # Arguments
/// * `num` - The number to transform
///
/// # Returns
/// A vector containing the resulting number(s) after transformation:
/// - Returns [1] if input is 0
/// - Returns [`left_half`, `right_half`] if input has an even number of digits
/// - Returns [`num` * 2024] otherwise
fn one_step(num: i64) -> Vec<i64> {
    match num {
        0 => vec![1],
        n if n.ilog10() % 2 == 1 => {
            let total_digits = n.ilog10() + 1;
            let divisor = 10_i64.pow(total_digits / 2);
            let left = n / divisor;
            let right = n % divisor;
            vec![left, right]
        }
        n => {
            vec![n * 2024]
        }
    }
}

impl Input {
    /// Runs the transformation process for a specified number of steps
    ///
    /// # Arguments
    /// * `steps` - Number of transformation steps to perform
    ///
    /// # Returns
    /// Total number of chunks after applying transformations to all initial numbers
    fn run(&self, steps: i64) -> usize {
        let rocks = &self.nums;
        let mut cache = Cache::new();
        rocks.iter().map(|&num| cache.chunks_after(num, steps)).sum()
    }
}

/// Caches intermediate results to avoid redundant calculations
struct Cache {
    /// Maps (`number`, `remaining_steps`) to the count of resulting chunks
    cache: AHashMap<(i64, i64), usize>,
}

impl Cache {
    /// Creates a new empty cache
    fn new() -> Self {
        Cache { cache: AHashMap::new() }
    }

    /// Calculates the number of chunks that result from transforming a number
    /// over a specified number of steps, using memoization for efficiency
    ///
    /// # Arguments
    /// * `num` - Starting number
    /// * `steps` - Number of remaining transformation steps
    ///
    /// # Returns
    /// Total number of chunks after all transformations
    fn chunks_after(&mut self, num: i64, steps: i64) -> usize {
        if steps == 0 {
            return 1;
        }
        if let Some(&n) = self.cache.get(&(num, steps)) {
            return n;
        }
        let rocks = one_step(num);
        let result = rocks
            .iter()
            .map(|&num| self.chunks_after(num, steps - 1))
            .sum::<usize>();
        self.cache.insert((num, steps), result);
        result
    }
}

/// Solves part 1 by running transformations for 25 steps
fn part1(input: &Input) -> usize {
    input.run(25)
}

/// Solves part 2 by running transformations for 75 steps
fn part2(input: &Input) -> usize {
    input.run(75)
}

/// Main function that reads input and solves both parts
///
/// # Errors
///
/// Returns an error if:
/// * Failed to read from stdin
/// * Failed to parse the input
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
        125 17
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 55312);
    }
}
