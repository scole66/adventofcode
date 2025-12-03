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

fn exclusive_divisors(n: usize) -> impl Iterator<Item = usize> {
    (1..n).filter(move |&i| n.is_multiple_of(i))
}

impl Pair {
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

fn part1(input: &Input) -> i64 {
    input.pairs.iter().map(|pair| pair.invalid_ids().sum::<i64>()).sum()
}

fn part2(input: &Input) -> i64 {
    input
        .pairs
        .iter()
        .map(|pair| pair.truly_invalid_ids().sum::<i64>())
        .sum()
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
