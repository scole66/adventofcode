//! # Solution for Advent of Code 2024 Day 8: Resonant Collinearity
//!
//! Ref: [Advent of Code 2024 Day 8](https://adventofcode.com/2024/day/8)
//!
//! This module solves a puzzle involving antennas placed on a grid and their resonance patterns.
//! Part 1 finds antinode locations based on pairs of antennas, while Part 2 extends this to
//! find all possible antinode locations along resonance lines.

use ahash::{AHashMap, AHashSet};
use anyhow::{bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

/// Represents an element in the grid - either empty space or an antenna
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GridElement {
    /// Empty space in the grid
    Empty,
    /// An antenna with its frequency identifier
    Antenna(char),
}

/// Contains the raw parsed input grid
#[derive(Debug, Clone)]
struct Input {
    /// Map of coordinates to grid elements
    grid: AHashMap<(i64, i64), GridElement>,
}

/// Contains processed puzzle data optimized for solving
#[derive(Clone)]
struct PuzzleData {
    /// Width of the grid
    width: i64,
    /// Height of the grid
    height: i64,
    /// Map of frequency identifiers to their antenna locations
    antennas: AHashMap<char, Vec<(i64, i64)>>,
}

impl FromStr for Input {
    type Err = Error;

    /// Parses the input grid from a string representation
    ///
    /// # Arguments
    ///
    /// * `s` - String containing the grid layout
    ///
    /// # Returns
    ///
    /// * `Ok(Input)` - Successfully parsed input
    /// * `Err` - If the input format is invalid
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
        Ok(Input { grid })
    }
}

impl From<Input> for PuzzleData {
    /// Converts raw input into optimized puzzle data structure
    ///
    /// Calculates grid dimensions and groups antenna locations by frequency
    fn from(value: Input) -> Self {
        let mut max_row = -1;
        let mut max_col = -1;
        let mut antennas: AHashMap<char, Vec<(i64, i64)>> = AHashMap::new();
        for (location, element) in value.grid {
            max_row = max_row.max(location.0);
            max_col = max_col.max(location.1);
            match element {
                GridElement::Empty => {}
                GridElement::Antenna(freq) => match antennas.get_mut(&freq) {
                    Some(fvec) => {
                        fvec.push(location);
                    }
                    None => {
                        antennas.insert(freq, vec![location]);
                    }
                },
            }
        }
        PuzzleData {
            width: max_col + 1,
            height: max_row + 1,
            antennas,
        }
    }
}

impl PuzzleData {
    /// Returns a set of all unique antenna frequencies in the grid
    fn frequencies(&self) -> AHashSet<char> {
        self.antennas.keys().copied().collect::<AHashSet<_>>()
    }

    /// Returns all locations of antennas with a specific frequency
    fn locations_of_frequency(&self, frequency: char) -> &Vec<(i64, i64)> {
        &self.antennas[&frequency]
    }

    /// Finds all antinode locations for a given frequency in part 1
    ///
    /// Antinodes are locations that complete a resonance pattern between two antennas
    /// of the same frequency, extending one step beyond their line.
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

    /// Finds all antinode locations for a given frequency in part 2
    ///
    /// Similar to part 1, but continues the resonance pattern indefinitely until
    /// reaching the grid boundary.
    fn locations_of_p2_antinodes_for_frequency(&self, frequency: char) -> AHashSet<(i64, i64)> {
        let locs = self.locations_of_frequency(frequency);
        let mut antinodes = AHashSet::new();
        for left in locs {
            for right in locs {
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

/// Solves part 1: counts total unique antinode locations across all frequencies
fn part1(input: &PuzzleData) -> usize {
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

/// Solves part 2: counts total unique extended antinode locations
fn part2(input: &PuzzleData) -> usize {
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

/// Main function that reads input from stdin and solves both parts of the puzzle
///
/// # Errors
///
/// Returns an error if:
/// * Failed to read from stdin
/// * Failed to parse the input
fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let data = PuzzleData::from(input);
    let part1 = part1(&data);
    let part2 = part2(&data);
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
        let input = PuzzleData::from(input);
        let locs = input.locations_of_frequency('A');
        assert_eq!(locs.len(), 3);
        assert!(locs.contains(&(5, 6)));
        assert!(locs.contains(&(8, 8)));
        assert!(locs.contains(&(9, 9)));
    }

    #[test]
    fn locations_of_antinodes_for_frequency() {
        let input = SAMPLE.parse::<Input>().unwrap();
        let input = PuzzleData::from(input);
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
        let input = PuzzleData::from(input);
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
        let input = PuzzleData::from(input);
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&PuzzleData::from(SAMPLE.parse::<Input>().unwrap())), 34);
    }
}
