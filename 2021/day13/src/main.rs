//! # Solution for Advent of Code 2021 Day 13
//!
//! Ref: [Advent of Code 2021 Day 13](https://adventofcode.com/2021/day/13)
//!

use ahash::AHashSet;
use anyhow::{self, Context};
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::sync::LazyLock;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position((i32, i32));
impl Position {
    fn parse(s: &str) -> anyhow::Result<Self> {
        static POS_PATTERN: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("^(?P<col>0|[1-9][0-9]*),(?P<row>0|[1-9][0-9]*)$").unwrap());
        let captures = POS_PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("‘{}’ is not a valid position description", s))?;
        let col_str = captures.name("col").unwrap().as_str();
        let column = col_str.parse::<i32>().context(format!(
            "{col_str} is not a valid 32-bit integer (in position description)",
        ))?;
        let row_str = captures.name("row").unwrap().as_str();
        let row = row_str.parse::<i32>().context(format!(
            "{row_str} is not a valid 32-bit integer (in position description)",
        ))?;
        Ok(Position((column, row)))
    }
}

#[derive(Debug, Clone)]
struct Grid(AHashSet<Position>);
impl Grid {
    fn apply_instruction(&mut self, inst: &Instruction) {
        let mut to_remove = vec![];
        let mut to_add = vec![];
        for spot in self.0.iter() {
            let (col, row) = spot.0;
            match inst {
                Instruction::Vertical(val) => {
                    // Vertical fold means we mirror on X
                    if col >= *val {
                        if col != *val {
                            to_add.push(Position((2 * val - col, row)));
                        }
                        to_remove.push(*spot);
                    }
                }
                Instruction::Horizontal(val) => {
                    // Horizontal fold means we mirror on Y
                    if row >= *val {
                        if row != *val {
                            to_add.push(Position((col, 2 * val - row)));
                        }
                        to_remove.push(*spot);
                    }
                }
            }
        }
        for pos in to_remove {
            self.0.remove(&pos);
        }
        for pos in to_add {
            self.0.insert(pos);
        }
    }

    fn visible_dots(&self) -> usize {
        self.0.len()
    }

    fn extents(&self) -> Option<(i32, i32, i32, i32)> {
        if self.0.is_empty() {
            None
        } else {
            let mut min_col = i32::MAX;
            let mut min_row = i32::MAX;
            let mut max_col = i32::MIN;
            let mut max_row = i32::MIN;
            for item in self.0.iter() {
                let (col, row) = item.0;
                if col < min_col {
                    min_col = col;
                }
                if col > max_col {
                    max_col = col;
                }
                if row < min_row {
                    min_row = row;
                }
                if row > max_row {
                    max_row = row;
                }
            }
            Some((min_col, max_col, min_row, max_row))
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.extents() {
            None => f.write_str("[empty]"),
            Some((col_min, col_max, row_min, row_max)) => {
                for row in row_min..=row_max {
                    for col in col_min..=col_max {
                        if self.0.contains(&Position((col, row))) {
                            f.write_str("\u{2588}")?;
                        } else {
                            f.write_str(" ")?;
                        }
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Vertical(i32),
    Horizontal(i32),
}

impl Instruction {
    fn parse(s: &str) -> anyhow::Result<Self> {
        static INST_PATTERN: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("^fold along (?P<xy>x|y)=(?P<val>0|[1-9][0-9]*)$").unwrap());
        let captures = INST_PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("‘{}’ is not a valid instruction", s))?;
        let xy = captures.name("xy").unwrap().as_str().chars().next().unwrap();
        let val_str = captures.name("val").unwrap().as_str();
        let val = val_str
            .parse::<i32>()
            .context(format!("{val_str} is not a valid 32-bit integer (in fold instruction)"))?;
        Ok(match xy {
            'x' => Instruction::Vertical(val),
            _ => Instruction::Horizontal(val),
        })
    }
}

#[derive(Debug)]
struct Data {
    initial_grid: Grid,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct ResultStringWrap(anyhow::Result<String>);
impl From<anyhow::Result<String>> for ResultStringWrap {
    fn from(src: anyhow::Result<String>) -> Self {
        Self(src)
    }
}
impl From<Result<String, std::io::Error>> for ResultStringWrap {
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}

impl FromIterator<ResultStringWrap> for anyhow::Result<Data> {
    fn from_iter<I: IntoIterator<Item = ResultStringWrap>>(iter: I) -> Self {
        let mut grid: AHashSet<Position> = AHashSet::new();
        let mut instructions: Vec<Instruction> = vec![];

        let mut loading_points = true;
        for ResultStringWrap(res) in iter.into_iter() {
            let line = res?;
            if loading_points {
                if line.is_empty() {
                    loading_points = false;
                } else {
                    let pos = Position::parse(&line)?;
                    grid.insert(pos);
                }
            } else {
                let inst = Instruction::parse(&line)?;
                instructions.push(inst);
            }
        }

        if grid.is_empty() {
            anyhow::bail!("No positions detected! At least one is required.");
        }
        if instructions.len() <= 1 {
            anyhow::bail!("At least two folding directions are required.")
        }

        Ok(Data {
            initial_grid: Grid(grid),
            instructions,
        })
    }
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    // Finally learned how to abort an iter at the first error and return it. Now we get:
    //     $ cargo r -q < /dev/urandom
    //     Error: Failed to parse puzzle input from stdin
    //
    //     Caused by:
    //         stream did not contain valid UTF-8
    let input = stdin
        .lock()
        .lines()
        .map(ResultStringWrap::from)
        .collect::<anyhow::Result<Data>>()
        .context("Failed to parse puzzle input from stdin")?;

    // Part 1: Apply the first instruction, and count the visible dots.
    let mut work_grid = input.initial_grid.clone();
    work_grid.apply_instruction(&input.instructions[0]);
    println!(
        "Part 1: Visible dots after one instruction: {}",
        work_grid.visible_dots()
    );

    // Part 2: Finish the instructions, and display the result.
    for i in &input.instructions[1..] {
        work_grid.apply_instruction(i);
    }
    println!("Part 2:\n{work_grid}");

    Ok(())
}
