//! # Solution for Advent of Code 2023 Day 3: Gear Ratios
//!
//! Ref: [Advent of Code 2023 Day 3](https://adventofcode.com/2023/day/3)
//!
use ahash::AHashMap;
use anyhow::{Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
    row: isize,
    col: isize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Item {
    Number { value: u32, digit_len: u32 },
    Symbol(char),
}

impl Item {
    fn get_part_number(&self, grid: &Grid, position: &Position) -> Option<u32> {
        match self {
            Self::Symbol(_) => None,
            Self::Number { value, digit_len } => ((position.col - 1..=position.col + *digit_len as isize).any(|col| {
                grid.has_symbol_at(&Position {
                    row: position.row - 1,
                    col,
                }) || grid.has_symbol_at(&Position {
                    row: position.row + 1,
                    col,
                })
            }) || grid.has_symbol_at(&Position {
                row: position.row,
                col: position.col - 1,
            }) || grid.has_symbol_at(&Position {
                row: position.row,
                col: position.col + *digit_len as isize,
            }))
            .then_some(*value),
        }
    }
    fn get_gear_ratio(&self, grid: &Grid, position: &Position) -> Option<u32> {
        const DELTAS: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        match self {
            Self::Symbol('*') => {
                let mut nearby = DELTAS
                    .iter()
                    .map(|(dcol, drow)| Position {
                        row: position.row + *drow,
                        col: position.col + *dcol,
                    })
                    .filter_map(|p| grid.number_at(&p))
                    .collect::<Vec<_>>();
                nearby.sort();
                nearby.dedup();
                if nearby.len() == 2 {
                    Some(nearby[0] * nearby[1])
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Grid {
    data: AHashMap<Position, Item>,
}

impl Grid {
    fn has_symbol_at(&self, position: &Position) -> bool {
        matches!(self.data.get(position), Some(Item::Symbol(_)))
    }
    fn number_at(&self, position: &Position) -> Option<u32> {
        let probe = self.data.get(position);
        if let Some(Item::Number { value, digit_len: _ }) = probe {
            return Some(*value);
        } else if let Some(Item::Symbol(_)) = probe {
            return None;
        }
        let mut col = position.col - 1;
        while col >= 0 {
            let pi = self.data.get(&Position { col, row: position.row });
            if let Some(Item::Number { value, digit_len }) = pi {
                if col + *digit_len as isize > position.col {
                    return Some(*value);
                }
            }
            if pi.is_some() {
                return None;
            }
            col -= 1;
        }
        None
    }
}

#[derive(Debug)]
struct Row(Vec<(u32, Item)>);
impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static ROW_ITEM_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"([1-9][0-9]*)|(.)").unwrap());
        Ok(Row(ROW_ITEM_PATTERN
            .captures_iter(s)
            .filter(|cap| cap.get(2).map(|m| m.as_str()).unwrap_or("x") != ".")
            .map(|cap| {
                cap.get(1)
                    .map(|m| {
                        let range = m.range();
                        let value = m.as_str().parse::<u32>()?;
                        let column = u32::try_from(range.start)?;
                        let digit_len = u32::try_from(range.end - range.start)?;
                        let item = Item::Number { value, digit_len };
                        Ok::<_, Self::Err>((column, item))
                    })
                    .unwrap_or_else(|| {
                        let m = cap.get(2).expect("either group 1 or group 2 should exist");
                        assert_eq!(m.as_str().len(), 1);
                        let ch = m.as_str().chars().next().unwrap();
                        let range = m.range();
                        let column = u32::try_from(range.start)?;
                        Ok::<_, Self::Err>((column, Item::Symbol(ch)))
                    })
            })
            .collect::<Result<Vec<_>>>()?))
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            data: {
                s.lines()
                    .enumerate()
                    .map(|(row, line)| {
                        let Row(things) = line.parse::<Row>()?;
                        Ok((row, things))
                    })
                    .map(|res: Result<_, Self::Err>| {
                        let (row, things) = res?;
                        things
                            .into_iter()
                            .map(|(col, item)| {
                                let row = isize::try_from(row)?;
                                let col = isize::try_from(col)?;
                                Ok((Position { row, col }, item))
                            })
                            .collect::<Result<Vec<(Position, Item)>>>()
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<AHashMap<_, _>>()
            },
        })
    }
}

fn part1(input: &str) -> Result<u32> {
    let grid = input.parse::<Grid>()?;
    Ok(grid
        .data
        .keys()
        .filter_map(|key| {
            let item = grid.data.get(key).unwrap();
            item.get_part_number(&grid, key)
        })
        .sum::<u32>())
}

fn part2(input: &str) -> Result<u32> {
    let grid = input.parse::<Grid>()?;
    Ok(grid
        .data
        .keys()
        .filter_map(|key| {
            let item = grid.data.get(key).unwrap();
            item.get_gear_ratio(&grid, key)
        })
        .sum::<u32>())
}

fn main() -> Result<()> {
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
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 4361);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 467835);
    }
}
