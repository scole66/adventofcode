//! # Solution for Advent of Code 2024 Day 10: Hoof It
//!
//! Ref: [Advent of Code 2024 Day 10](https://adventofcode.com/2024/day/10)
//!
//! This module implements a solution for finding paths through a height map grid where:
//! - Each cell contains a height from 0-9
//! - Valid paths must increase in height by exactly 1 at each step
//! - Part 1 counts reachable paths from height 0 to height 9
//! - Part 2 counts all possible valid paths from height 0 to height 9

use ahash::AHashMap;
use anyhow::{anyhow, Error, Result};
use astar::{search_astar, AStarNode};
use std::io::{self, Read};
use std::str::FromStr;

/// Represents the parsed input grid as a map of coordinates to heights
struct Input {
    /// Map of (row, col) coordinates to height values (0-9)
    topo: AHashMap<(i64, i64), i64>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn to_height(ch: char) -> Result<i64> {
            ch.to_digit(10)
                .map(i64::from)
                .ok_or_else(|| anyhow!("Improper height {ch}"))
        }

        let topo = s
            .lines()
            .enumerate()
            .flat_map(move |(row, line)| {
                let row = i64::try_from(row)?;
                Ok::<_, Error>(
                    line.chars()
                        .enumerate()
                        .map(move |(col, ch)| -> Result<((i64, i64), i64)> {
                            let col = i64::try_from(col)?;
                            let h = to_height(ch)?;
                            Ok(((row, col), h))
                        }),
                )
            })
            .flatten()
            .collect::<Result<AHashMap<_, _>, _>>()?;
        Ok(Input { topo })
    }
}

/// Represents a position in the grid for pathfinding
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    /// Row coordinate
    row: i64,
    /// Column coordinate
    col: i64,
}

impl AStarNode for Node {
    type Cost = i64;
    type AssociatedState = Input;

    fn heuristic(&self, goal: &Self, _: &Self::AssociatedState) -> Self::Cost {
        (goal.row - self.row).abs() + (goal.col - self.col).abs()
    }

    fn neighbors(&self, state: &Self::AssociatedState) -> impl Iterator<Item = (Self, Self::Cost)> {
        let current_height = *state.topo.get(&(self.row, self.col)).expect("Node should be in map");
        [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .into_iter()
            .map(|(dy, dx)| (self.row + dy, self.col + dx))
            .filter(move |&(row, col)| state.topo.get(&(row, col)).is_some_and(|h| *h == current_height + 1))
            .map(|(row, col)| (Node { row, col }, 1))
    }

    fn goal_match(&self, goal: &Self, _: &Self::AssociatedState) -> bool {
        self.row == goal.row && self.col == goal.col
    }
}

impl Input {
    /// Returns an iterator over all positions with the specified height
    fn all_of_height(&self, height: i64) -> impl Iterator<Item = Node> + '_ {
        self.topo.iter().filter_map(move |(pos, h)| {
            if *h == height {
                Some(Node { row: pos.0, col: pos.1 })
            } else {
                None
            }
        })
    }

    /// Returns an iterator over all positions with height 0
    fn zeros(&self) -> impl Iterator<Item = Node> + '_ {
        self.all_of_height(0)
    }

    /// Returns an iterator over all positions with height 9
    fn nines(&self) -> impl Iterator<Item = Node> + '_ {
        self.all_of_height(9)
    }

    /// Checks if there exists a valid path from start to goal
    /// where each step increases height by exactly 1
    fn reachable_from(&self, start: &Node, goal: &Node) -> bool {
        search_astar(*start, *goal, self).is_some()
    }

    /// Returns an iterator over all valid next positions that are exactly 1 height greater
    fn one_up_from(&self, start: Node) -> impl Iterator<Item = Node> + '_ {
        let current_height = self.topo.get(&(start.row, start.col));
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .filter_map(move |(delta_row, delta_col)| {
                let coords = (start.row + delta_row, start.col + delta_col);
                if self
                    .topo
                    .get(&coords)
                    .is_some_and(|h| *h == current_height.copied().unwrap_or(-20) + 1)
                {
                    Some(Node {
                        row: coords.0,
                        col: coords.1,
                    })
                } else {
                    None
                }
            })
    }

    /// Returns all valid paths from the start node to height 9,
    /// where each step increases height by exactly 1
    fn all_paths_from(&self, start: &Node) -> Vec<Vec<Node>> {
        let current_height = self.topo.get(&(start.row, start.col));
        if let Some(current_height) = current_height {
            let current_height = *current_height;
            if current_height == 9 {
                return vec![vec![*start]];
            }
            let mut paths_from_here: Vec<Vec<Node>> = Vec::new();
            let good_path_len = 9 - current_height;
            for neighbor in self.one_up_from(*start) {
                for next_path in self
                    .all_paths_from(&neighbor)
                    .into_iter()
                    .filter(|path| i64::try_from(path.len()).unwrap() == good_path_len)
                {
                    let mut new_path = vec![*start];
                    new_path.extend(next_path);
                    paths_from_here.push(new_path);
                }
            }
            paths_from_here
        } else {
            vec![]
        }
    }
}

/// Solves part 1: Count how many height-9 positions are reachable from each height-0 position
fn part1(input: &Input) -> usize {
    // For each zero, count the number of reachable 9's.
    input
        .zeros()
        .map(|zero| input.nines().filter(|nine| input.reachable_from(&zero, nine)).count())
        .sum()
}

/// Solves part 2: Count the total number of valid paths from height-0 to height-9 positions
fn part2(input: &Input) -> usize {
    // For each 0/9 pair: sum the number of paths between them
    input.zeros().map(|zero| input.all_paths_from(&zero).len()).sum()
}

/// Main function that reads input and solves both parts
///
/// # Errors
///
/// Returns an error if:
/// * Failed to read from stdin
/// * Failed to parse the input
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
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 36);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 81);
    }
}
