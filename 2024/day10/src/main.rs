//! # Solution for Advent of Code 2024 Day 10: Hoof It
//!
//! Ref: [Advent of Code 2024 Day 10](https://adventofcode.com/2024/day/10)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use astar::{search_astar, AStarNode};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
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

struct World {
    topo: AHashMap<(i64, i64), i64>,
    width: i64,
    height: i64,
}

impl From<Input> for World {
    fn from(value: Input) -> Self {
        let Input { topo } = value;
        let (width, height) = {
            let mut max_row = -1;
            let mut max_col = -1;
            for (row, col) in topo.keys() {
                max_row = max_row.max(*row);
                max_col = max_col.max(*col);
            }
            (max_col + 1, max_row + 1)
        };
        Self { topo, width, height }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    row: i64,
    col: i64,
}

impl AStarNode for Node {
    type Cost = i64;
    type AssociatedState = World;

    fn heuristic(&self, goal: &Self, state: &Self::AssociatedState) -> Self::Cost {
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

    fn goal_match(&self, goal: &Self, state: &Self::AssociatedState) -> bool {
        self.row == goal.row && self.col == goal.col
    }
}

impl World {
    fn all_of_height(&self, height: i64) -> impl Iterator<Item = Node> + '_ {
        self.topo.iter().filter_map(move |(pos, h)| {
            if *h == height {
                Some(Node { row: pos.0, col: pos.1 })
            } else {
                None
            }
        })
    }

    fn zeros(&self) -> impl Iterator<Item = Node> + '_ {
        self.all_of_height(0)
    }

    fn nines(&self) -> impl Iterator<Item = Node> + '_ {
        self.all_of_height(9)
    }

    fn reachable_from(&self, start: &Node, goal: &Node) -> bool {
        search_astar(*start, *goal, self).is_some()
    }

    fn all_paths_from(&self, start: &Node, goal: &Node) -> Vec<Vec<Node>> {
        // This is wrong; this is only the shortest path:
        match search_astar(*start, *goal, self) {
            None => vec![],
            Some(path) => vec![path]
        }
    }
}

fn part1(input: &World) -> usize {
    // For each zero, count the number of reachable 9's.
    input
        .zeros()
        .map(|zero| input.nines().filter(|nine| input.reachable_from(&zero, nine)).count())
        .sum()
}

fn part2(input: &World) -> usize {
    // For each 0/9 pair: sum the number of paths between them
    input
        .zeros()
        .flat_map(|zero| input.nines().map(move |nine| input.all_paths_from(&zero, &nine).len()))
        .sum()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;
    let world = World::from(input);

    let start_time = std::time::Instant::now();
    let part1 = part1(&world);
    let part2 = part2(&world);
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
        assert_eq!(part1(&World::from(SAMPLE.parse::<Input>().unwrap())), 36);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&World::from(SAMPLE.parse::<Input>().unwrap())), 81);
    }
}
