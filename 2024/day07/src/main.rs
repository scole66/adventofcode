//! # Solution for Advent of Code 2024 Day 7: Bridge Repair
//!
//! Ref: [Advent of Code 2024 Day 7](https://adventofcode.com/2024/day/7)
//!
//! This module implements a solution for finding valid mathematical expressions
//! that evaluate to target values using addition, multiplication, and concatenation
//! operations.

use anyhow::{anyhow, Context, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

/// Represents a mathematical equation with a target result and a list of operands
/// that must be combined to reach that result.
struct Equation {
    /// The target value that must be reached
    result: i64,
    /// The list of operands that can be combined using allowed operations
    operands: Box<[i64]>,
}

impl FromStr for Equation {
    type Err = Error;

    /// Parses an equation from a string in the format "result: num1 num2 num3..."
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(Equation)` - Successfully parsed equation
    /// * `Err` - If the string format is invalid or numbers can't be parsed
    fn from_str(s: &str) -> Result<Self> {
        let (result, operands) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Equation parse failure: no colon delimiter: {s:?}"))?;
        let result = result
            .parse::<i64>()
            .context(format!("Equation parse failure: Line {s:?}, parsing result {result:?}"))?;
        let operands = operands
            .split(' ')
            .map(|num| {
                num.parse()
                    .context(format!("Equation parse failure: Line {s:?}; parsing operand {num:?}"))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { result, operands })
    }
}

/// Represents the available mathematical operations that can be used to combine operands.
#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Add,
    Mul,
    Concatenate,
}
impl Equation {
    /// Checks if the equation can be solved using the given set of operations.
    ///
    /// # Arguments
    ///
    /// * `allowed_ops` - Slice of operations that can be used
    ///
    /// # Returns
    ///
    /// `true` if there exists a valid combination of operations that reaches
    /// the target result, `false` otherwise.
    fn has_solution_with(&self, allowed_ops: &[Operation]) -> bool {
        let Equation { result, operands } = self;
        allowed_ops.iter().copied().any(|op| {
            let left = operands[0];
            let right = &operands[1..];
            Self::addmul_subexpression_ok(*result, left, right, op, allowed_ops)
        })
    }

    /// Recursively checks if a subexpression can reach the target value.
    ///
    /// # Arguments
    ///
    /// * `target` - The target value to reach
    /// * `left` - The current left-hand value
    /// * `right` - Remaining operands to process
    /// * `op` - The operation to apply
    /// * `allowed_ops` - Available operations for subsequent combinations
    ///
    /// # Returns
    ///
    /// `true` if the target can be reached, `false` otherwise
    fn addmul_subexpression_ok(
        target: i64,
        left: i64,
        right: &[i64],
        op: Operation,
        allowed_ops: &[Operation],
    ) -> bool {
        if right.is_empty() {
            target == left
        } else if left > target {
            false
        } else {
            let new_left = match op {
                Operation::Add => left + right[0],
                Operation::Mul => left * right[0],
                Operation::Concatenate => format!("{left}{}", right[0])
                    .parse::<i64>()
                    .expect("math should be in bounds"),
            };
            let new_right = &right[1..];
            allowed_ops
                .iter()
                .copied()
                .any(|op| Self::addmul_subexpression_ok(target, new_left, new_right, op, allowed_ops))
        }
    }
}

/// Contains the parsed input data consisting of multiple equations.
struct Input {
    /// List of equations to solve
    equations: Box<[Equation]>,
}

impl FromStr for Input {
    type Err = Error;

    /// Parses multiple equations from a string, one per line
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            equations: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

/// Solves part 1: sum of results for equations solvable with addition and multiplication
///
/// # Arguments
///
/// * `input` - The parsed input containing all equations
///
/// # Returns
///
/// Sum of results for equations that can be solved using only addition and multiplication
fn part1(input: &Input) -> i64 {
    let mut sum = 0;
    for equation in &input.equations {
        if equation.has_solution_with(&[Operation::Add, Operation::Mul]) {
            sum += equation.result;
        }
    }
    sum
}

/// Solves part 2: sum of results for equations solvable with addition, multiplication,
/// and concatenation
///
/// # Arguments
///
/// * `input` - The parsed input containing all equations
///
/// # Returns
///
/// Sum of results for equations that can be solved using addition, multiplication,
/// and concatenation
fn part2(input: &Input) -> i64 {
    let mut sum = 0;
    for equation in &input.equations {
        if equation.has_solution_with(&[Operation::Add, Operation::Mul, Operation::Concatenate]) {
            sum += equation.result;
        }
    }
    sum
}

/// Main function that reads input from stdin and solves both parts of the puzzle
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
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 3749);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 11387);
    }
}
