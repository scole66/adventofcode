//! # Solution for Advent of Code 2021 Day 9
//!
//! Ref: [Advent of Code 2021 Day 9](https://adventofcode.com/2021/day/9)
//!

use ahash::{AHashMap, AHashSet};
use std::io::{self, BufRead};

struct HeightMap {
    map: AHashMap<(i32, i32), u32>,
}

impl<S> FromIterator<S> for HeightMap
where
    S: AsRef<str>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self
    where
        S: AsRef<str>,
    {
        let mut hm: AHashMap<(i32, i32), u32> = Default::default();

        for (row, s) in iter.into_iter().enumerate() {
            let r = row.try_into().unwrap();
            for (column, digit) in s.as_ref().chars().enumerate() {
                let c = column.try_into().unwrap();
                hm.insert((r, c), digit.to_digit(10).unwrap());
            }
        }

        HeightMap { map: hm }
    }
}

impl HeightMap {
    fn low_points(&self) -> Vec<(i32, i32)> {
        let mut result = vec![];

        for (&(row, col), &height) in self.map.iter() {
            if *self.map.get(&(row - 1, col)).unwrap_or(&u32::MAX) > height
                && *self.map.get(&(row + 1, col)).unwrap_or(&u32::MAX) > height
                && *self.map.get(&(row, col - 1)).unwrap_or(&u32::MAX) > height
                && *self.map.get(&(row, col + 1)).unwrap_or(&u32::MAX) > height
            {
                result.push((row, col));
            }
        }

        result
    }

    fn risk_level(&self, row: i32, col: i32) -> u32 {
        *self.map.get(&(row, col)).unwrap() + 1
    }

    fn basin_area(&self, row: i32, col: i32) -> usize {
        let mut to_do: AHashSet<(i32, i32)> = Default::default();
        to_do.insert((row, col));
        let mut done: AHashSet<(i32, i32)> = Default::default();

        while to_do.len() > 0 {
            let (row, col) = *to_do.iter().next().unwrap();
            to_do.remove(&(row, col));
            let current_height = *self.map.get(&(row, col)).unwrap();

            let above = (row - 1, col);
            let below = (row + 1, col);
            let to_left = (row, col - 1);
            let to_right = (row, col + 1);

            macro_rules! check_and_insert {
                ( $point: ident ) => {
                    let height = *self.map.get(&$point).unwrap_or(&u32::MAX);
                    if height > current_height && height < 9 && !done.contains(&$point) {
                        to_do.insert($point);
                    }
                };
            }

            check_and_insert!(above);
            check_and_insert!(below);
            check_and_insert!(to_left);
            check_and_insert!(to_right);

            done.insert((row, col));
        }

        done.len()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let height_map = stdin.lock().lines().map_while(Result::ok).collect::<HeightMap>();

    // Part 1: Sum of the risk levels of all the low points.
    let total_risk: u32 = height_map
        .low_points()
        .iter()
        .map(|&(row, col)| height_map.risk_level(row, col))
        .sum();

    println!("Part 1: Total Risk: {total_risk}");

    let mut all_basin_sizes: Vec<usize> = height_map
        .low_points()
        .iter()
        .map(|&(row, col)| height_map.basin_area(row, col))
        .collect();
    all_basin_sizes.sort_by(|a, b| b.cmp(a)); // biggest to smallest
    assert!(all_basin_sizes.len() >= 3);

    println!(
        "Part 2: Product of 3 biggest basins: {}",
        all_basin_sizes[0..3].iter().product::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &[&str] = &["2199943210", "3987894921", "9856789892", "8767896789", "9899965678"];

    #[test]
    fn low_points() {
        let height_map = SAMPLE.iter().collect::<HeightMap>();

        let low_points = height_map.low_points();
        assert_eq!(low_points.len(), 4);

        let low_point_set: AHashSet<(i32, i32)> = AHashSet::from_iter(low_points);
        assert_eq!(
            low_point_set,
            AHashSet::<(i32, i32)>::from_iter(vec![(0, 1), (0, 9), (2, 2), (4, 6)])
        );
    }
    #[test]
    fn risk_level() {
        let height_map = SAMPLE.iter().collect::<HeightMap>();

        let low_points = height_map.low_points();
        assert_eq!(
            low_points
                .iter()
                .map(|&(row, col)| height_map.risk_level(row, col))
                .sum::<u32>(),
            15
        );
    }

    #[test_case((0,1) => 3)]
    #[test_case((0,9) => 9)]
    #[test_case((2,2) => 14)]
    #[test_case((4,6) => 9)]
    fn basin_area(point: (i32, i32)) -> usize {
        let height_map = SAMPLE.iter().collect::<HeightMap>();
        height_map.basin_area(point.0, point.1)
    }
}
