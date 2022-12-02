//! # Solution for Advent of Code 2021 Day 17
//!
//! Ref: [Advent of Code 2021 Day 17](https://adventofcode.com/2021/day/17)
//!

use anyhow::{self, Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Data {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}
impl TryFrom<&str> for Data {
    type Error = anyhow::Error;
    fn try_from(src: &str) -> anyhow::Result<Self> {
        lazy_static! {
            static ref VALID_PATTERN: Regex = Regex::new("^target area: x=(?P<xmin>0|-?[1-9][0-9]*)..(?P<xmax>0|-?[1-9][0-9]*), y=(?P<ymin>0|-?[1-9][0-9]*)..(?P<ymax>0|-?[1-9][0-9]*)$").unwrap();
        }
        let captures = VALID_PATTERN
            .captures(src)
            .ok_or_else(|| anyhow::anyhow!("{} is not valid input", src))?;
        let xmin = captures.name("xmin").unwrap().as_str().parse::<i32>()?;
        let xmax = captures.name("xmax").unwrap().as_str().parse::<i32>()?;
        let ymin = captures.name("ymin").unwrap().as_str().parse::<i32>()?;
        let ymax = captures.name("ymax").unwrap().as_str().parse::<i32>()?;
        if xmin > xmax || ymin > ymax {
            anyhow::bail!("minima must be less than maxima");
        }
        if xmax < 0 {
            anyhow::bail!("We assume shooting to the right");
        }
        Ok(Data { xmin, xmax, ymin, ymax })
    }
}

impl Data {
    fn limits(&self) -> (i32, i32, i32, i32) {
        let max_x_velocity = self.xmax + 1; // go this or higher, and we overshoot on the first step.
        let min_x_velocity = (((1.0 + 8.0 * self.xmin as f64).sqrt() - 1.0) * 0.5).ceil() as i32; // go under this and we'll never get there.
        let min_y_velocity = self.ymin - 1;
        let max_y_velocity = self.ymax + (self.ymin - self.ymax).abs() * 100; // honestly no idea

        (min_x_velocity, max_x_velocity, min_y_velocity, max_y_velocity)
    }
}

#[derive(Debug, Clone)]
struct Stats {
    xpos: i32,
    ypos: i32,
    xvel: i32,
    yvel: i32,
}

impl Stats {
    fn step(self) -> Self {
        Self {
            xpos: self.xpos + self.xvel,
            ypos: self.ypos + self.yvel,
            xvel: (self.xvel.abs() - 1) * self.xvel.signum(),
            yvel: self.yvel - 1,
        }
    }

    fn in_target(&self, target: &Data) -> bool {
        target.xmin <= self.xpos && self.xpos <= target.xmax && target.ymin <= self.ypos && self.ypos <= target.ymax
    }

    fn beyond(&self, target: &Data) -> bool {
        self.ypos < target.ymin || self.xpos > target.xmax
    }
}

fn calculate(target: &Data) -> (i32, i32, i32) {
    let (min_x_velocity, max_x_velocity, min_y_velocity, max_y_velocity) = target.limits();
    let mut highest_y = 0;
    let mut best_xvel = 0;
    let mut best_yvel = 0;
    for initial_x in min_x_velocity..=max_x_velocity {
        for initial_y in min_y_velocity..=max_y_velocity {
            let mut state = Stats { xpos: 0, ypos: 0, xvel: initial_x, yvel: initial_y };
            let mut max_height = 0;
            while !state.beyond(target) {
                if state.ypos > max_height {
                    max_height = state.ypos;
                }
                if state.in_target(target) {
                    if max_height > highest_y {
                        highest_y = max_height;
                        best_xvel = initial_x;
                        best_yvel = initial_y;
                    }
                    break;
                }
                state = state.step();
            }
        }
    }

    (highest_y, best_xvel, best_yvel)
}

fn possibilities(target: &Data) -> usize {
    let (min_x_velocity, max_x_velocity, min_y_velocity, max_y_velocity) = target.limits();
    let mut valid_count = 0;
    for initial_x in min_x_velocity..=max_x_velocity {
        for initial_y in min_y_velocity..=max_y_velocity {
            let mut state = Stats { xpos: 0, ypos: 0, xvel: initial_x, yvel: initial_y };
            while !state.beyond(target) {
                if state.in_target(target) {
                    valid_count += 1;
                    break;
                }
                state = state.step();
            }
        }
    }

    valid_count
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(|r| r.map_err(anyhow::Error::from))
        .map(|r| r.and_then(|s| Data::try_from(s.as_str())))
        .collect::<anyhow::Result<Vec<_>>>()
        .and_then(|v| {
            v.first()
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Must have at least one line"))
        })
        .context("Failed to parse puzzle input from stdin")?;

    let (highest_y, best_xvel, best_yvel) = calculate(&input);
    println!(
        "Part 1: With initial velocity ({}, {}), reached height of {}",
        best_xvel, best_yvel, highest_y
    );

    println!(
        "Part 2: Number of potential initial velocities that reach the target: {}",
        possibilities(&input)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate() {
        let input = "target area: x=20..30, y=-10..-5";
        let target = Data::try_from(input).unwrap();
        let (h, x, y) = super::calculate(&target);
        assert_eq!(h, 45);
        assert_eq!(x, 6);
        assert_eq!(y, 9);
    }
    #[test]
    fn possibilities() {
        let input = "target area: x=20..30, y=-10..-5";
        let target = Data::try_from(input).unwrap();
        let count = super::possibilities(&target);
        assert_eq!(count, 112);
    }
}
