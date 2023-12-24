//! # Solution for Advent of Code 2023 Day 11: Cosmic Expansion
//!
//! Ref: [Advent of Code 2023 Day 11](https://adventofcode.com/2023/day/11)
//!
use ahash::AHashSet;
use anyhow::{anyhow, Error, Result};
use combinations::Combination;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Location {
    row: i64,
    col: i64,
}

#[derive(Debug)]
struct StarMap {
    stars: AHashSet<Location>,
}
impl fmt::Display for StarMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.stars.iter().map(|Location { row: _, col }| *col).max().unwrap() + 1;
        let height = self.stars.iter().map(|Location { row, col: _ }| *row).max().unwrap() + 1;
        for row in 0..height {
            for col in 0..width {
                write!(
                    f,
                    "{}",
                    if self.stars.contains(&Location { row, col }) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl FromStr for StarMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StarMap {
            stars: s
                .lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.chars().enumerate().filter_map(move |(col, ch)| {
                        if ch == '#' {
                            match i64::try_from(row) {
                                Err(err) => Some(Err(Error::from(err))),
                                Ok(row) => match i64::try_from(col) {
                                    Err(err) => Some(Err(Error::from(err))),
                                    Ok(col) => Some(Ok(Location { row, col })),
                                },
                            }
                        } else if ch != '.' {
                            Some(Err(anyhow!("Bad starfield signifier: {ch}")))
                        } else {
                            None
                        }
                    })
                })
                .collect::<Result<AHashSet<_>, _>>()?,
        })
    }
}

impl StarMap {
    fn expand(&self, factor: i64) -> StarMap {
        fn loc_mrow(loc: &mut Location) -> &mut i64 {
            &mut loc.row
        }
        fn loc_mcol(loc: &mut Location) -> &mut i64 {
            &mut loc.col
        }
        fn loc_rrow(loc: &Location) -> &i64 {
            &loc.row
        }
        fn loc_rcol(loc: &Location) -> &i64 {
            &loc.col
        }
        fn adjust_by(
            stars: &mut [Location],
            mut get_mut: impl FnMut(&mut Location) -> &mut i64,
            get_ref: impl Fn(&Location) -> &i64,
            factor: i64,
        ) {
            stars.sort_unstable_by_key(|loc| *get_ref(loc));
            let mut adjustment = 0;
            let mut previous = None;
            for loc in stars.iter_mut() {
                match previous {
                    None => {
                        previous = Some(*get_ref(loc));
                    }
                    Some(prev) => {
                        let delta = *get_ref(loc) - prev;
                        if delta > 1 {
                            adjustment += (delta - 1) * (factor - 1);
                        }
                        previous = Some(*get_ref(loc));
                        let mref: &mut i64 = get_mut(loc);
                        *mref += adjustment;
                    }
                }
            }
        }

        let mut new_stars = self.stars.iter().copied().collect::<Vec<_>>();
        adjust_by(&mut new_stars, loc_mcol, loc_rcol, factor);
        adjust_by(&mut new_stars, loc_mrow, loc_rrow, factor);

        StarMap { stars: new_stars.into_iter().collect::<AHashSet<Location>>() }
    }

    fn work_the_puzzle(&self, factor: i64) -> i64 {
        let expanded = self.expand(factor);
        let stars = expanded.stars.iter().copied().collect::<Vec<_>>();
        Combination::new(&stars, 2)
            .map(|pair| (pair[0].row - pair[1].row).abs() + (pair[0].col - pair[1].col).abs())
            .sum::<i64>()
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    let unexpanded = input.parse::<StarMap>()?;

    println!("Part1: {}", unexpanded.work_the_puzzle(2));
    println!("Part2: {}", unexpanded.work_the_puzzle(1000000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn parse() {
        let stars = SAMPLE.parse::<StarMap>().unwrap();
        assert_eq!(stars.stars.len(), 9);
        assert!(stars.stars.contains(&Location { row: 0, col: 3 }));
        assert!(stars.stars.contains(&Location { row: 1, col: 7 }));
        assert!(stars.stars.contains(&Location { row: 2, col: 0 }));
        assert!(stars.stars.contains(&Location { row: 4, col: 6 }));
        assert!(stars.stars.contains(&Location { row: 5, col: 1 }));
        assert!(stars.stars.contains(&Location { row: 6, col: 9 }));
        assert!(stars.stars.contains(&Location { row: 8, col: 7 }));
        assert!(stars.stars.contains(&Location { row: 9, col: 0 }));
        assert!(stars.stars.contains(&Location { row: 9, col: 4 }));
    }

    #[test]
    fn expand() {
        let stars = SAMPLE.parse::<StarMap>().unwrap();
        let expanded = stars.expand(2);
        println!("{expanded}");
        assert_eq!(expanded.stars.len(), 9);
        assert!(expanded.stars.contains(&Location { row: 0, col: 4 }));
        assert!(expanded.stars.contains(&Location { row: 1, col: 9 }));
        assert!(expanded.stars.contains(&Location { row: 2, col: 0 }));
        assert!(expanded.stars.contains(&Location { row: 5, col: 8 }));
        assert!(expanded.stars.contains(&Location { row: 6, col: 1 }));
        assert!(expanded.stars.contains(&Location { row: 7, col: 12 }));
        assert!(expanded.stars.contains(&Location { row: 10, col: 9 }));
        assert!(expanded.stars.contains(&Location { row: 11, col: 0 }));
        assert!(expanded.stars.contains(&Location { row: 11, col: 5 }));
    }

    #[test_case(2 => 374)]
    #[test_case(10 => 1030)]
    #[test_case(100 => 8410)]
    fn sample(factor: i64) -> i64 {
        SAMPLE.parse::<StarMap>().unwrap().work_the_puzzle(factor)
    }
}
