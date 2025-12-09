//! # Solution for Advent of Code 2025 Day 6: Trash Compactor
//!
//! Ref: [Advent of Code 2025 Day 6](https://adventofcode.com/2025/day/6)
//!
use ahash::{HashMap, HashMapExt};
use anyhow::{Context, Error, Result, anyhow, bail};
use std::cmp::max;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '*' => Ok(Operation::Multiply),
            '+' => Ok(Operation::Add),
            _ => Err(anyhow!(format!(
                "The character '{value}' cannot be transformed to an operation"
            ))),
        }
    }
}

#[derive(Debug)]
struct Chunk {
    chargrid: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
    operation: Operation,
}

impl Chunk {
    fn from_grid(source: &HashMap<(usize, usize), char>, column: usize, width: usize, height: usize) -> Result<Chunk> {
        let mut chunk_grid = HashMap::new();
        for row in 0..height - 1 {
            for col in column..column + width {
                chunk_grid.insert(
                    (row, col - column),
                    source
                        .get(&(row, col))
                        .copied()
                        .ok_or(anyhow!("Grid location ({row}, {col}) was missing"))?,
                );
            }
        }
        let operation = Operation::try_from(
            source
                .get(&(height - 1, column))
                .copied()
                .ok_or(anyhow!("Grid location ({}, {column}) was missing", height - 1))?,
        )?;

        Ok(Chunk {
            chargrid: chunk_grid,
            width,
            height: height - 1,
            operation,
        })
    }

    fn part1(&self) -> i64 {
        let number_iter = (0..self.height).map(|row| {
            (0..self.width)
                .map(|col| self.chargrid[&(row, col)])
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .expect("numbers should be reasonable")
        });
        match &self.operation {
            Operation::Add => number_iter.sum::<i64>(),
            Operation::Multiply => number_iter.product(),
        }
    }

    fn part2(&self) -> i64 {
        let number_iter = (0..self.width).map(|col| {
            (0..self.height)
                .map(|row| self.chargrid[&(row, col)])
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .expect("numbers should be reasonable")
        });
        match &self.operation {
            Operation::Add => number_iter.sum::<i64>(),
            Operation::Multiply => number_iter.product(),
        }
    }
}

#[derive(Debug)]
struct Input {
    chunks: Vec<Chunk>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let chargrid = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(column, ch)| ((row, column), ch)))
            .collect::<HashMap<(usize, usize), char>>();
        let (max_row, max_col) = chargrid
            .keys()
            .copied()
            .reduce(|(row_left, col_left), (row_right, col_right)| (max(row_left, row_right), max(col_left, col_right)))
            .ok_or(anyhow!("Grid had no items"))?;
        let width = max_col + 1;
        let height = max_row + 1;
        if (0..height).any(|row| !chargrid.contains_key(&(row, width - 1))) {
            bail!("Grid is not square (each row should have a column {})", width - 1);
        }

        let mut chunks = vec![];
        let mut start_column = 0;
        for col in 0..=width {
            if (0..height).all(|row| col == width || chargrid[&(row, col)] == ' ') {
                let chunk_width = col - start_column;
                if chunk_width > 0 {
                    chunks.push(
                        Chunk::from_grid(&chargrid, start_column, chunk_width, height).context(format!(
                            "Parsing the chunk from column {start_column} to column {}",
                            start_column + chunk_width
                        ))?,
                    );
                }
                start_column = col + 1;
            }
        }

        Ok(Input { chunks })
    }
}

fn part1(input: &Input) -> i64 {
    input.chunks.iter().map(Chunk::part1).sum()
}

fn part2(input: &Input) -> i64 {
    input.chunks.iter().map(Chunk::part2).sum()
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
    123 328  51 64 
     45 64  387 23 
      6 98  215 314
    *   +   *   +  
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 4_277_556);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 3_263_827);
    }
}
