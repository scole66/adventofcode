//! # Solution for Advent of Code 2025 Day 3: Lobby
//!
//! Ref: [Advent of Code 2025 Day 3](https://adventofcode.com/2025/day/3)
//!
use anyhow::{Error, Result, anyhow};
use std::io::{self, Read};
use std::str::FromStr;

struct Bank {
    batteries: Vec<u32>,
}
impl FromStr for Bank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            batteries: s
                .chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .ok_or(anyhow!(format!("Bad digit {ch} in input string")))
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl Bank {
    /// Computes the maximum possible joltage by selecting a fixed number of batteries.
    ///
    /// This method selects `battery_count` batteries from the bank to form the largest possible
    /// number, digit by digit, from left to right. At each step, it chooses the **largest available**
    /// battery (by value) from a valid range of positions, preserving the order of selection.
    ///
    /// The selection process:
    /// - Starts from the beginning of the battery list.
    /// - For each digit of the result, looks ahead to find the largest remaining battery
    ///   in the valid range `[prior_location, num_batteries - batteries_left]`.
    /// - Adds that digit to the result, appending it to the joltage value.
    /// - Moves past the selected battery and continues the process until `battery_count` digits are chosen.
    ///
    /// If multiple batteries have the same value, the one appearing **earlier** in the list is preferred.
    ///
    /// # Arguments
    ///
    /// * `battery_count` – The number of batteries (digits) to select from the bank.
    ///
    /// # Returns
    ///
    /// The maximum joltage that can be formed using the selected number of batteries, as an `i64`.
    ///
    /// # Panics
    ///
    /// - If the slice range is invalid (e.g. `battery_count` is larger than `self.batteries.len()`).
    /// - If an index conversion fails (should not happen unless battery list is excessively large).
    ///
    /// # Example
    ///
    /// ```
    /// let bank = Bank { batteries: vec![3, 1, 4, 1, 5, 9] };
    /// let result = bank.maximum_joltage(3);
    /// assert_eq!(result, 459); // picks 4 → 5 → 9
    /// ```
    fn maximum_joltage(&self, battery_count: usize) -> i64 {
        let num_batteries = self.batteries.len();
        let mut joltage = 0;
        let mut batteries_to_process = battery_count;
        let mut prior_location = 0;
        while batteries_to_process > 0 {
            let (location, &largest_digit) = self.batteries[prior_location..=num_batteries - batteries_to_process]
                .iter()
                .enumerate()
                .max_by_key(|(idx, val)| {
                    (
                        **val,
                        -(i64::try_from(*idx).expect("we should have a reasonable number of batteries")),
                    )
                })
                .expect("there should be batteries");
            let digit = i64::from(largest_digit);
            joltage = joltage * 10 + digit;
            prior_location += location + 1;
            batteries_to_process -= 1;
        }
        joltage
    }
}

struct Input {
    banks: Vec<Bank>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            banks: s.lines().map(str::parse::<Bank>).collect::<Result<_>>()?,
        })
    }
}

fn part1(input: &Input) -> i64 {
    input.banks.iter().map(|bank| bank.maximum_joltage(2)).sum()
}

fn part2(input: &Input) -> i64 {
    input.banks.iter().map(|bank| bank.maximum_joltage(12)).sum()
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
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 357);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 3_121_910_778_619);
    }
}
