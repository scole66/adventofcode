//! # Solution for Advent of Code 2022 Day 17: Pyroclastic Flow
//!
//! Ref: [Advent of Code 2022 Day 17](https://adventofcode.com/2022/day/17)
//!
use ahash::AHashMap;
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::fmt::Display;
use std::io::{self, Read};
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    col: isize,
    row: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    rock_idx: u32,
    jet_idx: u32,
    rocks: Vec<Point>,
}
impl CacheKey {
    fn new(value: &Canvas) -> Self {
        let mut rocks = value.spots.keys().cloned().collect::<Vec<_>>();
        rocks.sort_by(|a, b| match a.row.cmp(&b.row) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.col.cmp(&b.col),
            Ordering::Greater => Ordering::Greater,
        });

        CacheKey {
            rock_idx: u32::try_from(value.rock_idx).expect("rock_idx should fit in a u32"),
            jet_idx: u32::try_from(value.jet_idx).expect("jet_idx should fit in a u32"),
            rocks,
        }
    }
}

#[derive(Debug)]
struct CacheEntry {
    height_delta: u32,
    jet_idx_delta: u32,
    rocks: Vec<Point>,
    iter_number: usize,
    original_floor_offset: usize,
}

struct Canvas {
    spots: AHashMap<Point, Rock>,
    jets: AirJets,
    jet_idx: usize,
    rock_idx: usize,
    state: RocksAre,
    floor_offset: usize,
    cache: AHashMap<CacheKey, CacheEntry>,
}
impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(highest_nonempty_row) = self.highest_nonempty_row() {
            for row in (0..=highest_nonempty_row).rev() {
                for col in 0..CANVAS_WIDTH {
                    match self.at_spot(&Point { col, row }) {
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
        Canvas {
            spots: AHashMap::new(),
            jets,
            jet_idx: 0,
            rock_idx: 0,
            state: RocksAre::Stopped,
            floor_offset: 0,
            cache: AHashMap::new(),
        }
    }

    fn at_spot(&self, key: &Point) -> Space {
        self.spots.get(key).into()
    }

    fn highest_nonempty_row(&self) -> Option<isize> {
        self.spots.keys().max_by_key(|&pt| pt.row).map(|pt| pt.row)
    }

    fn add_new_rock(&mut self) {
        let row_offset = self.highest_nonempty_row().unwrap_or(-1) + 4;
        let col_offset = 2;
        let pattern = &PATTERNS[self.rock_idx];
        self.rock_idx = (self.rock_idx + 1) % PATTERNS.len();
        for &(x, y) in pattern.iter() {
            let col = x as isize;
            let row = y as isize;
            self.spots
                .insert(Point { col: col_offset + col, row: row_offset + row }, Rock::Falling);
        }

        self.state = RocksAre::Falling;
    }

    fn rock_in_motion(&self) -> bool {
        self.state == RocksAre::Falling
    }

    fn blow(&mut self) {
        let blow_direction = self.jets.0[self.jet_idx];
        self.jet_idx = (self.jet_idx + 1) % self.jets.0.len();
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
            .filter_map(|(key, value)| (*value == Rock::Falling).then_some(*key))
            .collect::<Vec<_>>();
        // Check to see if any of those would bump into anything, either the edge of the canvas, or another
        // stationary rock.
        if !locs_to_adjust.iter().any(|spot| {
            let adjusted = Point { col: spot.col + delta_x, row: spot.row };
            matches!(self.spots.get(&adjusted), Some(Rock::Stuck)) || adjusted.col < 0 || adjusted.col >= CANVAS_WIDTH
        }) {
            // We're good. Take all the old ones out entirely, and then put them back in where they belong.
            let mut todo_list = vec![];
            for spot in locs_to_adjust {
                let value = self.spots.remove(&spot).unwrap();
                todo_list.push((spot, value));
            }
            for (spot, value) in todo_list {
                self.spots
                    .insert(Point { col: spot.col + delta_x, row: spot.row }, value);
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
            .filter_map(|(key, value)| (*value == Rock::Falling).then_some(*key))
            .collect::<Vec<_>>();
        // Check to see if any of those would bump into anything, either the floor of the canvas, or another
        // stationary rock.
        if !locs_to_adjust.iter().any(|spot| {
            let adjusted = Point { col: spot.col, row: spot.row - 1 };
            matches!(self.spots.get(&adjusted), Some(Rock::Stuck)) || adjusted.row < 0
        }) {
            // We're good. Go ahead and move stuff around.
            let mut todo_list = vec![];
            for spot in locs_to_adjust {
                let value = self.spots.remove(&spot).unwrap();
                todo_list.push((spot, value));
            }
            for (spot, value) in todo_list {
                self.spots.insert(Point { col: spot.col, row: spot.row - 1 }, value);
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

    fn raise_floor(&mut self) {
        // After a rock has fallen, keep only the rows that matter to handle future falls, and update the
        // floor offset. (This is to keep tall towers fitting into memory.)
        let lowest_keepable_row = (0..CANVAS_WIDTH)
            .map(|col| {
                self.spots
                    .keys()
                    .filter_map(|pt| (pt.col == col).then_some(pt.row))
                    .max()
                    .unwrap_or(0)
            })
            .min()
            .expect("CANVAS_WIDTH should not be negative");

        if lowest_keepable_row > 0 {
            let new_spots = AHashMap::from_iter(self.spots.keys().filter_map(|&pt| {
                (pt.row >= lowest_keepable_row)
                    .then_some((Point { col: pt.col, row: pt.row - lowest_keepable_row }, Rock::Stuck))
            }));
            self.spots = new_spots;
            self.floor_offset += usize::try_from(lowest_keepable_row).expect("positive value should not be negative");
        }
    }

    fn drop_rock(&mut self, iteration_num: usize) {
        let key = CacheKey::new(self);
        match self.cache.get(&key) {
            Some(entry) => {
                self.floor_offset += entry.height_delta as usize;
                self.jet_idx = (self.jet_idx + entry.jet_idx_delta as usize) % self.jets.0.len();
                self.rock_idx = (self.rock_idx + 1) % PATTERNS.len();
                self.spots = AHashMap::from_iter(entry.rocks.iter().map(|&point| (point, Rock::Stuck)))
            }
            None => {
                let starting_offset = self.floor_offset;
                let starting_jet_index = self.jet_idx;

                self.add_new_rock();
                while self.rock_in_motion() {
                    self.blow();
                    self.fall();
                }

                // raise the floor
                self.raise_floor();

                // add back to cache:
                let mut rocks = self.spots.keys().cloned().collect::<Vec<_>>();
                rocks.sort_by(|a, b| match a.row.cmp(&b.row) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => a.col.cmp(&b.col),
                    Ordering::Greater => Ordering::Greater,
                });
                let entry = CacheEntry {
                    height_delta: u32::try_from(self.floor_offset - starting_offset)
                        .expect("Height deltas should fit in a u32"),
                    jet_idx_delta: u32::try_from(
                        (self.jets.0.len() + self.jet_idx - starting_jet_index) % self.jets.0.len(),
                    )
                    .expect("Jet index should fit in a u32"),
                    rocks,
                    iter_number: iteration_num,
                    original_floor_offset: starting_offset,
                };
                self.cache.insert(key, entry);
            }
        }
    }

    fn height(&self) -> isize {
        self.highest_nonempty_row().unwrap_or(-1)
            + 1
            + isize::try_from(self.floor_offset).expect("floor should fit into an isize")
    }

    fn height_after(&mut self, iterations: usize) -> isize {
        let mut max_step = iterations;
        let mut num = 0;
        let mut might_repeat = true;
        while num < max_step {
            if might_repeat {
                let key = CacheKey::new(self);
                if let Some(entry) = self.cache.get(&key) {
                    let cycle_repetitions = num - entry.iter_number;
                    let cycle_height = self.floor_offset - entry.original_floor_offset;
                    let instantly_consumed = (max_step - num) / cycle_repetitions;
                    self.floor_offset += cycle_height * instantly_consumed;
                    max_step -= instantly_consumed * cycle_repetitions;
                    might_repeat = false;
                }
            }
            self.drop_rock(num);
            num += 1;
        }
        self.height()
    }
}

fn part1(input: &str) -> anyhow::Result<isize> {
    let jets = input.parse::<AirJets>()?;
    let mut canvas = Canvas::new(jets);

    for num in 0..2022 {
        canvas.drop_rock(num);
    }

    Ok(canvas.height())
}

fn part2(input: &str) -> anyhow::Result<isize> {
    let jets = input.parse::<AirJets>()?;

    let mut cached_canvas = Canvas::new(jets);
    Ok(cached_canvas.height_after(1000000000000))
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
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 1514285714288);
    }
}
