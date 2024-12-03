//! # Solution for Advent of Code 2015 Day 18: Like a GIF For Your Yard
//!
//! Ref: [Advent of Code 2015 Day 18](https://adventofcode.com/2015/day/18)
//!
use ahash::AHashSet;
use anyhow::{anyhow, Error, Result};
use itertools::iproduct;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}
#[derive(Debug)]
struct Board(AHashSet<Point>);
impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, ch)| match ch {
                    '.' => None,
                    '#' => Some(
                        i32::try_from(row)
                            .and_then(|row| i32::try_from(col).map(|col| Point { row, col }))
                            .map_err(|err| Error::from(err).context("board should have a reasonable size")),
                    ),
                    _ => Some(Err(anyhow!("board markers should be '.' or '#'"))),
                })
            })
            .collect::<Result<AHashSet<_>>>()
            .map(Board)
    }
}

impl Board {
    fn neighbors(pt: &Point) -> Vec<Point> {
        iproduct!([-1, 0, 1].into_iter(), [-1, 0, 1].into_iter())
            .filter_map(|(dx, dy)| match (dx, dy) {
                (0, 0) => None,
                (dx, dy) => Some(Point {
                    row: pt.row + dy,
                    col: pt.col + dx,
                }),
            })
            .collect::<Vec<_>>()
    }
    fn num_active_neighbors(&self, pt: &Point) -> usize {
        Self::neighbors(pt).into_iter().filter(|pt| self.0.contains(pt)).count()
    }
    fn new_generation(&self, max_row: i32, max_col: i32) -> Board {
        Board(
            iproduct!(0..max_row, 0..max_col)
                .map(|(row, col)| Point { row, col })
                .filter(|pt| {
                    let an = self.num_active_neighbors(pt);
                    (self.0.contains(pt) && an == 2) || an == 3
                })
                .collect::<AHashSet<_>>(),
        )
    }
    fn add_corners(&mut self, max_row: i32, max_col: i32) {
        self.0.insert(Point { row: 0, col: 0 });
        self.0.insert(Point {
            row: 0,
            col: max_col - 1,
        });
        self.0.insert(Point {
            row: max_row - 1,
            col: 0,
        });
        self.0.insert(Point {
            row: max_row - 1,
            col: max_col - 1,
        });
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut board = input.parse::<Board>()?;
    for _ in 0..100 {
        board = board.new_generation(100, 100);
    }
    Ok(board.0.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut board = input.parse::<Board>()?;
    board.add_corners(100, 100);
    for _ in 0..100 {
        board = board.new_generation(100, 100);
        board.add_corners(100, 100);
    }
    Ok(board.0.len())
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}
