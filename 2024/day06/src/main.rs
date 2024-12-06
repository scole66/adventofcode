//! # Solution for Advent of Code 2024 Day 6: Guard Gallivant
//!
//! Ref: [Advent of Code 2024 Day 6](https://adventofcode.com/2024/day/6)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Space {
    Floor,
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

#[derive(Clone)]
struct Map {
    grid: AHashMap<(i64, i64), Space>,
}

impl Map {
    fn walk(&self, start: (i64, i64), mut visit: impl FnMut((i64, i64))) -> bool {
        let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].into_iter().cycle();
        let mut current_direction = directions.next().expect("sequence should be infinite");
        let mut current_location = start;
        let mut cache = AHashSet::new();
        loop {
            visit(current_location);
            loop {
                let probe = (
                    current_location.0 + current_direction.0,
                    current_location.1 + current_direction.1,
                );
                let at_probe = self.grid.get(&probe);
                match at_probe {
                    None => {
                        return true;
                    }
                    Some(Space::Floor) => {
                        current_location = probe;
                        if !cache.insert((current_location, current_direction)) {
                            // we've been here before. exit, as we've detected a loop.
                            return false;
                        }
                        break;
                    }
                    Some(Space::Obstacle) => {
                        current_direction = directions.next().expect("sequence should be infinite");
                    }
                }
            }
        }
    }
}

struct Input {
    map: Map,
    start: (i64, i64),
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut starting_location = None;
        let mut map = AHashMap::new();
        for (line_number, line) in s.lines().enumerate() {
            for (column_number, ch) in line.chars().enumerate() {
                let column_number = i64::try_from(column_number)?;
                let line_number = i64::try_from(line_number)?;
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

fn part1(input: &Input) -> usize {
    let mut locations_visited = AHashSet::new();
    input.map.walk(input.start, |loc| {
        locations_visited.insert(loc);
    });
    locations_visited.len()
}

fn part2(input: &Input) -> usize {
    // the locations visited in the no blockage path (other than the start), are where we should try to place obstacles
    let mut locations_visited = AHashSet::new();
    input.map.walk(input.start, |loc| {
        locations_visited.insert(loc);
    });
    locations_visited.remove(&input.start);

    // Now try to put a blockage on each of those locations. If the walk produces a cycle (returns false), we bump our count.
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
