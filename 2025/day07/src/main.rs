//! # Solution for Advent of Code 2025 Day 7: Laboratories
//!
//! Ref: [Advent of Code 2025 Day 7](https://adventofcode.com/2025/day/7)
//!
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use anyhow::{Error, Result, anyhow, bail};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Start,
    Splitter,
}

#[derive(Debug)]
struct Input {
    start: (i64, i64),
    splitters: HashSet<(i64, i64)>,
    height: i64,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut items = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, ch)| match ch {
                    '^' => Some(((row, col), Kind::Splitter)),
                    'S' => Some(((row, col), Kind::Start)),
                    _ => None,
                })
            })
            .collect::<HashMap<(usize, usize), Kind>>();
        let starts = items
            .extract_if(|_, v| match v {
                Kind::Splitter => false,
                Kind::Start => true,
            })
            .collect::<Vec<_>>();
        if starts.len() != 1 {
            bail!("Too many (or zero) start locations in input data");
        }

        let start = (i64::try_from(starts[0].0.0)?, i64::try_from(starts[0].0.1)?);
        let splitters = items
            .keys()
            .map(|(row, col)| Ok((i64::try_from(*row)?, i64::try_from(*col)?)))
            .collect::<Result<HashSet<_>>>()?;
        let height = 1 + splitters
            .iter()
            .max_by_key(|(row, _)| *row)
            .ok_or(anyhow!("no splitters in input"))?
            .0;

        Ok(Input {
            start,
            splitters,
            height,
        })
    }
}

fn part1(input: &Input) -> usize {
    let mut previous_paths = HashSet::new();
    let mut splits = 0;
    previous_paths.insert(input.start.1);
    for row in input.start.0 + 1..input.height {
        let mut next_row = HashSet::new();
        for column in previous_paths {
            if input.splitters.contains(&(row, column)) {
                next_row.insert(column - 1);
                next_row.insert(column + 1);
                splits += 1;
            } else {
                next_row.insert(column);
            }
        }
        previous_paths = next_row;
    }
    splits
}

impl Input {
    fn find_futures(&self, column: i64, row: i64, futures: &HashMap<(i64, i64), i64>) -> i64 {
        if let Some(future) = (row + 1..self.height).find_map(|r| futures.get(&(r, column))) {
            *future
        } else {
            1
        }
    }
}

fn part2(input: &Input) -> i64 {
    let mut potential_futures = HashMap::new();
    for row in (input.start.0..input.height).rev() {
        for splitter in input
            .splitters
            .iter()
            .filter(|&&(splitter_row, _)| row == splitter_row)
            .copied()
        {
            let left = input.find_futures(splitter.1 - 1, row, &potential_futures);
            let right = input.find_futures(splitter.1 + 1, row, &potential_futures);
            potential_futures.insert(splitter, left + right);
        }
    }
    input.find_futures(input.start.1, input.start.0, &potential_futures)
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
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 21);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 40);
    }
}
