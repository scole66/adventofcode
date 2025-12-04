//! # Solution for Advent of Code 2025 Day 4: Printing Department
//!
//! Ref: [Advent of Code 2025 Day 4](https://adventofcode.com/2025/day/4)
//!
use ahash::HashSet;
use anyhow::{Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    rolls: HashSet<(i64, i64)>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            rolls: s
                .lines()
                .enumerate()
                .flat_map(|(row, line)| line.chars().enumerate().map(move |(column, ch)| (column, row, ch)))
                .map(|(column, row, ch)| Ok((i64::try_from(column)?, i64::try_from(row)?, ch)))
                .filter_map(|result| match result {
                    Ok((column, row, ch)) => {
                        if ch == '@' {
                            Some(Ok((column, row)))
                        } else {
                            None
                        }
                    }
                    Err(err) => Some(Err(err)),
                })
                .collect::<Result<HashSet<_>>>()?,
        })
    }
}

impl Input {
    fn rolls_adjacent_to(&self, column: i64, row: i64) -> impl Iterator<Item = (i64, i64)> {
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
            .into_iter()
            .filter(move |&(delta_col, delta_row)| {
                let probe_pos = (column + delta_col, row + delta_row);
                self.rolls.contains(&probe_pos)
            })
    }

    fn remove_removable_rolls(&mut self) -> usize {
        let items = self
            .rolls
            .iter()
            .filter(|&&(column, row)| self.rolls_adjacent_to(column, row).count() < 4)
            .map(|&(col, row)| (col, row))
            .collect::<Vec<_>>();
        let num_items = items.len();
        for item in items {
            self.rolls.remove(&item);
        }
        num_items
    }
}

fn part1(input: &Input) -> usize {
    input
        .rolls
        .iter()
        .filter(|&&(column, row)| input.rolls_adjacent_to(column, row).count() < 4)
        .count()
}

fn part2(input: &mut Input) -> usize {
    let mut total_removed = 0;
    loop {
        let num_removed = input.remove_removable_rolls();
        total_removed += num_removed;
        if num_removed == 0 {
            break;
        }
    }
    total_removed
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let mut input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = part2(&mut input);
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
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 13);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&mut SAMPLE.parse::<Input>().unwrap()), 43);
    }
}
