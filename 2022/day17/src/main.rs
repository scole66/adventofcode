//! # Solution for Advent of Code 2022 Day 17: Pyroclastic Flow
//!
//! Ref: [Advent of Code 2022 Day 17](https://adventofcode.com/2022/day/17)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap,AHashSet};
use anyhow::Context;
use once_cell::sync::Lazy;
use std::fmt::Display;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

// Coord system: x grows to the right, y grows upward. (0,0) is the leftmost spot just above the floor. ("Just
// above the floor" means no further downward travel is possible.)

static PATTERNS: Lazy<Vec<Vec<(u8, u8)>>> = Lazy::new(|| {
    vec![
        // ####
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // .#.
        // ###
        // .#.
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        // ..#
        // ..#
        // ###
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        // #
        // #
        // #
        // #
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // ##
        // ##
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ]
});

#[derive(PartialEq, Eq)]
enum Rock {
    Falling,
    Stuck,
}

#[derive(PartialEq, Eq)]
enum Space {
    Falling,
    Stuck,
    Air,
}

impl From<Option<&Rock>> for Space {
    fn from(value: Option<&Rock>) -> Self {
        match value {
            Some(Rock::Falling) => Space::Falling,
            Some(Rock::Stuck) => Space::Stuck,
            None => Space::Air,
        }
    }
}

#[derive(Copy, Clone)]
enum AirJet {
    Left,
    Right,
}

impl TryFrom<char> for AirJet {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(AirJet::Left),
            '>' => Ok(AirJet::Right),
            _ => Err(anyhow::anyhow!("Character '{value}' does not describe a valid jet")),
        }
    }
}

struct AirJets(Vec<AirJet>);
impl FromStr for AirJets {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AirJets(
            s.chars()
                .map(AirJet::try_from)
                .collect::<anyhow::Result<Vec<AirJet>>>()?,
        ))
    }
}

#[derive(PartialEq, Eq)]
enum RocksAre {
    Falling,
    Stopped,
}

struct Canvas {
    spots: AHashMap<(isize, isize), Rock>,
    jets: std::iter::Cycle<std::vec::IntoIter<AirJet>>,
    rocks: std::iter::Cycle<std::slice::Iter<'static, Vec<(u8, u8)>>>,
    state: RocksAre,
}
impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(highest_nonempty_row) = self.highest_nonempty_row() {
            for row in (0..=highest_nonempty_row).rev() {
                for col in 0..CANVAS_WIDTH {
                    match self.at_spot(&(col, row)) {
                        Space::Falling => write!(f, "@")?,
                        Space::Stuck => write!(f, "#")?,
                        Space::Air => write!(f, ".")?,
                    }
                }
                writeln!(f)?
            }
        }
        Ok(())
    }
}
const CANVAS_WIDTH: isize = 7;
impl Canvas {
    fn new(jets: AirJets) -> Self {
        let jetiter = jets.0.into_iter().cycle();
        let rocks = PATTERNS.iter().cycle();
        Canvas { spots: AHashMap::new(), jets: jetiter, rocks, state: RocksAre::Stopped }
    }

    fn at_spot(&self, key: &(isize, isize)) -> Space {
        self.spots.get(key).into()
    }

    fn first_empty_row(&self) -> isize {
        (0..)
            .find(|&row| !(0..CANVAS_WIDTH).any(|col| self.spots.contains_key(&(col, row))))
            .unwrap()
    }

    fn highest_nonempty_row(&self) -> Option<isize> {
        self.spots.keys().max_by_key(|(_, row)| row).map(|(_, row)| *row)
    }

    fn add_new_rock(&mut self) {
        let row_offset = self.highest_nonempty_row().unwrap_or(-1) + 4;
        let col_offset = 2;
        let pattern = self.rocks.next().unwrap();
        for &(x, y) in pattern {
            let col = x as isize;
            let row = y as isize;
            self.spots.insert((col_offset + col, row_offset + row), Rock::Falling);
        }

        self.state = RocksAre::Falling;
    }

    fn rock_in_motion(&self) -> bool {
        self.state == RocksAre::Falling
    }

