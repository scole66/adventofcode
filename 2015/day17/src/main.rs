//! # Solution for Advent of Code 2015 Day 17: No Such Thing as Too Much
//!
//! Ref: [Advent of Code 2015 Day 17](https://adventofcode.com/2015/day/17)
//!
use anyhow::{Context, Error, Result};
use itertools::chain;
use std::io::{self, Read};
use std::str::FromStr;

struct ContainerSet(Vec<i32>);
impl FromStr for ContainerSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ContainerSet(
            s.lines()
                .map(|line| {
                    line.parse::<i32>().map_err(|err| {
                        Error::from(err).context(format!(
                            "should see only integer-valued container sizes (not \"{line}\")"
                        ))
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

impl ContainerSet {
    fn combos(items: &[i32], size: usize, offset: usize) -> Vec<Vec<i32>> {
        if size == 0 {
            vec![vec![]]
        } else if items.is_empty() {
            vec![]
        } else {
            let first = usize::try_from(items[0]).expect("container sizes should not be negative");
            let p1 = if first <= size {
                Self::combos(&items[1..], size - first, offset + 1)
                .into_iter().map(|list| chain!(vec![i32::try_from(offset).expect("there shouldn't be more than a few thousand containers")], list).collect::<Vec<_>>())
                .collect::<Vec<_>>()
            } else {
                vec![]
            };
            let p2 = Self::combos(&items[1..], size, offset + 1);
            chain!(p1, p2).collect::<Vec<_>>()
        }
    }

    fn combos_with_size(&self, size: usize) -> Vec<Vec<i32>> {
        Self::combos(&self.0, size, 0)
    }
}

fn part1(input: &str) -> Result<usize> {
    let containers = input.parse::<ContainerSet>().context("input stream should be valid")?;
    let combos = containers.combos_with_size(150);
    Ok(combos.len())
}

fn part2(input: &str) -> Result<usize> {
    let containers = input.parse::<ContainerSet>().context("input stream should be valid")?;
    let mut combos = containers.combos_with_size(150);
    combos.sort_by_key(|lst| lst.len());
    let shortest_length = combos[0].len();
    Ok(combos.iter().filter(|&lst| lst.len() == shortest_length).count())
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        20
        15
        10
        5
        5
    "};

    #[test]
    fn part1_sample() {
        let cs = SAMPLE.parse::<ContainerSet>().unwrap();
        assert_eq!(cs.combos_with_size(25).len(), 4);
    }

    #[test]
    fn combos() {
        let cs = SAMPLE.parse::<ContainerSet>().unwrap();
        let combos = cs.combos_with_size(25);
        assert!(combos.contains(&vec![0, 3]));
        assert!(combos.contains(&vec![0, 4]));
        assert!(combos.contains(&vec![1, 2]));
        assert!(combos.contains(&vec![1, 3, 4]));
        assert_eq!(combos.len(), 4);
    }
}
