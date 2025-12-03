//! # Solution for Advent of Code 2025 Day 2: Gift Shop
//!
//! Ref: [Advent of Code 2025 Day 2](https://adventofcode.com/2025/day/2)
//!
use anyhow::{Context, Error, Result, anyhow};
use std::io::{self, Read};
use std::str::FromStr;

struct Pair {
    start: i64,
    end: i64,
}
impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (start_str, end_str) = s.split_once('-').ok_or(anyhow!("Invalid pair: {s}"))?;
        let start = start_str
            .parse::<i64>()
            .context(format!("Parsing {start_str} as a start value"))?;
        let end = end_str
            .trim()
            .parse::<i64>()
            .context(format!("Parsing {end_str} as an end value"))?;
        Ok(Pair { start, end })
    }
}
struct Input {
    pairs: Vec<Pair>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Input {
            pairs: s.split(',').map(str::parse::<Pair>).collect::<Result<Vec<_>>>()?,
        })
    }
}

/// Returns an iterator over the proper (exclusive) divisors of a given number.
///
/// A proper divisor of `n` is any positive integer less than `n` that divides `n` evenly.
///
/// # Arguments
///
/// * `n` - A positive integer for which to find all proper divisors.
///
/// # Returns
///
/// An iterator over all integers `i` such that `1 <= i < n` and `n % i == 0`.
///
/// # Examples
///
/// ```
/// let divisors: Vec<_> = exclusive_divisors(12).collect();
/// assert_eq!(divisors, vec![1, 2, 3, 4, 6]);
/// ```
fn exclusive_divisors(n: usize) -> impl Iterator<Item = usize> {
    (1..=n/2).filter(move |&i| n.is_multiple_of(i))
}

impl Pair {
    /// Returns an iterator over all invalid IDs within this `Pair`'s inclusive range.
    ///
    /// The range is defined by `self.start..=self.end`.  
    /// An ID is considered *invalid* if:
    /// - It contains an even number of digits, and  
    /// - The first half of its digits are exactly the same as the second half.  
    ///
    /// For example, `1212`, `4444`, and `9898` are invalid because their digit
    /// sequences repeat in two equal halves.
    ///
    /// # Returns
    ///
    /// An iterator over all `i64` values in the pair's range that meet the invalid-ID criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// let pair = Pair { start: 1000, end: 1020 };
    /// let invalid: Vec<_> = pair.invalid_ids().collect();
    ///
    /// // 1010 is invalid because "10" == "10"
    /// assert!(invalid.contains(&1010));
    /// ```
    fn invalid_ids(&self) -> impl Iterator<Item = i64> {
        (self.start..=self.end).filter(|id| {
            let id_str = id.to_string();
            let num_digits = id_str.len();
            if !id_str.is_empty() && num_digits.is_multiple_of(2) {
                let (left, right) = id_str.split_at(num_digits / 2);
                left == right
            } else {
                false
            }
        })
    }

    /// Returns an iterator over all *truly invalid* IDs within this `Pair`'s inclusive range.
    ///
    /// An ID is considered **truly invalid** if its digit string can be evenly divided into
    /// repeating substrings of equal length. That is, the ID consists of multiple copies
    /// of the same substring, where the substring length is a proper divisor of the total number of digits.
    ///
    /// For example:
    /// - `1212` is truly invalid because it is `"12"` repeated
    /// - `123123` is truly invalid because it is `"123"` repeated
    ///
    /// # Returns
    ///
    /// An iterator over all `i64` values in the pair’s range `[self.start, self.end]`
    /// whose digit patterns match this truly invalid structure.
    ///
    /// # Examples
    ///
    /// ```
    /// let pair = Pair { start: 1000, end: 1700 };
    /// let truly_invalid: Vec<_> = pair.truly_invalid_ids().collect();
    ///
    /// assert_eq!(truly_invalid, vec![1111, 1212, 1313, 1414, 1515, 1616]);
    /// 
    /// let pair = Pair { start: 121100, end: 123300 };
    /// let truly_invalid: Vec<_> = pair.truly_invalid_ids().collect();
    /// assert_eq!(truly_invalid, vec![121121, 121212, 122122, 123123]);
    /// ```
    fn truly_invalid_ids(&self) -> impl Iterator<Item = i64> {
        (self.start..=self.end).filter(|id| {
            let id_str = id.to_string();
            let num_digits = id_str.len();
            exclusive_divisors(num_digits).any(|window_size| {
                let slice = &id_str[0..window_size];
                (window_size..num_digits).step_by(window_size).all(|comparison_start| {
                    let end = comparison_start + window_size;
                    &id_str[comparison_start..end] == slice
                })
            })
        })
    }
}

