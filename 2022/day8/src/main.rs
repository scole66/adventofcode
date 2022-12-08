//! # Solution for Advent of Code 2022 Day 8: Treetop Tree House
//!
//! Ref: [Advent of Code 2022 Day 8](https://adventofcode.com/2022/day/8)
//!
#![allow(unused_imports, dead_code, unused_variables)]
use ahash::{AHashMap, AHashSet};
use std::io::{self, Read};
use std::iter::{Iterator, Peekable};

struct Input {
    trees: AHashMap<(isize, isize), u8>,
    max_col: isize,
    max_row: isize,
}

impl TryFrom<&str> for Input {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut result: AHashMap<(isize, isize), u8> = AHashMap::new();
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in value.lines().enumerate() {
            let row = isize::try_from(row)?;
            for (column, tree) in line.chars().enumerate() {
                let column = isize::try_from(column)?;
                if tree.is_ascii_digit() {
                    let height = tree as u8 - b'0';

                    result.insert((column, row), height);
                } else {
                    anyhow::bail!("Invalid character in heightmap");
                }
                if column > max_col {
                    max_col = column;
                }
            }
            if row > max_row {
                max_row = row;
            }
        }
        Ok(Input { trees: result, max_col, max_row })
    }
}

struct CoordIter {
    starting_column: isize,
    starting_row: isize,
    delta_row: isize,
    delta_column: isize,
    current: isize,
    num_steps: isize,
}

impl Iterator for CoordIter {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.num_steps {
            None
        } else {
            let result = Some((
                self.starting_column + self.current * self.delta_column,
                self.starting_row + self.current * self.delta_row,
            ));
            self.current += 1;
            result
        }
    }
}

impl Input {
    fn downward_path(&self, starting_column: isize, starting_row: isize) -> CoordIter {
        CoordIter {
            starting_column,
            starting_row,
            delta_row: 1,
            delta_column: 0,
            current: 0,
            num_steps: self.max_row - starting_row + 1,
        }
    }
    fn upward_path(&self, starting_column: isize, starting_row: isize) -> CoordIter {
        CoordIter {
            starting_column,
            starting_row,
            delta_row: -1,
            delta_column: 0,
            current: 0,
            num_steps: starting_row + 1,
        }
    }
    fn leftward_path(&self, starting_column: isize, starting_row: isize) -> CoordIter {
        CoordIter {
            starting_column,
            starting_row,
            delta_row: 0,
            delta_column: -1,
            current: 0,
            num_steps: starting_column + 1,
        }
    }
    fn rightward_path(&self, starting_column: isize, starting_row: isize) -> CoordIter {
        CoordIter {
            starting_column,
            starting_row,
            delta_row: 0,
            delta_column: 1,
            current: 0,
            num_steps: self.max_col - starting_column + 1,
        }
    }

    fn scan_trees(&self, path: CoordIter) -> AHashSet<(isize, isize)> {
        let mut result: AHashSet<(isize, isize)> = AHashSet::new();
        let mut previous_max = -1;
        for coords in path {
            let probe_height = *self.trees.get(&coords).expect("not sparse") as i32;
            if probe_height > previous_max {
                result.insert(coords);
                previous_max = probe_height;
            }
        }
        result
    }

    fn visibility_scan(&self, idx_max: isize, pathgen: impl Fn(isize) -> CoordIter) -> AHashSet<(isize, isize)> {
        let mut result = AHashSet::new();
        for idx in 0..=idx_max {
            result.extend(self.scan_trees(pathgen(idx)));
        }
        result
    }

    fn visible_from_top(&self) -> AHashSet<(isize, isize)> {
        self.visibility_scan(self.max_col, |col| self.downward_path(col, 0))
    }
    fn visible_from_left(&self) -> AHashSet<(isize, isize)> {
        self.visibility_scan(self.max_row, |row| self.rightward_path(0, row))
    }
    fn visible_from_right(&self) -> AHashSet<(isize, isize)> {
        self.visibility_scan(self.max_row, |row| self.leftward_path(self.max_col, row))
    }
    fn visible_from_bottom(&self) -> AHashSet<(isize, isize)> {
        self.visibility_scan(self.max_col, |col| self.upward_path(col, self.max_row))
    }
    fn visible(&self) -> AHashSet<(isize, isize)> {
        let mut result = self.visible_from_top();
        result.extend(self.visible_from_left());
        result.extend(self.visible_from_right());
        result.extend(self.visible_from_bottom());
        result
    }

    fn viewing_distance(&self, mut path: CoordIter) -> isize {
        let viewer_loc = path.next().expect("start in the map");
        let target_height = *self.trees.get(&viewer_loc).expect("start in the map");

        let mut distance = 0;
        loop {
            let probe_loc = path.next();
            match probe_loc {
                None => return distance,
                Some(location) => {
                    let probe_height = *self.trees.get(&location).expect("dense map");
                    if probe_height >= target_height {
                        return distance + 1;
                    }
                    distance += 1;
                }
            }
        }
    }

    fn scenic_score(&self, column: isize, row: isize) -> isize {
        self.viewing_distance(self.downward_path(column, row))
            * self.viewing_distance(self.leftward_path(column, row))
            * self.viewing_distance(self.rightward_path(column, row))
            * self.viewing_distance(self.upward_path(column, row))
    }
}

fn part1(input_str: &str) -> anyhow::Result<usize> {
    let input = Input::try_from(input_str)?;

    Ok(input.visible().len())
}

fn part2(input_str: &str) -> anyhow::Result<isize> {
    let input = Input::try_from(input_str)?;

    let mut max_scenic_score = -1;
    for col in 0..=input.max_col {
        for row in 0..=input.max_row {
            let scenic_score = input.scenic_score(col, row);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }
    Ok(max_scenic_score)
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

    static SAMPLE: &str = indoc::indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn part1_sample() {
        let input = SAMPLE;
        assert_eq!(part1(&input).unwrap(), 21);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 8);
    }
}
