//! # Solution for Advent of Code 2024 Day 11: Plutonian Pebbles
//!
//! Ref: [Advent of Code 2024 Day 11](https://adventofcode.com/2024/day/11)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;
use rayon::prelude::*;

struct Input {
    nums: Vec<i64>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let nums = s
            .trim()
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<i64>, _>>()?;
        Ok(Input { nums })
    }
}

fn one_step(num: i64) -> Vec<i64> {
    match num {
        0 => vec![1],
        n if n.ilog10() % 2 == 1 => {
            let total_digits = n.ilog10() + 1;
            let divisor = 10_i64.pow(total_digits / 2);
            let left = n / divisor;
            let right = n % divisor;
            vec![left, right]
        }
        n => {
            vec![n * 2024]
        }
    }
}

fn step_rocks(rocks: &mut Vec<i64>) {
    let mut idx = 0;
    while idx < rocks.len() {
        let current = rocks[idx];
        match current {
            0 => rocks[idx] = 1,
            n if n.ilog10() % 2 == 1 => {
                let total_digits = n.ilog10() + 1;
                let divisor = 10_i64.pow(total_digits / 2);
                let left = n / divisor;
                let right = n % divisor;
                rocks[idx] = left;
                rocks.insert(idx + 1, right);
                idx += 1;
            }
            n => {
                rocks[idx] = n * 2024;
            }
        }
        idx += 1;
    }
}

fn part1(input: &Input) -> usize {
    let mut rocks = input.nums.clone();
    for _ in 0..25 {
        step_rocks(&mut rocks);
    }
    rocks.len()
}

struct Cache {
    cache: AHashMap<(i64, i64), Vec<i64>>,
}

impl Cache {
    fn new() -> Self {
        Cache { cache: AHashMap::new() }
    }
    fn do_it(&mut self, num: i64, steps: i64) -> Vec<i64> {
        if let Some(v) = self.cache.get(&(num, steps)) {
            return v.clone();
        }
        let mut steps = steps;
        let mut rocks = vec![num];
        while steps > 0 {
            rocks = rocks
                .iter()
                .map(|&num| {
                    let one_step = self.cache.get(&(num, 1)).cloned().unwrap_or_else(|| {
                        let result = one_step(num);
                        self.cache.insert((num, 1), result.clone());
                        result
                    });
                    one_step
                })
                .flatten()
                .collect::<Vec<_>>();
            steps -= 1;
        }
        todo!()
    }
}

fn part2x(input: &Input) -> usize {
    let rocks = &input.nums;
    let mut cache = Cache::new();
    rocks.iter().map(|&num| cache.do_it(num, 25).len()).sum()
}

fn part2(input: &Input) -> usize {
    fn p2(rocks: &[i64], steps: i64) -> usize {
        if steps == 1 { 
            rocks.iter().map(|&num| one_step(num).len()).sum()
         } else {
            rocks.par_iter().map(|&num| {
                p2(&one_step(num), steps - 1)
            }).sum()
         }
    }
    let rocks = input.nums.clone();
    p2(&rocks, 75)
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
        125 17
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 55312);
    }
}