    fn blow(&mut self) {
        let blow_direction = self.jets.next().unwrap();
        let delta_x = match blow_direction {
            AirJet::Left => -1,
            AirJet::Right => 1,
        };

        // Move all the falling blocks horizontally by delta_x. Since the keys in the hashmap are the
        // positions of the rocks, (and because we don't store empty spots), we can't actually mutate the
        // locations while in an iterator. So we build up a vector of (col, row) that need the adjustment, and
        // then play with the hashmap.
        let locs_to_adjust = self
            .spots
            .iter()
            .filter_map(|(key, value)| if *value == Rock::Falling { Some(*key) } else { None })
            .collect::<Vec<_>>();
        // Check to see if any of those would bump into anything, either the edge of the canvas, or another
        // stationary rock.
        if !locs_to_adjust.iter().any(|spot| {
            let adjusted = (spot.0 + delta_x, spot.1);
            matches!(self.spots.get(&adjusted), Some(Rock::Stuck)) || adjusted.0 < 0 || adjusted.0 >= CANVAS_WIDTH
        }) {
            // We're good. Take all the old ones out entirely, and then put them back in where they belong.
            let mut todo_list = vec![];
            for spot in locs_to_adjust {
                let value = self.spots.remove(&spot).unwrap();
                todo_list.push((spot, value));
            }
            for (spot, value) in todo_list {
                self.spots.insert((spot.0 + delta_x, spot.1), value);
            }
        }
    }

    fn fall(&mut self) {
        // Move all the falling blocks downward by 1. Since the keys in the hashmap are the positions of the
        // rocks, (and because we don't store empty spots), we can't actually mutate the locations while in an
        // iterator. So we build up a vector of (col, row) that need the adjustment, and then play with the
        // hashmap.
        let locs_to_adjust = self
            .spots
            .iter()
            .filter_map(|(key, value)| if *value == Rock::Falling { Some(*key) } else { None })
            .collect::<Vec<_>>();
        // Check to see if any of those would bump into anything, either the floor of the canvas, or another
        // stationary rock.
        if !locs_to_adjust.iter().any(|spot| {
            let adjusted = (spot.0, spot.1 - 1);
            matches!(self.spots.get(&adjusted), Some(Rock::Stuck)) || adjusted.1 < 0
        }) {
            // We're good. Go ahead and move stuff around.
            let mut todo_list = vec![];
            for spot in locs_to_adjust {
                let value = self.spots.remove(&spot).unwrap();
                todo_list.push((spot, value));
            }
            for (spot, value) in todo_list {
                self.spots.insert((spot.0, spot.1 - 1), value);
            }
        } else {
            // Downward motion was blocked. In this case, we switch all of the rocks to "Stuck", and turn off
            // the "things are falling" state.
            for spot in locs_to_adjust {
                self.spots.insert(spot, Rock::Stuck);
            }
            self.state = RocksAre::Stopped;
        }
    }

    fn drop_rock(&mut self) {
        self.add_new_rock();
        while self.rock_in_motion() {
            self.blow();
            self.fall();
        }
    }

    fn height(&self) -> isize {
        self.highest_nonempty_row().unwrap_or(-1) + 1
    }
}

fn part1(input: &str) -> anyhow::Result<isize> {
    let jets = input.parse::<AirJets>()?;
    let mut canvas = Canvas::new(jets);

    for _ in 0..2022 {
        canvas.drop_rock();
    }

    Ok(canvas.height())
}


#[derive(PartialEq, Eq, Hash)]
struct CacheKey {
    jet_index: u8,
    rock_index: u8,
    jumble: Vec<u8>,
}
struct CachedCanvas {
    spots: AHashMap<(isize, isize), Rock>,
    jets: AirJets,
    jet_index: usize,
    rock_index: usize,
    state: RocksAre,
    cache: AHashMap<CacheKey, AHashSet<(u8,u8)>>,
    virtual_row_count: isize,
}
impl CachedCanvas {
    fn new(jets: AirJets) -> Self {
        CachedCanvas {
            spots: AHashMap::new(),
            jets,
            jet_index: 0,
            rock_index: 0,
            state: RocksAre::Stopped,
            cache: AHashMap::new(),
            virtual_row_count: 0,
        }
    }
    
    fn highest_nonempty_row(&self) -> Option<isize> {
        self.spots.keys().map(|(_, row)| *row).max()
    }

    fn height(&self) -> isize {
        self.highest_nonempty_row().unwrap_or(-1) + 1 + self.virtual_row_count
    }
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let jets = input.parse::<AirJets>()?;

    let mut cached_canvas = CachedCanvas::new(jets);
    cached_canvas.height_after(1000000000000)
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

    static SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 3068);
    }

    #[test]
    #[should_panic]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 1514285714288);
    }

    mod first_empty_row {
        use super::*;
        #[test]
        fn empty() {
            let jets = SAMPLE.parse::<AirJets>().unwrap();
            let canvas = Canvas::new(jets);
            assert_eq!(canvas.first_empty_row(), 0);
        }
    }
}
