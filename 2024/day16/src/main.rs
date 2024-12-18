//! # Solution for Advent of Code 2024 Day 16: Reindeer Maze
//!
//! Ref: [Advent of Code 2024 Day 16](https://adventofcode.com/2024/day/16)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use astar::{search_astar, AStarNode};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone)]
struct Input {
    map: AHashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let mut end = None;
        let mut map = AHashSet::new();
        for (row, line) in s.lines().enumerate() {
            let row = i64::try_from(row)?;
            for (col, ch) in line.chars().enumerate() {
                let col = i64::try_from(col)?;
                match ch {
                    '#' => {
                        map.insert((row, col));
                    }
                    '.' => {}
                    'S' => {
                        start = Some((row, col));
                    }
                    'E' => {
                        end = Some((row, col));
                    }
                    _ => bail!("Bad Map Item"),
                }
            }
        }
        let start = start.ok_or_else(|| anyhow!("Missing Start"))?;
        let end = end.ok_or_else(|| anyhow!("Missing End"))?;
        Ok(Input { map, start, end })
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
enum Facing {
    North,
    South,
    East,
    West,
}
impl Facing {
    fn clockwise(self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::South => Facing::West,
            Facing::East => Facing::South,
            Facing::West => Facing::North,
        }
    }

    fn counter_clockwise(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
            Facing::West => Facing::South,
        }
    }

    fn turn_cost(self, new_facing: Self) -> i64 {
        if self == new_facing {
            0
        } else if self.clockwise() == new_facing || self.counter_clockwise() == new_facing {
            1000
        } else {
            2000
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Node {
    row: i64,
    col: i64,
    facing: Facing,
}

impl Node {
    fn needed_facing(&self, new_spot: (i64, i64)) -> Facing {
        let (row, col) = new_spot;
        match (row - self.row, col - self.col) {
            (-1, 0) => Facing::North,
            (1, 0) => Facing::South,
            (0, -1) => Facing::West,
            (0, 1) => Facing::East,
            _ => panic!("invariants violated"),
        }
    }
}

impl AStarNode for Node {
    type Cost = i64;
    type AssociatedState = Input;

    fn heuristic(&self, goal: &Self, state: &Self::AssociatedState) -> Self::Cost {
        (goal.row - self.row).abs() + (goal.col - self.col).abs()
    }

    fn neighbors(&self, state: &Self::AssociatedState) -> impl Iterator<Item = (Self, Self::Cost)> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| (self.row + dx, self.col + dy))
            .filter(|probe| !state.map.contains(probe))
            .map(|(row, col)| {
                let new_facing = self.needed_facing((row, col));
                let cost = 1 + self.facing.turn_cost(new_facing);
                (
                    Node {
                        row,
                        col,
                        facing: new_facing,
                    },
                    cost,
                )
            })
    }

    fn goal_match(&self, goal: &Self, state: &Self::AssociatedState) -> bool {
        self.row == goal.row && self.col == goal.col
    }
}

fn path_cost(path: &[Node]) -> i64 {
    path.windows(2)
        .map(|items| {
            let prev = &items[0];
            let next = &items[1];
            let new_facing = prev.needed_facing((next.row, next.col));
            1 + prev.facing.turn_cost(new_facing)
        })
        .sum()
}

fn part1(input: &Input) -> i64 {
    let start = Node {
        row: input.start.0,
        col: input.start.1,
        facing: Facing::East,
    };
    let goal = Node {
        row: input.end.0,
        col: input.end.1,
        facing: Facing::East,
    };
    let path = search_astar(start, goal, input).unwrap();
    path_cost(&path)
}

fn part2(input: &Input) -> usize {
    let start = Node {
        row: input.start.0,
        col: input.start.1,
        facing: Facing::East,
    };
    let goal = Node {
        row: input.end.0,
        col: input.end.1,
        facing: Facing::East,
    };
    let first_path = search_astar(start, goal, input).unwrap();
    let target_cost = path_cost(&first_path);
    let mut good_seats = first_path.iter().map(|n| (n.row, n.col)).collect::<AHashSet<_>>();
    let mut to_check = good_seats.clone();
    let mut already_checked = AHashSet::from([input.start, input.end]);
    to_check.remove(&input.start);
    to_check.remove(&input.end);
    // Ok: For each seat in to_check, other than the start or finish: Turn it into a wall. See if another path
    // exists with the same cost. If so, add that other path to good_seats and add any spots not in
    // already_checked to to_check.
    while let Some(location) = to_check.iter().next().copied() {
        already_checked.insert(location);
        to_check.remove(&location);
        let mut new_input = input.clone();
        new_input.map.insert(location);
        if let Some(new_path) = search_astar(start, goal, &new_input) {
            let cost = path_cost(&new_path);
            if cost == target_cost {
                for Node { row, col, facing: _ } in new_path {
                    if !already_checked.contains(&(row, col)) {
                        good_seats.insert((row, col));
                        to_check.insert((row, col));
                    }
                }
            }
        }
    }
    good_seats.len()
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
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    static SAMPLE2: &str = indoc::indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test_case(SAMPLE => 7036; "first sample")]
    #[test_case(SAMPLE2 => 11048; "second sample")]
    fn part1_sample(inp: &str) -> i64 {
        part1(&inp.parse::<Input>().unwrap())
    }

    #[test_case(SAMPLE => 45; "first sample")]
    #[test_case(SAMPLE2 => 64; "second sample")]
    fn part2_sample(inp: &str) -> usize {
        part2(&inp.parse::<Input>().unwrap())
    }
}
