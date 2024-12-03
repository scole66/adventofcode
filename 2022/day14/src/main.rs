//! # Solution for Advent of Code 2022 Day 14: Regolith Reservoir
//!
//! Ref: [Advent of Code 2022 Day 14](https://adventofcode.com/2022/day/14)
//!
use ahash::AHashMap;
use anyhow::Context;
use itertools::{chain, Itertools};
use std::fmt::Display;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Piece {
    Rock,
    Air,
    Sand,
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::Rock => '#',
                Piece::Air => '.',
                Piece::Sand => 'o',
            }
        )
    }
}

#[derive(Debug)]
struct InitialBoard {
    data: Vec<Piece>,
    width: usize,
    height: usize,
    offset: isize,
}

impl Display for InitialBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let val = self.data[col + row * self.width];
                write!(f, "{val}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    col: isize,
    row: isize,
}
impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col_str, row_str) = s
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("{s}: not a valid point (no comma)"))?;
        Ok(Point {
            col: col_str.parse::<isize>().context("Parsing column value in point")?,
            row: row_str.parse::<isize>().context("Parsing row value in point")?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Squiggle(Vec<Point>);
impl FromStr for Squiggle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(" -> ")
            .map(|pt| pt.parse::<Point>())
            .collect::<anyhow::Result<Vec<_>>>()
            .and_then(|items| {
                if items.len() < 2 {
                    Err(anyhow::anyhow!("Not enough points to form a squiggle"))
                } else if items
                    .iter()
                    .tuple_windows()
                    .any(|(left, right)| left.col != right.col && left.row != right.row)
                {
                    Err(anyhow::anyhow!(
                        "Segments in a squiggle must be exactly horizontal or vertical"
                    ))
                } else {
                    Ok(Squiggle(items))
                }
            })
    }
}

impl Squiggle {
    fn max_col(&self) -> isize {
        self.0
            .iter()
            .map(|pt| pt.col)
            .max()
            .expect("Squiggles have at least two elements")
    }
    fn min_col(&self) -> isize {
        self.0
            .iter()
            .map(|pt| pt.col)
            .min()
            .expect("Squggles have at least two elements")
    }
    fn max_row(&self) -> isize {
        self.0
            .iter()
            .map(|pt| pt.row)
            .max()
            .expect("Squiggles have at least two elements")
    }
}

impl FromStr for InitialBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let squiggles = s
            .lines()
            .map(|line| line.parse::<Squiggle>())
            .collect::<anyhow::Result<Vec<_>>>()?;

        let max_col = chain!([500], squiggles.iter().map(|s| s.max_col()))
            .max()
            .expect("Chain guarantees at least one element");
        let min_col = chain!([500], squiggles.iter().map(|s| s.min_col()))
            .min()
            .expect("Chain guarantes at least one element");
        let max_row = chain!([0], squiggles.iter().map(|s| s.max_row()))
            .max()
            .expect("Chain guarantees at least one element");

        let width = usize::try_from(1 + max_col - min_col).expect("max >= min");
        let height = usize::try_from(max_row + 1)?;
        let horiz_offset = min_col;

        let mut data = vec![Piece::Air; width * height];

        for s in squiggles {
            for (start, end) in s.0.iter().tuple_windows() {
                let (sx, ex) = if start.col > end.col {
                    (end.col, start.col)
                } else {
                    (start.col, end.col)
                };
                let (sy, ey) = if start.row > end.row {
                    (end.row, start.row)
                } else {
                    (start.row, end.row)
                };
                for xpos in sx..=ex {
                    for ypos in sy..=ey {
                        let loc_idx = (xpos - horiz_offset + ypos * width as isize) as usize;
                        data[loc_idx] = Piece::Rock;
                    }
                }
            }
        }

        Ok(InitialBoard {
            data,
            width,
            height,
            offset: horiz_offset,
        })
    }
}

struct GameData {
    data: AHashMap<(isize, isize), Piece>,
    floor_row: isize,
}

impl From<InitialBoard> for GameData {
    fn from(ib: InitialBoard) -> Self {
        let mut data = AHashMap::new();
        for (idx, item) in ib.data.iter().enumerate() {
            if item != &Piece::Air {
                let (col, row) = ((idx % ib.width) as isize + ib.offset, (idx / ib.width) as isize);
                data.insert((col, row), *item);
            }
        }
        let floor_row = ib.height as isize + 1;
        GameData { data, floor_row }
    }
}

#[derive(PartialEq, Eq)]
enum DropResult {
    SandStopped,
    SandFellForever,
    NoRoomAtAll,
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum ItFalls {
    ToFloor,
    Forever,
}

impl GameData {
    fn loc_is_empty(&self, location: &(isize, isize), falls: ItFalls) -> bool {
        !self.data.contains_key(location) && (falls == ItFalls::Forever || location.1 < self.floor_row)
    }
    fn next_step(&self, location: (isize, isize), falls: ItFalls) -> Option<(isize, isize)> {
        [0, -1, 1]
            .iter()
            .map(|dx| (location.0 + dx, location.1 + 1))
            .find(|point| self.loc_is_empty(point, falls))
    }
    fn below_all(&self, location: (isize, isize)) -> bool {
        self.data.keys().all(|&(_, row)| row < location.1)
    }
    fn drop_grain(&mut self, falls: ItFalls) -> DropResult {
        static START: (isize, isize) = (500, 0);
        let mut location = START;
        loop {
            match self.next_step(location, falls) {
                None => {
                    return match self.data.insert(location, Piece::Sand) {
                        None => DropResult::SandStopped,
                        Some(_) => DropResult::NoRoomAtAll,
                    };
                }
                Some(new_location) => {
                    if falls == ItFalls::Forever && self.below_all(location) {
                        return DropResult::SandFellForever;
                    }
                    location = new_location;
                }
            }
        }
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let initial = input.parse::<InitialBoard>()?;
    let mut game = GameData::from(initial);
    let mut grains_dropped = 0;
    let grains = loop {
        if game.drop_grain(ItFalls::Forever) == DropResult::SandFellForever {
            break grains_dropped;
        }
        grains_dropped += 1;
    };
    Ok(grains)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let initial = input.parse::<InitialBoard>()?;
    let mut game = GameData::from(initial);
    let mut grains_dropped = 0;
    let grains = loop {
        if game.drop_grain(ItFalls::ToFloor) == DropResult::NoRoomAtAll {
            break grains_dropped;
        }
        grains_dropped += 1;
    };
    Ok(grains)
}

fn main() -> anyhow::Result<()> {
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
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 24);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 93);
    }

    #[test_case("498,4" => Point{col: 498, row: 4})]
    fn parse_point(s: &str) -> Point {
        s.parse::<Point>().unwrap()
    }

    #[test_case("498,4 -> 498,6 -> 496,6" => Squiggle(vec![Point{col:498,row:4}, Point{col:498,row:6}, Point{col:496,row:6}]))]
    fn parse_squiggle(s: &str) -> Squiggle {
        s.parse::<Squiggle>().unwrap()
    }

    #[test]
    fn parse_board() {
        let board = SAMPLE.parse::<InitialBoard>().unwrap();
        let repr = format!("{board}");
        let expected = indoc::indoc! {"
            ..........
            ..........
            ..........
            ..........
            ....#...##
            ....#...#.
            ..###...#.
            ........#.
            ........#.
            #########.
        "};
        assert_eq!(repr, expected);
    }
}
