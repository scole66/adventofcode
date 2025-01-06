//! # Solution for Advent of Code 2022 Day 23: Unstable Diffusion
//!
//! Ref: [Advent of Code 2022 Day 23](https://adventofcode.com/2022/day/23)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::{bail, Error, Result};
use std::fmt::Display;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point {
    col: i64,
    row: i64,
}
struct Input {
    map: AHashSet<Point>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = AHashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '.' => {}
                    '#' => {
                        map.insert(Point {
                            col: col as i64,
                            row: row as i64,
                        });
                    }
                    _ => {
                        bail!("Bad char in map");
                    }
                }
            }
        }
        Ok(Input { map })
    }
}

// round:
//   1. Each elf makes a proposal, from current list of potential proposals
//   2. Proposals are accepted/rejected
//   3. Accepted proposals are acted upon
//   4. Potential Proposal List is rotated
#[derive(Debug, Copy, Clone)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}
#[derive(Default)]
struct Elf {
    proposal: Option<Point>,
}
struct Limits {
    east_most: i64,
    west_most: i64,
    north_most: i64,
    south_most: i64,
}
impl Limits {
    fn area(&self) -> i64 {
        let width = self.east_most + 1 - self.west_most;
        let height = self.south_most + 1 - self.north_most;
        width * height
    }
}
struct ElfBall {
    choices: Vec<CardinalDirection>,
    elves: AHashMap<Point, Elf>,
}
impl From<Input> for ElfBall {
    fn from(value: Input) -> Self {
        ElfBall {
            choices: vec![
                CardinalDirection::North,
                CardinalDirection::South,
                CardinalDirection::West,
                CardinalDirection::East,
            ],
            elves: AHashMap::from_iter(value.map.into_iter().map(|idx| (idx, Elf::default()))),
        }
    }
}
impl Display for ElfBall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lims = self.limits();
        for row in lims.north_most..=lims.south_most {
            for col in lims.west_most..=lims.east_most {
                if self.elves.contains_key(&Point { row, col }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl ElfBall {
    fn has_neighbors(&self, origin: Point, dir: CardinalDirection) -> bool {
        let Point { col, row } = origin;
        [-1, 0, 1]
            .into_iter()
            .map(|delta| match dir {
                CardinalDirection::North => Point {
                    row: row - 1,
                    col: col + delta,
                },
                CardinalDirection::South => Point {
                    row: row + 1,
                    col: col + delta,
                },
                CardinalDirection::East => Point {
                    row: row + delta,
                    col: col + 1,
                },
                CardinalDirection::West => Point {
                    row: row + delta,
                    col: col - 1,
                },
            })
            .any(|pt| self.elves.contains_key(&pt))
    }

    fn new_destination(origin: Point, dir: CardinalDirection) -> Point {
        let Point { row, col } = origin;
        match dir {
            CardinalDirection::North => Point { row: row - 1, col },
            CardinalDirection::South => Point { row: row + 1, col },
            CardinalDirection::East => Point { row, col: col + 1 },
            CardinalDirection::West => Point { row, col: col - 1 },
        }
    }

    fn make_proposals(&mut self) {
        let proposals = self
            .elves
            .iter()
            .map(|(&elf_spot, _)| {
                (
                    elf_spot,
                    if self.choices.iter().all(|&dir| !self.has_neighbors(elf_spot, dir)) {
                        None
                    } else {
                        self.choices
                            .iter()
                            .filter(|&&choice| !self.has_neighbors(elf_spot, choice))
                            .map(|&dir| Self::new_destination(elf_spot, dir))
                            .next()
                    },
                )
            })
            .collect::<Vec<_>>();
        for (spot, proposal) in proposals {
            self.elves.get_mut(&spot).unwrap().proposal = proposal;
        }
    }

    fn reject_proposals(&mut self) {
        // See how many elves pick each destination
        let mut counts = AHashMap::new();
        for pt in self.elves.iter().filter_map(|(_, elf)| elf.proposal) {
            counts.entry(pt).and_modify(|counter| *counter += 1).or_insert(1);
        }
        // Reject any proposals that have 2 or more elves making the same suggestion
        for (_, elf) in self.elves.iter_mut() {
            if elf.proposal.map(|pt| counts[&pt] > 1).unwrap_or(false) {
                elf.proposal = None;
            }
        }
    }

    fn act_on_proposals(&mut self) -> usize {
        // Build a list of instructions, so that I don't iterate the map while I'm modifying the map.
        let do_this = self
            .elves
            .iter()
            .filter_map(|(&from, elf)| elf.proposal.map(|new_loc| (from, new_loc)))
            .collect::<Vec<_>>();
        let number_of_elves_to_move = do_this.len();
        for (from, to) in do_this {
            let elf = self.elves.remove(&from).unwrap();
            let prior = self.elves.insert(to, elf);
            assert!(prior.is_none());
        }
        number_of_elves_to_move
    }

    fn rotate_choices(&mut self) {
        self.choices.rotate_left(1);
    }

    fn round(&mut self) -> usize {
        self.make_proposals();
        self.reject_proposals();
        let elves_in_motion = self.act_on_proposals();
        self.rotate_choices();
        elves_in_motion
    }

    fn limits(&self) -> Limits {
        let mut row_min = i64::MAX;
        let mut row_max = i64::MIN;
        let mut col_min = i64::MAX;
        let mut col_max = i64::MIN;
        for pt in self.elves.keys() {
            let Point { row, col } = *pt;
            row_min = row_min.min(row);
            row_max = row_max.max(row);
            col_min = col_min.min(col);
            col_max = col_max.max(col);
        }
        Limits {
            east_most: col_max,
            west_most: col_min,
            north_most: row_min,
            south_most: row_max,
        }
    }

    fn empty_ground_tiles(&self) -> i64 {
        let total_area = self.limits().area();
        total_area - self.elves.len() as i64
    }
}

fn part1(input: &str) -> Result<i64> {
    let mut elfball = ElfBall::from(input.parse::<Input>()?);
    for _ in 0..10 {
        elfball.round();
    }
    Ok(elfball.empty_ground_tiles())
}

fn part2(input: &str) -> Result<usize> {
    let mut elfball = ElfBall::from(input.parse::<Input>()?);
    let mut round_number = 0;
    loop {
        round_number += 1;
        let elves_in_motion = elfball.round();
        if elves_in_motion == 0 {
            break;
        }
    }
    Ok(round_number)
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
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 110);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 20);
    }
}
