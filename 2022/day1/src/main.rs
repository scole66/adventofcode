//! # Solution for Advent of Code 2022 Day 1
//!
//! Ref: [Advent of Code 2022 Day 1](https://adventofcode.com/2022/day/1)
//!

use anyhow::{self, Context};
use std::io::{self, BufRead};

struct Elf {
    total: usize,
}

impl Elf {
    pub fn new(items: Vec<usize>) -> Self {
        let total = items.iter().sum::<usize>();
        Self { total }
    }
}
struct Elves {
    individuals: Vec<Elf>,
}

struct ResultString(anyhow::Result<String>);
impl From<anyhow::Result<String>> for ResultString {
    /// Converts an `anyhow::Result<String>` into a `ResultString`
    fn from(src: anyhow::Result<String>) -> Self {
        Self(src)
    }
}
impl From<Result<String, std::io::Error>> for ResultString {
    /// Converts a `Result<String, std::io::Error>` into a `ResultString`
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}

impl FromIterator<ResultString> for anyhow::Result<Elves> {
    fn from_iter<T: IntoIterator<Item = ResultString>>(iter: T) -> Self {
        let mut elves = vec![];
        let mut current_elf = vec![];
        for ResultString(res) in iter.into_iter() {
            let line = res?;
            if line.is_empty() {
                elves.push(Elf::new(current_elf));
                current_elf = vec![];
            } else {
                let calories = line.parse::<usize>().map_err(anyhow::Error::from)?;
                current_elf.push(calories);
            }
        }
        if !current_elf.is_empty() {
            elves.push(Elf::new(current_elf));
        }
        Ok(Elves { individuals: elves })
    }
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut elves = stdin
        .lock()
        .lines()
        .map(ResultString::from)
        .collect::<anyhow::Result<Elves>>()
        .context("Failed to parse puzzle input from stdin")?;

    // Sort the elves from most to least
    elves.individuals.sort_by(|a, b| b.total.cmp(&a.total));

    let question_1 = elves.individuals[0].total;
    println!("Elf with most calories is carrying {question_1}.");

    let question_2 = elves.individuals[0..3].iter().map(|elf| elf.total).sum::<usize>();
    println!("Most cals for first 3: {question_2}");

    Ok(())
}
