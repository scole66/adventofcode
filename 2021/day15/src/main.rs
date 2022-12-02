//! # Solution for Advent of Code 2021 Day 15
//!
//! Ref: [Advent of Code 2021 Day 15](https://adventofcode.com/2021/day/15)
//!

use ahash::AHashMap;
use anyhow::{self, Context};
use priority_queue::PriorityQueue;
use std::io::{self, BufRead};

/// A newtype wrapping a u8 and allowing only the values 0-9 plus [u8::MAX] for the "infinity" value off the edges of the map.
///
/// Conversions are provided for [char] and [u8].
#[derive(Debug, Copy, Clone)]
struct RiskLevel(u8);
impl TryFrom<u8> for RiskLevel {
    type Error = anyhow::Error;
    fn try_from(src: u8) -> anyhow::Result<Self> {
        match src {
            0..=9 | u8::MAX => Ok(Self(src)),
            _ => Err(anyhow::anyhow!("Risk Level too high")),
        }
    }
}
impl TryFrom<char> for RiskLevel {
    type Error = anyhow::Error;
    fn try_from(src: char) -> anyhow::Result<Self> {
        match src {
            '0'..='9' => RiskLevel::try_from(src as u8 - 0x30),
            _ => Err(anyhow::anyhow!("{} is not a risk level", src)),
        }
    }
}
impl TryFrom<isize> for RiskLevel {
    type Error = anyhow::Error;
    fn try_from(src: isize) -> anyhow::Result<Self> {
        let byte = u8::try_from(src)?;
        Self::try_from(byte)
    }
}

/// A NewType wrapping an `anyhow::Result<String>`
///
/// This is really nothing more than a new type created so that we can implement what would otherwise be
/// `FromIterator<anyhow::Result<String>> for anyhow::Result<Data>`.
#[derive(Debug)]
struct ResultStringWrap(anyhow::Result<String>);
impl From<anyhow::Result<String>> for ResultStringWrap {
    /// Converts an `anyhow::Result<String>` into a `ResultStringWrap`
    fn from(src: anyhow::Result<String>) -> Self {
        Self(src)
    }
}
impl From<Result<String, std::io::Error>> for ResultStringWrap {
    /// Converts a `Result<String, std::io::Error>` into a `ResultStringWrap`
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}

/// Simple newtype struct to hold positions.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position((isize, isize));
impl Position {
    fn neighbors(&self) -> [Position; 4] {
        let Position((me_x, me_y)) = self;
        [
            Position((me_x + 1, *me_y)),
            Position((me_x - 1, *me_y)),
            Position((*me_x, me_y + 1)),
            Position((*me_x, me_y - 1)),
        ]
    }
}

/// Model of a Cavern
///
/// This is really just a 2-d array of risk levels. A [FromIterator] is provied for `anyhow::Result<Cavern>` from
/// [ResultStringWrap]. Indexing into this array with a [Position] will give you either the risk level from the array or
/// [u8::MAX].
#[derive(Debug)]
struct Cavern {
    map: Vec<Vec<RiskLevel>>,
    expansion_factor: i32,
}
impl FromIterator<ResultStringWrap> for anyhow::Result<Cavern> {
    fn from_iter<I: IntoIterator<Item = ResultStringWrap>>(iter: I) -> Self {
        let mut rows = Vec::<Vec<RiskLevel>>::new();
        for ResultStringWrap(res) in iter.into_iter() {
            let line = res?;
            let mut this_row = Vec::<RiskLevel>::new();
            for ch in line.chars() {
                let rl = RiskLevel::try_from(ch)?;
                this_row.push(rl);
            }
            rows.push(this_row);
        }
        let row_count = rows.len();
        if row_count == 0 {
            anyhow::bail!("Must have at least one row");
        }
        let line_one_length = rows[0].len();
        if !rows.iter().all(|r| r.len() == line_one_length) {
            anyhow::bail!("All rows must be the same length");
        }
        Ok(Cavern { map: rows, expansion_factor: 1 })
    }
}

