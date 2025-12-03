//! # Advent of Code 2025 – Day 3: Lobby
//!
//! This binary solves [Advent of Code 2025 Day 3](https://adventofcode.com/2025/day/3).
//!
//! The challenge involves selecting digits from a list of batteries in order to maximize
//! a joltage value, under specific constraints on selection order. The input consists of
//! one or more lines of digit sequences, each representing a battery bank.
//!
//! The solution is implemented as a `Bank` type that supports greedy digit selection, and
//! is evaluated in two parts:
//!
//! - **Part 1**: Computes the maximum joltage using 2 battery digits.
//! - **Part 2**: Computes the maximum joltage using 12 battery digits.
//!
//! ## How to Run
//!
//! ```sh
//! cargo run --release < input.txt
//! ```
//!
//! This will parse the input, compute solutions for both parts, and print:
//!
//! ```text
//! Part1: <result>
//! Part2: <result>
//! Time: <elapsed time>
//! ```
//!
//! ## Key Components
//!
//! - [`Bank`]: Parses a line of digit input and stores battery values.
//! - [`Bank::maximum_joltage`]: Core logic to compute the highest possible joltage
//!   using greedy selection of digits.
//! - `part1`, `part2`: Functions that apply the joltage algorithm to parsed input.
//! - `main`: Entry point that handles I/O, parsing, timing, and result output.
//!
//! ## Dependencies
//!
//! - [`mod@anyhow`] is used for error handling.
//!
//! ## Performance
//!
//! The program measures total runtime using [`std::time::Instant`] and prints the
//! duration to help track performance on large inputs.

use anyhow::{Error, Result, anyhow};
use std::cmp::min;
use std::io::{self, Read};
use std::str::FromStr;

/// A collection of battery digits used to compute maximum joltage values.
///
/// Each battery is represented by a single digit (`0–9`), and the entire sequence is stored
/// as a `Vec<u32>`. The [`Bank`] type supports calculating the largest possible joltage that can
/// be formed by selecting a fixed number of batteries, preserving order and choosing greedily.
///
/// Input is typically parsed from a string of digits via [`FromStr`].
///
/// # Example
///
/// ```
/// use std::str::FromStr;
///
/// let bank = Bank::from_str("314159").unwrap();
/// assert_eq!(bank.batteries, vec![3, 1, 4, 1, 5, 9]);
/// ```
///
/// # See also
///
/// - [`Bank::maximum_joltage`] — Calculates the maximum joltage from selected batteries.
struct Bank {
    batteries: Vec<u32>,
}
impl FromStr for Bank {
    type Err = Error;

    /// Parses a `Bank` from a string of digits.
    ///
    /// Each character in the input string must be a digit (`'0'`–`'9'`). The digits are
    /// converted into a vector of battery values stored in the [`Bank`] struct.
    ///
    /// # Errors
    ///
    /// Returns an error if any character in the input is not a valid base-10 digit.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// let bank = Bank::from_str("314159").unwrap();
    /// assert_eq!(bank.batteries, vec![3, 1, 4, 1, 5, 9]);
    /// ```
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
        (1..=min(num_batteries, battery_count))
            .rev()
            .fold((0, 0), |(prior_location, joltage), batteries_to_process| {
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
                (prior_location + location + 1, joltage * 10 + digit)
            })
            .1
    }
}

/// Represents the full input for the puzzle, consisting of multiple battery banks.
///
/// Each line of the input corresponds to one [`Bank`], parsed from a sequence of digits.
/// This struct wraps all banks so they can be processed collectively in [`part1`] and [`part2`].
///
/// # Example
///
/// ```
/// let input = Input {
///     banks: vec![
///         Bank { batteries: vec![3, 1, 4, 1, 5, 9] },
///         Bank { batteries: vec![2, 6, 5, 3, 5, 8] },
///     ],
/// };
/// assert_eq!(input.banks.len(), 2);
/// ```
struct Input {
    banks: Vec<Bank>,
}
impl FromStr for Input {
    type Err = Error;

    /// Parses an [`Input`] from a multi-line string, where each line represents a [`Bank`].
    ///
    /// Each line must consist of digits (`'0'`–`'9'`) with no spaces or separators.
    /// Lines are parsed into [`Bank`] values using their [`FromStr`] implementation.
    ///
    /// # Errors
    ///
    /// Returns an error if any line contains non-digit characters or fails to parse as a [`Bank`].
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    /// # use your_crate::{Input, Bank}; // replace with actual module path
    ///
    /// let raw = "314159\n265358\n";
    /// let input = Input::from_str(raw).unwrap();
    /// assert_eq!(input.banks.len(), 2);
    /// assert_eq!(input.banks[0].batteries, vec![3, 1, 4, 1, 5, 9]);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            banks: s.lines().map(str::parse::<Bank>).collect::<Result<_>>()?,
        })
    }
}

/// Computes the solution to Part 1 of the puzzle.
///
/// For each [`Bank`] in the input, selects the **2 most powerful batteries** (digits),
/// preserving order and choosing greedily to form the largest possible joltage number.
/// The sum of all such joltage values is returned.
///
/// # Arguments
///
/// * `input` – A reference to the parsed [`Input`] containing all battery banks.
///
/// # Returns
///
/// The total sum of maximum joltages from all banks using 2 batteries each.
///
/// # Example
///
/// ```
/// let input = Input::from_str("314159\n265358").unwrap();
/// let result = part1(&input);
/// assert_eq!(result, 59 + 68); // picks 5→9 and 6→8 from each line
/// ```
fn part1(input: &Input) -> i64 {
    input.banks.iter().map(|bank| bank.maximum_joltage(2)).sum()
}

/// Computes the solution to Part 2 of the puzzle.
///
/// For each [`Bank`] in the input, selects the **12 most powerful batteries** (digits),
/// preserving order and choosing greedily to form the largest possible joltage number.
/// The sum of all such joltage values is returned.
///
/// This function assumes that each bank has at least 12 digits, as required by the problem.
///
/// # Arguments
///
/// * `input` – A reference to the parsed [`Input`] containing all battery banks.
///
/// # Returns
///
/// The total sum of maximum joltages from all banks using 12 batteries each.
///
/// # Example
///
/// ```
/// let input = Input::from_str("987654321098\n123456789012").unwrap();
/// let result = part2(&input);
/// assert_eq!(result, 987654321098 + 123456789012); // example values
/// ```
fn part2(input: &Input) -> i64 {
    input.banks.iter().map(|bank| bank.maximum_joltage(12)).sum()
}

/// Program entry point.
///
/// This function reads input from standard input, parses it into an [`Input`] object,
/// and computes the answers to both parts of the puzzle by calling [`part1`] and [`part2`].
/// It also measures and prints the total execution time.
///
/// Input should be provided as a list of digit strings, one per line, each representing a [`Bank`].
///
/// # Example
///
/// Run the program with:
///
/// ```sh
/// cargo run --release < input.txt
/// ```
///
/// Example output:
///
/// ```text
/// Part1: 114
/// Part2: 987654321012
/// Time: 12.34ms
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Reading from stdin fails.
/// - The input cannot be parsed into [`Input`] (e.g., due to non-digit characters).
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
