//! # Solution for Advent of Code 2022 Day 9: Rope Bridge
//!
//! Ref: [Advent of Code 2022 Day 9](https://adventofcode.com/2022/day/9)
//!
use ahash::AHashSet;
use anyhow::Context;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (insn_str, value_str) = s
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("Bad instruction parse with \"{s}\""))?;
        let value = value_str
            .parse::<isize>()
            .with_context(|| format!("Bad instruction parse with \"{s}\""))?;
        match insn_str {
            "U" => Ok(Instruction::Up(value)),
            "D" => Ok(Instruction::Down(value)),
            "L" => Ok(Instruction::Left(value)),
            "R" => Ok(Instruction::Right(value)),
            _ => Err(anyhow::anyhow!("Bad instruction parse with \"{s}\"")),
        }
    }
}

struct GameBoard {
    // coords are: (column, row)
    knots: Vec<(isize, isize)>,
    tail_visits: AHashSet<(isize, isize)>,
}

impl GameBoard {
    fn new(knot_count: usize) -> Self {
        let mut knots = Vec::with_capacity(knot_count);
        knots.extend(itertools::repeat_n((0, 0), knot_count));
        let mut tail_visits = AHashSet::new();
        tail_visits.insert((0, 0));
        GameBoard { knots, tail_visits }
    }

    fn moveit(&mut self, col_delta: isize, row_delta: isize) {
        self.knots[0] = (self.knots[0].0 + col_delta, self.knots[0].1 + row_delta);
        loop {
            let mut motion_detected = false;
            for idx in 1..self.knots.len() {
                let delta = (
                    self.knots[idx - 1].0 - self.knots[idx].0,
                    self.knots[idx - 1].1 - self.knots[idx].1,
                );
                if delta.0.abs() > 1 || delta.1.abs() > 1 {
                    self.knots[idx] = (
                        self.knots[idx].0 + delta.0.signum(),
                        self.knots[idx].1 + delta.1.signum(),
                    );
                    motion_detected = true;
                }
            }
            self.tail_visits.insert(self.knots[self.knots.len() - 1]);
            if !motion_detected {
                break;
            }
        }
    }

    fn down(&mut self, amt: isize) {
        // row decreases
        self.moveit(0, -amt)
    }
    fn up(&mut self, amt: isize) {
        // row increases
        self.moveit(0, amt)
    }
    fn left(&mut self, amt: isize) {
        // column decreases
        self.moveit(-amt, 0)
    }
    fn right(&mut self, amt: isize) {
        // column increases
        self.moveit(amt, 0)
    }

    fn tail_visits(&self) -> &AHashSet<(isize, isize)> {
        &self.tail_visits
    }
}

fn run_game(input: &str, knot_count: usize) -> anyhow::Result<usize> {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, anyhow::Error>>()?;

    let mut board = GameBoard::new(knot_count);
    for insn in instructions {
        match insn {
            Instruction::Down(val) => board.down(val),
            Instruction::Up(val) => board.up(val),
            Instruction::Left(val) => board.left(val),
            Instruction::Right(val) => board.right(val),
        }
    }

    Ok(board.tail_visits().len())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    run_game(input, 2)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    run_game(input, 10)
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
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    static SAMPLE2: &str = indoc::indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 13);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE2).unwrap(), 36);
    }
}
