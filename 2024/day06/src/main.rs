//! # Solution for Advent of Code 2024 Day 6: Guard Gallivant
//!
//! Ref: [Advent of Code 2024 Day 6](https://adventofcode.com/2024/day/6)
//!
//! This module implements a solution for finding paths through a grid-based map
//! where a guard walks in a specific pattern, turning right when hitting obstacles.
//! Part 1 counts visited spaces, while Part 2 finds possible obstacle placements
//! that create cycles in the guard's path.
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

/// A space on the map grid
#[derive(Debug, Copy, Clone, PartialEq)]
enum Space {
    /// An empty floor tile that can be walked on
    Floor,
    /// An obstacle that causes the guard to turn right
    Obstacle,
}

impl TryFrom<char> for Space {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Self::Floor),
            '#' => Ok(Self::Obstacle),
            _ => bail!("improper floor character found: {value}"),
        }
    }
}

/// The complete map grid and provides path-finding functionality
#[derive(Clone)]
struct Map {
    /// Grid storage using coordinates as keys and space types as values
    grid: AHashMap<(i64, i64), Space>,
}

impl Map {
    /// Simulates a guard walking through the map from a given starting position
    ///
    /// The guard follows these rules:
    /// - Starts walking upward
    /// - Turns right when hitting an obstacle
    /// - Continues until walking off the map or entering a cycle
    ///
    /// # Arguments
    /// * `start` - Starting coordinates (x, y)
    /// * `visit` - Closure called for each visited location
    ///
    /// # Returns
    /// * `true` if the guard walks off the map
    /// * `false` if the guard enters a cycle
    fn walk(&self, start: (i64, i64), mut visit: impl FnMut((i64, i64))) -> bool {
        // Define the four cardinal directions: up, right, down, left
        // Creating an infinite cycle for right-hand turns
        let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].into_iter().cycle();
        let mut current_direction = directions.next().expect("sequence should be infinite");
        let mut current_location = start;

        // Track visited locations with their directions to detect cycles
        let mut cache = AHashSet::new();

        loop {
            visit(current_location);
            loop {
                // Calculate the next position based on current direction
                let probe = (
                    current_location.0 + current_direction.0,
                    current_location.1 + current_direction.1,
                );
                let at_probe = self.grid.get(&probe);
                match at_probe {
                    // Walking off the map
                    None => {
                        return true;
                    }
                    // Empty space - move forward
                    Some(Space::Floor) => {
                        current_location = probe;
                        // If we've been here before with the same direction, it's a cycle
                        if !cache.insert((current_location, current_direction)) {
                            return false;
                        }
                        break;
                    }
                    // Hit an obstacle - turn right
                    Some(Space::Obstacle) => {
                        current_direction = directions.next().expect("sequence should be infinite");
                    }
                }
            }
        }
    }
}

/// The parsed input data for the puzzle
struct Input {
    /// The map grid with all spaces and obstacles
    map: Map,
    /// Starting coordinates for the guard (x, y)
    start: (i64, i64),
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut starting_location = None;
        let mut map = AHashMap::new();

        // Parse the grid, converting characters to spaces
        for (line_number, line) in s.lines().enumerate() {
            for (column_number, ch) in line.chars().enumerate() {
                let column_number = i64::try_from(column_number)?;
                let line_number = i64::try_from(line_number)?;
                // Special handling for start position (^)
                let space = if ch == '^' {
                    starting_location = Some((column_number, line_number));
                    Space::Floor
                } else {
                    Space::try_from(ch)?
                };
                map.insert((column_number, line_number), space);
            }
        }
        Ok(Self {
            map: Map { grid: map },
            start: starting_location.ok_or_else(|| anyhow!("missing starting location"))?,
        })
    }
}

/// Solves part 1: counts the total number of unique spaces visited by the guard
///
/// # Arguments
/// * `input` - The parsed puzzle input
///
/// # Returns
/// * The number of unique locations visited by the guard
fn part1(input: &Input) -> usize {
    let mut locations_visited = AHashSet::new();
    input.map.walk(input.start, |loc| {
        locations_visited.insert(loc);
    });
    locations_visited.len()
}

/// Solves part 2: counts the number of possible obstacle placements that create cycles
///
/// This function:
/// 1. Finds all locations visited in the original path
/// 2. Tests placing an obstacle at each visited location
/// 3. Counts how many placements result in a cyclic path
///
/// # Arguments
/// * `input` - The parsed puzzle input
///
/// # Returns
/// * The number of possible obstacle placements that create cycles
fn part2(input: &Input) -> usize {
    // First collect all locations visited in the original path (excluding start)
    let mut locations_visited = AHashSet::new();
    input.map.walk(input.start, |loc| {
        locations_visited.insert(loc);
    });
    // Remove start position since we can't block it
    locations_visited.remove(&input.start);

    // Try placing an obstacle at each visited location
    // If the walk produces a cycle (returns false), increment our count
    let mut altered = input.map.clone();
    let mut count = 0;
    for spot in &locations_visited {
        // Change the floorplan by adding a new obstacle.
        altered.grid.insert(*spot, Space::Obstacle);
        if !altered.walk(input.start, |_| {}) {
            count += 1;
        }
        // Restore the foor for the next go-around
        altered.grid.insert(*spot, Space::Floor);
    }

    count
}

/// Main function that reads input from stdin and solves both parts of the puzzle
///
/// # Errors
/// * IO errors when reading from stdin
/// * Parse errors if the input format is invalid
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
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 41);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 6);
    }
}
