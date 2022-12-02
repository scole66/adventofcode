//! # Solution for Advent of Code 2022 Day 1
//!
//! Ref: [Advent of Code 2022 Day 1](https://adventofcode.com/2022/day/1)
//!

use anyhow::{self, Context};
use std::io::{self, BufRead};

struct ResultString(anyhow::Result<String>);
impl From<Result<String, std::io::Error>> for ResultString {
    /// Converts a `Result<String, std::io::Error>` into a `ResultString`
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}

type ElfCollection = Vec<usize>;

impl FromIterator<ResultString> for anyhow::Result<ElfCollection> {
    fn from_iter<T: IntoIterator<Item = ResultString>>(iter: T) -> Self {
        let mut elves = vec![];
        let mut current_elf = 0;
        for ResultString(res) in iter.into_iter() {
            let line = res?;
            if line.is_empty() {
                elves.push(current_elf);
                current_elf = 0;
            } else {
                let calories = line.parse::<usize>().map_err(anyhow::Error::from)?;
                current_elf += calories;
            }
        }
        if current_elf > 0 {
            elves.push(current_elf);
        }
        if elves.is_empty() {
            Err(anyhow::anyhow!("No elves in the input"))
        } else {
            Ok(elves)
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut elves = stdin
        .lock()
        .lines()
        .map(ResultString::from)
        .collect::<anyhow::Result<ElfCollection>>()
        .context("Failed to parse puzzle input from stdin")?;

    // Sort the elves from most to least
    elves.sort_by(|a, b| b.cmp(a));

    let question_1 = elves[0];
    println!("Elf with most calories is carrying {question_1}.");

    let question_2 = elves[0..3].iter().sum::<usize>();
    println!("Most cals for first 3: {question_2}");

    Ok(())
}
