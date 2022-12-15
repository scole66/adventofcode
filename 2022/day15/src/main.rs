//! # Solution for Advent of Code 2022 Day 15: Beacon Exclusion Zone
//!
//! Ref: [Advent of Code 2022 Day 15](https://adventofcode.com/2022/day/15)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::AHashSet;
use anyhow::Context;
use itertools::chain;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

//         Sensor at x=2, y=18: closest beacon is at x=-2, y=15

struct Item {
    scanner: Point,
    beacon: Point,
}
impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sensor: {}; beacon: {}", self.scanner, self.beacon)
    }
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static STEP_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^Sensor at x=(?P<scanner_x>0|-?[1-9][0-9]*), y=(?P<scanner_y>0|-?[1-9][0-9]*): closest beacon is at x=(?P<beacon_x>0|-?[1-9][0-9]*), y=(?P<beacon_y>0|-?[1-9][0-9]*)$").unwrap()
        });
        let caps = STEP_PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Bad data format: {s}"))?;
        let scanner_x = caps["scanner_x"].parse::<isize>()?;
        let scanner_y = caps["scanner_y"].parse::<isize>()?;
        let beacon_x = caps["beacon_x"].parse::<isize>()?;
        let beacon_y = caps["beacon_y"].parse::<isize>()?;
        Ok(Item { scanner: Point { col: scanner_x, row: scanner_y }, beacon: Point { col: beacon_x, row: beacon_y } })
    }
}

struct InputData(Vec<Item>);
impl FromStr for InputData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InputData(
            s.lines()
                .map(|s| s.parse::<Item>())
                .collect::<anyhow::Result<Vec<_>>>()?,
        ))
    }
}

#[derive(Copy, Clone)]
struct Point {
    col: isize,
    row: isize,
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.col, self.row)
    }
}

impl Point {
    fn mh_distance(&self, other: &Self) -> isize {
        (self.col - other.col).abs() + (self.row - other.row).abs()
    }
}

impl Item {
    fn row_impact(&self, row: isize) -> Option<(isize, isize)> {
        let item_size = self.scanner.mh_distance(&self.beacon);
        let rows_away = (self.scanner.row - row).abs();
        if rows_away > item_size {
            None
        } else {
            let left = self.scanner.col - (rows_away - item_size).abs();
            let right = self.scanner.col + (rows_away - item_size).abs();
            if self.beacon.row != row {
                Some((left, right + 1))
            } else if self.beacon.col < self.scanner.col {
                Some((left + 1, right + 1))
            } else if self.beacon.col > self.scanner.col {
                Some((left, right))
            } else {
                None
            }
        }
    }

    fn right_edge(&self) -> impl Iterator<Item = Point> + '_ {
        let scanner_radius = self.scanner.mh_distance(&self.beacon);
        (-scanner_radius..=scanner_radius)
            .map(move |x| Point { col: self.scanner.col + (scanner_radius.abs() - x) + 1, row: self.scanner.row + x })
    }

    fn covers(&self, point: &Point) -> bool {
        // Returns true if the input point is within this scanner's detection diamond.
        let scanner_manhatten_distance = self.scanner.mh_distance(&self.beacon);
        let input_manhatten_distance = self.scanner.mh_distance(point);
        input_manhatten_distance <= scanner_manhatten_distance
    }
}

fn part1(input: &str, row: isize) -> anyhow::Result<isize> {
    let data = input.parse::<InputData>()?;

    // So what we want to do here is identify where a beacon cannot exist for
    // just one line. This is modelled as a sorted vector of start/end pairs.
    let impacts = data
        .0
        .iter()
        .flat_map(|item| item.row_impact(row))
        .collect::<Vec<_>>();
    let mut starts = impacts.iter().map(|&(start, _)| start).collect::<Vec<_>>();
    starts.sort();
    let mut ends = impacts.iter().map(|&(_, end)| end).collect::<Vec<_>>();
    ends.sort();

    let mut level = 0;
    let mut si = starts.iter().peekable();
    let mut ei = ends.iter().peekable();
    let mut result = vec![];
    loop {
        let (sp, ep) = (si.peek(), ei.peek());
        match (sp, ep) {
            (Some(&start), Some(&end)) if start < end => {
                // increase level
                if level == 0 {
                    result.push(*start);
                }
                level += 1;
                si.next();
            }
            (Some(&start), Some(&end)) if start > end => {
                // decrease level
                level -= 1;
                if level == 0 {
                    result.push(*end);
                }
                ei.next();
            }
            (Some(_), Some(_)) => {
                // no level change, but advance the iterators.
                si.next();
                ei.next();
            }
            (Some(&start), None) => anyhow::bail!("Start after end"),
            (None, Some(&end)) => {
                // decrease level
                level -= 1;
                if level == 0 {
                    result.push(*end);
                }
                ei.next();
            }
            (None, None) => {
                break;
            }
        }
    }

    // result is now a merged start/end/start/end/.../start/end sequence.
    Ok(result.chunks_exact(2).map(|pair| pair[1] - pair[0]).sum())
}

fn part2(input: &str, max_dimension: isize) -> anyhow::Result<isize> {
    // So: many spots not covered by a scanner will have a covered spot or the left edge on its left. Any
    // other uncovered spots will be contained within a region that has at least one of them. Therefore:
    // uncovered regions can be detected simply by scanning the left edge and the right borders of all the
    // scanner diamonds. Any additional uncovered spots will be neighbors of those detected spots. (But the
    // problem statement suggests there will be only one, so the scan will stop when the first is found.)
    let data = input.parse::<InputData>()?;

    let points_to_check = chain!(
        data.0
            .iter()
            .flat_map(|scanner| scanner.right_edge())
            .filter(|&Point { col, row }| col >= 0 && col <= max_dimension && row >= 0 && row <= max_dimension),
        (0..=max_dimension).map(|x| Point { col: 0, row: max_dimension })
    );

    for point in points_to_check {
        if data.0.iter().all(|scanner| !scanner.covers(&point)) {
            return Ok(point.col * 4000000 + point.row);
        }
    }

    Err(anyhow::anyhow!("No uncovered location found."))
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input, 2000000)?);
    println!("Part2: {}", part2(&input, 4000000)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE, 10).unwrap(), 26);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE, 20).unwrap(), 56000011);
    }

    #[test_case("Sensor at x=8, y=7: closest beacon is at x=2, y=10", 10 => Some((3,15)))]
    fn impact(s: &str, row: isize) -> Option<(isize, isize)> {
        let item = s.parse::<Item>().unwrap();
        item.row_impact(row)
    }
}
