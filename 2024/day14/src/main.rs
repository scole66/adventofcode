//! # Solution for Advent of Code 2024 Day 14: Restroom Redoubt
//!
//! Ref: [Advent of Code 2024 Day 14](https://adventofcode.com/2024/day/14)
//!
#![expect(clippy::cast_precision_loss)]
use anyhow::{anyhow, bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct RobotInfo {
    starting_position: (i64, i64),
    velocity: (i64, i64),
}

const FIELD_WIDTH: i64 = 101;
const FIELD_HEIGHT: i64 = 103;

impl FromStr for RobotInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (initial, velo) = s.split_once(' ').ok_or_else(|| anyhow!("Bad robot"))?;
        let (id, info) = initial.split_once('=').ok_or_else(|| anyhow!("Bad robot"))?;
        if id != "p" {
            bail!("Bad robot");
        }
        let (init_x, init_y) = info.split_once(',').ok_or_else(|| anyhow!("bad robot"))?;
        let sp_x = init_x.parse::<i64>()?;
        let sp_y = init_y.parse::<i64>()?;
        let (id, velo) = velo.split_once('=').ok_or_else(|| anyhow!("bad robot"))?;
        if id != "v" {
            bail!("bad robot");
        }
        let (velo_x, velo_y) = velo.split_once(',').ok_or_else(|| anyhow!("bad robot"))?;
        let vx = velo_x.parse::<i64>()?;
        let vy = velo_y.parse::<i64>()?;

        Ok(RobotInfo {
            starting_position: (sp_x, sp_y),
            velocity: (vx, vy),
        })
    }
}

impl RobotInfo {
    fn after(&self, seconds: i64, field_width: i64, field_height: i64) -> (i64, i64) {
        let (px, py) = self.starting_position;
        let (vx, vy) = self.velocity;
        let new_x = (px + seconds * vx).rem_euclid(field_width);
        let new_y = (py + seconds * vy).rem_euclid(field_height);

        (new_x, new_y)
    }
}

// 0 1 2 3 4 XX 6 7 8 9 10   (width=11)

fn count_by_quadrant(
    positions: impl Iterator<Item = (i64, i64)>,
    width: i64,
    height: i64,
) -> (usize, usize, usize, usize) {
    positions.fold((0, 0, 0, 0), |acc, position| match position {
        (x, y) if x < width / 2 && y < height / 2 => (acc.0 + 1, acc.1, acc.2, acc.3),
        (x, y) if x < width / 2 && y > height / 2 => (acc.0, acc.1 + 1, acc.2, acc.3),
        (x, y) if x > width / 2 && y < height / 2 => (acc.0, acc.1, acc.2 + 1, acc.3),
        (x, y) if x > width / 2 && y > height / 2 => (acc.0, acc.1, acc.2, acc.3 + 1),
        _ => (acc.0, acc.1, acc.2, acc.3),
    })
}

struct Input {
    robot_info: Vec<RobotInfo>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Input {
            robot_info: s.lines().map(RobotInfo::from_str).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Input {
    fn safety_factor(&self, seconds: i64, width: i64, height: i64) -> usize {
        let (q1, q2, q3, q4) = count_by_quadrant(
            self.robot_info.iter().map(|ri| ri.after(seconds, width, height)),
            width,
            height,
        );
        q1 * q2 * q3 * q4
    }

    fn average_position_after(&self, seconds: i64, width: i64, height: i64) -> (f64, f64) {
        let num_robots = self.robot_info.len() as f64;
        let sum_all_positions = self
            .robot_info
            .iter()
            .map(|ri| ri.after(seconds, width, height))
            .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));
        (
            sum_all_positions.0 as f64 / num_robots,
            sum_all_positions.1 as f64 / num_robots,
        )
    }

    fn distance_variance_after(&self, seconds: i64, width: i64, height: i64) -> f64 {
        let num_robots = self.robot_info.len() as f64;
        let average_location = self.average_position_after(seconds, width, height);
        self.robot_info
            .iter()
            .map(|ri| ri.after(seconds, width, height))
            .map(|(x, y)| {
                let x = x as f64;
                let y = y as f64;
                (x - average_location.0).powf(2.0) + (y - average_location.1).powf(2.0)
            })
            .sum::<f64>()
            / num_robots
    }
}

fn part1(input: &Input, width: i64, height: i64) -> usize {
    input.safety_factor(100, width, height)
}

fn part2(input: &Input, width: i64, height: i64) -> i64 {
    let iter_limit = width * height;
    let mut best_step = (-1, f64::INFINITY);
    for step in 0..iter_limit {
        let variance = input.distance_variance_after(step, width, height);
        if variance < best_step.1 {
            best_step = (step, variance);
        }
    }
    best_step.0
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input, FIELD_WIDTH, FIELD_HEIGHT);
    let part2 = part2(&input, FIELD_WIDTH, FIELD_HEIGHT);
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
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap(), 11, 7), 12);
    }
}
