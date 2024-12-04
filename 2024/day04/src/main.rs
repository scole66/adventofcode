//! # Solution for Advent of Code 2024 Day 4: Ceres Search
//!
//! Ref: [Advent of Code 2024 Day 4](https://adventofcode.com/2024/day/4)
//!
use ahash::AHashMap;
use anyhow::{Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    puzzle: AHashMap<(isize, isize), char>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut puzzle = AHashMap::new();
        for (row_index, row) in s.lines().enumerate() {
            for (col_index, letter) in row.chars().enumerate() {
                let col_index = isize::try_from(col_index)?;
                let row_index = isize::try_from(row_index)?;
                puzzle.insert((col_index, row_index), letter);
            }
        }
        Ok(Input { puzzle })
    }
}

const DELTAS_NW: [(isize, isize); 3] = [(-1, -1), (-2, -2), (-3, -3)];
const DELTAS_N: [(isize, isize); 3] = [(0, -1), (0, -2), (0, -3)];
const DELTAS_NE: [(isize, isize); 3] = [(1, -1), (2, -2), (3, -3)];
const DELTAS_E: [(isize, isize); 3] = [(1, 0), (2, 0), (3, 0)];
const DELTAS_SE: [(isize, isize); 3] = [(1, 1), (2, 2), (3, 3)];
const DELTAS_S: [(isize, isize); 3] = [(0, 1), (0, 2), (0, 3)];
const DELTAS_SW: [(isize, isize); 3] = [(-1, 1), (-2, 2), (-3, 3)];
const DELTAS_W: [(isize, isize); 3] = [(-1, 0), (-2, 0), (-3, 0)];
const DELTAS: [[(isize, isize); 3]; 8] = [
    DELTAS_NW, DELTAS_N, DELTAS_NE, DELTAS_E, DELTAS_SE, DELTAS_S, DELTAS_SW, DELTAS_W,
];
const MATCH: [char; 3] = ['M', 'A', 'S'];

impl Input {
    fn matches(&self, location: (isize, isize), deltas: &[(isize, isize)]) -> bool {
        deltas.iter().zip(MATCH.iter()).all(|(d, ch)| {
            let probe = (location.0 + d.0, location.1 + d.1);
            self.puzzle
                .get(&probe)
                .map(|in_puzzle| ch == in_puzzle)
                .unwrap_or(false)
        })
    }

    fn all_matches_at(&self, location: (isize, isize)) -> usize {
        DELTAS.iter().filter(|path| self.matches(location, *path)).count()
    }

    fn all_letters(&self, letter: char) -> Vec<(isize, isize)> {
        self.puzzle
            .iter()
            .filter_map(|(key, val)| if *val == letter { Some(*key) } else { None })
            .collect::<Vec<_>>()
    }

    fn all_exes(&self) -> Vec<(isize, isize)> {
        self.all_letters('X')
    }

    fn match_count(&self) -> usize {
        self.all_exes()
            .iter()
            .map(|loc| self.all_matches_at(*loc))
            .sum::<usize>()
    }

    fn all_ayes(&self) -> Vec<(isize, isize)> {
        self.all_letters('A')
    }

    fn cross_at(&self, location: (isize, isize)) -> bool {
        for n in 0..=3 {
            if ['M', 'M', 'S', 'S']
                .iter()
                .cycle()
                .skip(n)
                .take(4)
                .zip([(-1, -1), (1, -1), (1, 1), (-1, 1)].iter())
                .all(|(letter, delta)| {
                    let probe = (location.0 + delta.0, location.1 + delta.1);
                    self.puzzle
                        .get(&probe)
                        .map(|in_puzzle| letter == in_puzzle)
                        .unwrap_or(false)
                })
            {
                return true;
            }
        }
        false
    }

    fn count_crosses(&self) -> usize {
        self.all_ayes().into_iter().filter(|loc| self.cross_at(*loc)).count()
    }
}

fn part1(input: &Input) -> usize {
    input.match_count()
}

fn part2(input: &Input) -> usize {
    input.count_crosses()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 18);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 9);
    }
}
