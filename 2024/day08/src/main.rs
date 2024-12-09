//! # Solution for Advent of Code 2024 Day 8: Resonant Collinearity
//!
//! Ref: [Advent of Code 2024 Day 8](https://adventofcode.com/2024/day/8)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::{bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GridElement {
    Empty,
    Antenna(char),
}

#[derive(Debug, Clone)]
struct Input {
    grid: AHashMap<(i64, i64), GridElement>,
    width: i64,
    height: i64,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(column, ch)| {
                    let location = (i64::try_from(row)?, i64::try_from(column)?);
                    match ch {
                        '.' => Ok((location, GridElement::Empty)),
                        '0'..='9' | 'a'..='z' | 'A'..='Z' => Ok((location, GridElement::Antenna(ch))),
                        _ => bail!("Improper Antenna Identifier {ch}"),
                    }
                })
            })
            .collect::<Result<AHashMap<_, _>, _>>()?;
        let (height, width) = grid.keys().fold((0, 0), |acc, (row, col)| {
            let w = acc.1.max(*col + 1);
            let h = acc.0.max(*row + 1);
            (h, w)
        });
        Ok(Input { grid, width, height })
    }
}

impl Input {
    fn frequencies(&self) -> AHashSet<char> {
        self.grid
            .values()
            .filter_map(|element| match element {
                GridElement::Empty => None,
                GridElement::Antenna(ch) => Some(*ch),
            })
            .collect::<AHashSet<_>>()
    }

    fn locations_of_frequency(&self, frequency: char) -> Box<[(i64, i64)]> {
        self.grid
            .iter()
            .filter_map(|(loc, val)| {
                if matches!(val, &GridElement::Antenna(f) if f == frequency) {
                    Some(*loc)
                } else {
                    None
                }
            })
            .collect::<Box<[(i64, i64)]>>()
    }

    fn locations_of_antinodes_for_frequency(&self, frequency: char) -> AHashSet<(i64, i64)> {
        let locs = self.locations_of_frequency(frequency);
        locs.iter()
            .flat_map(|left| {
                locs.iter().filter_map(move |right| {
                    if left == right {
                        None
                    } else {
                        let delta = (right.0 - left.0, right.1 - left.1);
                        let anti = (right.0 + delta.0, right.1 + delta.1);
                        if anti.0 >= 0 && anti.0 < self.height && anti.1 >= 0 && anti.1 < self.width {
                            Some(anti)
                        } else {
                            None
                        }
                    }
                })
            })
            .collect::<AHashSet<_>>()
    }

    fn locations_of_p2_antinodes_for_frequency(&self, frequency: char) -> AHashSet<(i64, i64)> {
        let locs = self.locations_of_frequency(frequency);
        let mut antinodes = AHashSet::new();
        for left in &locs {
            for right in &locs {
                if left != right {
                    let delta = (right.0 - left.0, right.1 - left.1);
                    let mut mul = 0;
                    loop {
                        let anti = (right.0 + delta.0 * mul, right.1 + delta.1 * mul);
                        if anti.0 < 0 || anti.0 >= self.height || anti.1 < 0 || anti.1 >= self.width {
                            break;
                        }
                        antinodes.insert(anti);
                        mul += 1;
                    }
                }
            }
        }
        antinodes
    }
}

fn part1(input: &Input) -> usize {
    let frequencies = input.frequencies();
    let mut total_antinodes = AHashSet::new();
    for loc in frequencies
        .iter()
        .flat_map(|&freq| input.locations_of_antinodes_for_frequency(freq).into_iter())
    {
        total_antinodes.insert(loc);
    }

    total_antinodes.len()
}

fn part2(input: &Input) -> usize {
    let frequencies = input.frequencies();
    let mut total_antinodes = AHashSet::new();
    for loc in frequencies
        .iter()
        .flat_map(|&freq| input.locations_of_p2_antinodes_for_frequency(freq).into_iter())
    {
        total_antinodes.insert(loc);
    }

    total_antinodes.len()
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
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn locations_of_frequency() {
        let input = SAMPLE.parse::<Input>().unwrap();
        let locs = input.locations_of_frequency('A');
        assert_eq!(locs.len(), 3);
        assert!(locs.contains(&(5, 6)));
        assert!(locs.contains(&(8, 8)));
        assert!(locs.contains(&(9, 9)));
    }

    #[test]
    fn locations_of_antinodes_for_frequency() {
        let input = SAMPLE.parse::<Input>().unwrap();
        let locs = input.locations_of_antinodes_for_frequency('A');
        assert_eq!(locs.len(), 5);
        assert!(locs.contains(&(10, 10)));
        assert!(locs.contains(&(7, 7)));
        assert!(locs.contains(&(1, 3)));
        assert!(locs.contains(&(2, 4)));
        assert!(locs.contains(&(11, 10)));
    }
    #[test]
    fn locations_of_p2_antinodes_for_frequency() {
        let input = SAMPLE.parse::<Input>().unwrap();
        let locs = input.locations_of_p2_antinodes_for_frequency('A');
        println!("{locs:?}");
        assert_eq!(locs.len(), 16);
        assert!(locs.contains(&(1, 3)));
        assert!(locs.contains(&(2, 4)));
        assert!(locs.contains(&(11, 10)));
        assert!(locs.contains(&(5, 6)));
        assert!(locs.contains(&(0, 0)));
        assert!(locs.contains(&(1, 1)));
        assert!(locs.contains(&(2, 2)));
        assert!(locs.contains(&(3, 3)));
        assert!(locs.contains(&(4, 4)));
        assert!(locs.contains(&(5, 5)));
        assert!(locs.contains(&(6, 6)));
        assert!(locs.contains(&(7, 7)));
        assert!(locs.contains(&(8, 8)));
        assert!(locs.contains(&(9, 9)));
        assert!(locs.contains(&(10, 10)));
        assert!(locs.contains(&(11, 11)));
    }

    #[test]
    fn part1_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 34);
    }
}
