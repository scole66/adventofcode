//! # Solution for Advent of Code 2024 Day 12: Garden Groups
//!
//! Ref: [Advent of Code 2024 Day 12](https://adventofcode.com/2024/day/12)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    grid: AHashMap<(i64, i64), char>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = s
            .lines()
            .enumerate()
            .flat_map(move |(row, line)| {
                let row = i64::try_from(row)?;
                Ok::<_, Error>(line.chars().enumerate().map(move |(col, crop)| {
                    let col = i64::try_from(col)?;
                    Ok::<_, Error>(((row, col), crop))
                }))
            })
            .flatten()
            .collect::<Result<AHashMap<(i64, i64), char>, _>>()?;
        Ok(Input { grid })
    }
}
impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row = 0;
        let mut col = 0;
        loop {
            let crop = self.grid.get(&(row, col));
            match crop {
                Some(crop) => {
                    write!(f, "{crop}")?;
                    col += 1;
                }
                None => {
                    if col == 0 {
                        break;
                    }
                    write!(f, "\n")?;
                    col = 0;
                    row += 1;
                }
            }
        }
        Ok(())
    }
}

impl Input {
    fn all_crops(&self) -> impl Iterator<Item = char> {
        self.grid.values().copied().collect::<AHashSet<_>>().into_iter()
    }

    fn area(&self, crop: char) -> usize {
        self.grid.values().filter(|&in_grid| *in_grid == crop).count()
    }

    fn perimeter(&self, crop: char) -> usize {
        self.grid
            .iter()
            .filter(|item| (*item).1 == &crop)
            .map(|((row, col), crop)| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .filter(|(drow, dcol)| {
                        let probe_row = *row + *drow;
                        let probe_col = *col + *dcol;
                        self.grid.get(&(probe_row, probe_col)) != Some(&crop)
                    })
                    .count()
            })
            .sum()
    }

    fn price(&self, crop: char) -> usize {
        self.area(crop) * self.perimeter(crop)
    }
}

fn part1(input: &Input) -> usize {
    input.all_crops().map(|crop| input.price(crop)).sum()
}

fn part2(input: &Input) -> usize {
    todo!()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = 0; //part2(&input);
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

    static SAMPLE1: &str = indoc::indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};
    static SAMPLE2: &str = indoc::indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};
    static SAMPLE3: &str = indoc::indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test_case(SAMPLE1 => SAMPLE1.to_string(); "small sample")]
    #[test_case(SAMPLE2 => SAMPLE2.to_string(); "inclusion sample")]
    #[test_case(SAMPLE3 => SAMPLE3.to_string(); "big sample")]
    fn parse(inp: &str) -> String {
        let input = inp.parse::<Input>().unwrap();
        format!("{input}")
    }

    #[test_case("A", 'A' => 4; "one crop")]
    #[test_case("AB", 'A' => 4; "two neighbors - A")]
    #[test_case("AB", 'B' => 4; "two neighbors - B")]
    #[test_case(SAMPLE2, 'X' => 16; "inclusion - X")]
    #[test_case(SAMPLE2, 'O' => 36; "inclusion - O")]
    fn perimeter(input: &str, crop: char) -> usize {
        let grid = input.parse::<Input>().unwrap();
        grid.perimeter(crop)
    }

    #[test_case(SAMPLE2, 'X' => 4; "inclusion sample - X")]
    #[test_case(SAMPLE2, 'O' => 21; "inclusion sample - O")]
    fn area(input: &str, crop: char) -> usize {
        let grid = input.parse::<Input>().unwrap();
        grid.area(crop)
    }

    #[test_case(SAMPLE2 => AHashSet::from(['O', 'X']); "inclusion")]
    fn crops(input: &str) -> AHashSet<char> {
        let grid = input.parse::<Input>().unwrap();
        grid.all_crops().collect::<AHashSet<_>>()
    }

    #[test_case("A" => 4; "one crop")]
    #[test_case("AB" => 8; "two crop")]
    #[test_case(SAMPLE1 => 140; "small sample")]
    #[test_case(SAMPLE2 => 772; "inclusion sample")]
    #[test_case(SAMPLE3 => 1930; "bigger sample")]
    fn part1_sample(inp: &str) -> usize {
        part1(&inp.parse::<Input>().unwrap())
    }

    #[test]
    #[should_panic]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE1.parse::<Input>().unwrap()), 36);
    }
}