impl Cavern {
    fn get(&self, index: Position) -> anyhow::Result<RiskLevel> {
        let (idx_col, idx_row) = index.0;

        if idx_col < 0 || idx_row < 0 {
            return Ok(RiskLevel(u8::MAX));
        }
        let column_count: isize = self.map[0].len().try_into().unwrap();
        let row_count: isize = self.map.len().try_into().unwrap();
        if idx_col >= column_count * isize::try_from(self.expansion_factor)?
            || idx_row >= row_count * isize::try_from(self.expansion_factor)?
        {
            Ok(RiskLevel(u8::MAX))
        } else {
            let row_block = idx_row / row_count;
            let col_block = idx_col / column_count;
            let delta = row_block + col_block;
            let value: RiskLevel = self.map[usize::try_from(idx_row % row_count).unwrap()]
                [usize::try_from(idx_col % column_count).unwrap()];
            let RiskLevel(rl) = value;
            let new_rl = RiskLevel::try_from((rl as isize + delta - 1) % 9 + 1).unwrap();
            Ok(new_rl)
        }
    }

    fn find_path(&self, start: Position, goal: Position) -> anyhow::Result<Option<(Vec<Position>, isize)>> {
        // Shamelessly stolen from wikipedia: https://en.wikipedia.org/wiki/A*_search_algorithm
        let h = |n: &Position| {
            let Position((goal_x, goal_y)) = &goal;
            let Position((n_x, n_y)) = n;
            (goal_x - n_x).abs() + (goal_y - n_y).abs()
        };

        // The set of discovered nodes that may need to be (re-)expanded. Initially, only the start node is known. This
        // is usually implemented as a min-heap or priority queue rather than a hash-set.
        let mut open_set = PriorityQueue::<Position, isize>::new();

        // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start to n currently
        // known.
        let mut came_from = AHashMap::<Position, Position>::new();

        // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
        let mut g_score = AHashMap::<Position, isize>::new();
        g_score.insert(start, 0);

        // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to how short a path
        // from start to finish can be if it goes through n.
        let start_score = h(&start);
        open_set.push(start, -start_score);

        while !open_set.is_empty() {
            let current = open_set.pop().unwrap().0; // unwrap ok because set is not empty

            if current == goal {
                return Ok(Some((self.reconstruct_path(&came_from, current), g_score[&current])));
            }

            open_set.remove(&current);
            for neighbor in current.neighbors() {
                let tentative_g_score = g_score[&current] + self.get(neighbor)?.0 as isize;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&isize::MAX) {
                    // This path to neighbor is better than any previous one. Record it!
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    let new_score = tentative_g_score + h(&neighbor);
                    open_set.push(neighbor, -new_score);
                }
            }
        }

        Ok(None)
    }

    fn reconstruct_path(&self, came_from: &AHashMap<Position, Position>, current: Position) -> Vec<Position> {
        let mut total_path = vec![current];
        let mut walker = current;
        while came_from.contains_key(&walker) {
            walker = came_from[&walker];
            total_path.push(walker);
        }
        total_path.reverse();
        total_path
    }

    fn top_left(&self) -> Position {
        Position((0, 0))
    }

    fn bottom_right(&self) -> anyhow::Result<Position> {
        //let (maxrow, finalcol) = (
        let maxrow: usize = self.map.len() * usize::try_from(self.expansion_factor)? - 1;
        let finalcol: usize = self.map[self.map.len() - 1].len() * usize::try_from(self.expansion_factor)? - 1;

        Ok(Position((finalcol.try_into()?, maxrow.try_into()?)))
    }

    fn expand(&mut self, factor: i32) {
        self.expansion_factor = factor;
    }
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input = stdin
        .lock()
        .lines()
        .map(ResultStringWrap::from)
        .collect::<anyhow::Result<Cavern>>()
        .context("Failed to parse puzzle input from stdin")?;

    let (_cheapest_path, total_risk) = input.find_path(input.top_left(), input.bottom_right()?)?.unwrap();
    println!("Part 1: Lowest risk path has risk value {total_risk}");

    input.expand(5);
    let (_cheapest_path, total_risk) = input.find_path(input.top_left(), input.bottom_right()?)?.unwrap();
    println!("Part 2: Expanded map, lowest risk path has value {total_risk}");
    Ok(())
}
