//! # Solution for Advent of Code 2022 Day 24: Blizzard Basin
//!
//! Ref: [Advent of Code 2022 Day 24](https://adventofcode.com/2022/day/24)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Error, Result};
use astar::{search_astar, AStarNode};
use num::traits::Zero;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::RefCell;
use std::hash::Hash;
use std::io::{self, Read};
use std::ops::{Div, Mul, Rem};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Blizzard {
    direction: Direction,
    fixed_coordinate: i64, // either the row or column this moves along
    offset: i64,
}
#[derive(Debug)]
struct Input {
    width: i64,
    height: i64,
    blizzards: Vec<Blizzard>,
    cycle_modulo: usize,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static START_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#\.#+$").unwrap());
        static END_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#+\.#$").unwrap());
        let mut iter = s.lines().enumerate();

        let (_, map_top) = iter.next().ok_or_else(|| anyhow!("Parse Error: empty input"))?;
        if !START_PATTERN.is_match(map_top) {
            bail!("Bad map parse (top line)");
        }
        let width = map_top.len() - 2;
        let mut blizzards = vec![];
        loop {
            let (idx, blizzards_or_end) = iter.next().ok_or_else(|| anyhow!("Parse Error: early termination"))?;
            if blizzards_or_end.len() != width + 2 {
                bail!("Inconsistent line widthts");
            }
            if END_PATTERN.is_match(blizzards_or_end) {
                let height = idx - 1;
                let cycle_modulo = lcm(height, width);
                return Ok(Input {
                    width: width.try_into()?,
                    height: height.try_into()?,
                    blizzards,
                    cycle_modulo,
                });
            }
            let mut ch_iter = blizzards_or_end.chars().enumerate();
            let (_, left_wall) = ch_iter.next().ok_or_else(|| anyhow!("empty line"))?;
            if left_wall != '#' {
                bail!("Left wall not seen");
            }
            for (col, ch) in ch_iter {
                if col > width + 1 {
                    bail!("Line too long");
                }
                match ch {
                    '#' => {
                        if col != width + 1 {
                            bail!("Early wall");
                        }
                    }
                    '<' => blizzards.push(Blizzard {
                        direction: Direction::Left,
                        fixed_coordinate: idx as i64 - 1,
                        offset: col as i64 - 1,
                    }),
                    '>' => blizzards.push(Blizzard {
                        direction: Direction::Right,
                        fixed_coordinate: idx as i64 - 1,
                        offset: col as i64 - 1,
                    }),
                    '^' => blizzards.push(Blizzard {
                        direction: Direction::Up,
                        fixed_coordinate: col as i64 - 1,
                        offset: idx as i64 - 1,
                    }),
                    'v' => blizzards.push(Blizzard {
                        direction: Direction::Down,
                        fixed_coordinate: col as i64 - 1,
                        offset: idx as i64 - 1,
                    }),
                    '.' => {}
                    _ => {
                        bail!("Bad map decode");
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    col: i64,
    row: i64,
}

fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + PartialEq + Zero + Copy,
{
    let mut u = a;
    let mut v = b;
    while v != T::zero() {
        let r = u % v;
        u = v;
        v = r;
    }
    u
}

fn lcm<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + PartialEq + Zero + Mul<Output = T> + Div<Output = T> + Copy,
{
    let gcd = gcd(a, b);
    a * b / gcd
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct TraversalState {
    cycle: usize, // Runs 0..lcm(width,height)
    row: i64,
    col: i64,
}

struct TraversalSharedInfo {
    cycle_modulo: usize,
    blizzards: Vec<Blizzard>,
    width: i64,
    height: i64,
    cache: RefCell<AHashMap<usize, AHashSet<Point>>>,
}
impl Input {
    fn start(&self, starting_cycle: usize) -> TraversalState {
        TraversalState {
            cycle: starting_cycle,
            row: -1,
            col: 0,
        }
    }
    fn goal(&self, starting_cycle: usize) -> TraversalState {
        TraversalState {
            cycle: starting_cycle,
            row: self.height,
            col: self.width - 1,
        }
    }
    fn info(&self) -> TraversalSharedInfo {
        TraversalSharedInfo {
            cycle_modulo: self.cycle_modulo,
            blizzards: self.blizzards.clone(),
            width: self.width,
            height: self.height,
            cache: RefCell::new(AHashMap::new()),
        }
    }
}
impl TraversalSharedInfo {
    fn blizzard_spots(&self, cycle: usize) -> AHashSet<Point> {
        let cached_item = self.cache.borrow().get(&cycle).cloned();
        if let Some(item) = cached_item {
            return item;
        }
        let mut snowy = AHashSet::new();
        for blizzard in self.blizzards.iter() {
            let pt_to_add = match &blizzard.direction {
                Direction::Up => Point {
                    col: blizzard.fixed_coordinate,
                    row: (blizzard.offset - cycle as i64).rem_euclid(self.height),
                },
                Direction::Down => Point {
                    col: blizzard.fixed_coordinate,
                    row: (blizzard.offset + cycle as i64).rem_euclid(self.height),
                },
                Direction::Left => Point {
                    col: (blizzard.offset - cycle as i64).rem_euclid(self.width),
                    row: blizzard.fixed_coordinate,
                },
                Direction::Right => Point {
                    col: (blizzard.offset + cycle as i64).rem_euclid(self.width),
                    row: blizzard.fixed_coordinate,
                },
            };
            snowy.insert(pt_to_add);
        }
        self.cache.borrow_mut().insert(cycle, snowy.clone());
        snowy
    }
}

struct NeighborIter {
    next_index: usize,
    cycle: usize,
    items: [Option<Point>; 5],
}
impl Iterator for NeighborIter {
    type Item = (TraversalState, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.items.len() {
            None
        } else {
            let rval = self.items[self.next_index].map(|pt| {
                (
                    TraversalState {
                        cycle: self.cycle,
                        row: pt.row,
                        col: pt.col,
                    },
                    1,
                )
            });
            self.next_index += 1;
            rval
        }
    }
}
impl AStarNode for TraversalState {
    type Cost = i64;

    type AssociatedState = TraversalSharedInfo;

    fn heuristic(&self, goal: &Self, _: &Self::AssociatedState) -> Self::Cost {
        // This is an optimistic assessment of the cost to reach the goal. In the case of the blizzard
        // simulation, it's just the Manhattan distance between the current location and the goal location.
        let dx = (goal.col - self.col).abs();
        let dy = (goal.row - self.row).abs();
        dx + dy
    }

    fn neighbors(&self, state: &Self::AssociatedState) -> impl Iterator<Item=(Self, Self::Cost)> {
        // Remember that a "neighbor" is "a new state we could transition to". So, "don't move" is also a
        // valid neighbor. This is the routine where we actually need to check the blizzard conditions.
        let next_cycle = (self.cycle + 1) % state.cycle_modulo;
        let next_blizzard_locations = state.blizzard_spots(next_cycle);
        let center = Point {
            col: self.col,
            row: self.row,
        };
        let mut idx = 0;
        let mut neighbor_buf: [Option<Point>; 5] = [None; 5];
        if !next_blizzard_locations.contains(&center) {
            neighbor_buf[idx] = Some(center);
            idx += 1;
        }
        if center.row > 0 {
            let above = Point {
                col: center.col,
                row: center.row - 1,
            };
            if !next_blizzard_locations.contains(&above) {
                neighbor_buf[idx] = Some(above);
                idx += 1;
            }
        }
        if center.row < state.height - 1 {
            let below = Point {
                col: center.col,
                row: center.row + 1,
            };
            if !next_blizzard_locations.contains(&below) {
                neighbor_buf[idx] = Some(below);
                idx += 1;
            }
        }
        if center.col > 0 && center.row >= 0 && center.row < state.height {
            let to_the_left = Point {
                col: center.col - 1,
                row: center.row,
            };
            if !next_blizzard_locations.contains(&to_the_left) {
                neighbor_buf[idx] = Some(to_the_left);
                idx += 1;
            }
        }
        if center.col < state.width - 1 && center.row >= 0 && center.row < state.height {
            let to_the_right = Point {
                col: center.col + 1,
                row: center.row,
            };
            if !next_blizzard_locations.contains(&to_the_right) {
                neighbor_buf[idx] = Some(to_the_right);
                idx += 1;
            }
        }
        if center.col == state.width - 1 && center.row == state.height - 1 {
            neighbor_buf[idx] = Some(Point {
                col: state.width - 1,
                row: state.height,
            });
            idx += 1;
        }
        if center.col == 0 && center.row == 0 {
            neighbor_buf[idx] = Some(Point { col: 0, row: -1 });
        }
        NeighborIter {
            next_index: 0,
            cycle: next_cycle,
            items: neighbor_buf,
        }
    }

    fn goal_match(&self, goal: &Self, _: &Self::AssociatedState) -> bool {
        // Have we reached the goal? Equality isn't precisely what we want for the blizzard, as we don't
        // actually care what spot we're at in the blizzard cycle when we hit the exit. (And we couldn't
        // predict it anyway.) All we care about is the map location.
        self.col == goal.col && self.row == goal.row
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let input = input.parse::<Input>()?;
    let info = input.info();
    let path = search_astar(input.start(0), input.goal(0), &info).unwrap();
    Ok(path.len() - 1)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let input = input.parse::<Input>()?;
    let info = input.info();
    let first_path = search_astar(input.start(0), input.goal(0), &info).unwrap();
    let second_start_time = first_path.len() - 1;
    let second_path = search_astar(input.goal(second_start_time), input.start(0), &info).unwrap();
    let third_start_time = first_path.len() + second_path.len() - 2;
    let third_path = search_astar(input.start(third_start_time), input.goal(0), &info).unwrap();
    Ok(first_path.len() + second_path.len() + third_path.len() - 3)
}

fn main() -> anyhow::Result<()> {
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
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 18);
    }

    #[test]
    fn no_blizzards_part1() {
        let map = indoc::indoc! {"
            #.######
            #......#
            #......#
            #......#
            #......#
            ######.#
        "};
        assert_eq!(part1(map).unwrap(), 10);
    }

    #[test]
    fn no_blizzards_part2() {
        let map = indoc::indoc! {"
            #.######
            #......#
            #......#
            #......#
            #......#
            ######.#
        "};
        assert_eq!(part2(map).unwrap(), 30);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 54);
    }
}