/// Computes the sum of all invalid IDs across all `Pair`s in the input.
///
/// For each `Pair` in the input, this function:
/// - Finds all *invalid* IDs within the pair's range using [`Pair::invalid_ids`], and
/// - Sums those IDs,
/// Then it returns the total sum across all pairs.
///
/// An ID is considered *invalid* if:
/// - It has an even number of digits, and
/// - Its first half is exactly equal to its second half (e.g., `1212`, `4444`)
///
/// # Arguments
///
/// * `input` - A reference to an `Input` containing a collection of `Pair`s.
///
/// # Returns
///
/// The total sum of all invalid IDs from all pairs in the input.
///
/// # Example
///
/// ```
/// let input = Input {
///     pairs: vec![
///         Pair { start: 1000, end: 1020 },
///         Pair { start: 1100, end: 1120 },
///     ],
/// };
///
/// let result = part1(&input);
/// assert_eq!(result, 2121); // sum of 1010 and 1111
/// ```
///
/// # See also
///
/// - [`Pair::invalid_ids`] for the definition of an invalid ID.
fn part1(input: &Input) -> i64 {
    input.pairs.iter().map(|pair| pair.invalid_ids().sum::<i64>()).sum()
}

/// Computes the sum of all *truly invalid* IDs across all `Pair`s in the input.
///
/// For each `Pair` in the input, this function:
/// - Identifies all truly invalid IDs using [`Pair::truly_invalid_ids`], and
/// - Sums those IDs,
/// Then it returns the total sum across all pairs.
///
/// An ID is considered *truly invalid* if its digit string can be evenly divided
/// into repeating substrings of equal length. The substring length must be a proper
/// divisor of the number of digits.  
/// Examples:
/// - `1212` → `"12"` repeated
/// - `123123` → `"123"` repeated
///
/// # Arguments
///
/// * `input` - A reference to an `Input` containing a collection of `Pair`s.
///
/// # Returns
///
/// The total sum of all truly invalid IDs from all pairs in the input.
///
/// # Example
///
/// ```
/// let input = Input {
///     pairs: vec![
///         Pair { start: 1000, end: 1020 },
///         Pair { start: 1100, end: 1120 },
///         Pair { start: 121100, end: 123300 },
///     ],
/// };
///
/// let result = part2(&input);
/// assert_eq!(result, 489699);
/// ```
///
/// # See also
///
/// - [`Pair::truly_invalid_ids`] for the definition of a truly invalid ID.
fn part2(input: &Input) -> i64 {
    input
        .pairs
        .iter()
        .map(|pair| pair.truly_invalid_ids().sum::<i64>())
        .sum()
}

/// Entry point for the program.
///
/// This function reads input from standard input, parses it into an [`Input`] structure,
/// and computes solutions for both parts of the problem:
///
/// - [`part1`] computes the sum of all invalid IDs.
/// - [`part2`] computes the sum of all truly invalid IDs.
///
/// It also measures and prints the total execution time.
///
/// # Input Format
///
/// The input is read from standard input (`stdin`) and must conform to the format
/// expected by the [`Input`] type's `FromStr` implementation.
///
/// # Output
///
/// The results for both parts, along with the execution time, are printed to standard output:
///
/// ```text
/// Part1: <sum of invalid IDs>
/// Part2: <sum of truly invalid IDs>
/// Time: <elapsed time>
/// ```
///
/// # Errors
///
/// Returns a [`Result::Err`] if:
/// - Reading from standard input fails
/// - Parsing the input into the `Input` type fails
///
/// # See also
///
/// - [`Input`]
/// - [`Pair::invalid_ids`]
/// - [`Pair::truly_invalid_ids`]
/// - [`part1`]
/// - [`part2`]
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
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 1_227_775_554);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 4_174_379_265);
    }
}
