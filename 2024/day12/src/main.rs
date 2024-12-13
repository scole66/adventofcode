//! # Solution for Advent of Code 2024 Day 12: Garden Groups
//!
//! Ref: [Advent of Code 2024 Day 12](https://adventofcode.com/2024/day/12)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::{Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    grid: AHashMap<(i64, i64), char>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = s
            .lines()
            .enumerate()
            .flat_map(move |(row, line)| {
                let row = i64::try_from(row)?;
                Ok::<_, Error>(line.chars().enumerate().map(move |(col, crop)| {
                    let col = i64::try_from(col)?;
                    Ok::<_, Error>(((row, col), crop))
                }))
            })
            .flatten()
            .collect::<Result<AHashMap<(i64, i64), char>, _>>()?;
        Ok(Input { grid })
    }
}
impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row = 0;
        let mut col = 0;
        loop {
            let crop = self.grid.get(&(row, col));
            if let Some(crop) = crop {
                write!(f, "{crop}")?;
                col += 1;
            } else {
                if col == 0 {
                    break;
                }
                writeln!(f)?;
                col = 0;
                row += 1;
            }
        }
        Ok(())
    }
}

// Corner Patterns:
// if A == 1 and X == 0, then each of these is a 4-bit number.
// No Corners:
// AA  AA  XX  XX  AX  XA
// AA  XX  AA  XX  AX  XA
// 0xF 0xC 0x3 0x0 0xA 0x5
//
// One Corner:
// AX  XA  XX  XX  AA  AA  AX  XA
// XX  XX  AX  XA  AX  XA  AA  AA
// 0x8 0x4 0x2 0x1 0xE 0xD 0xB 0x7
//
// Two corners:
// AX  XA
// XA  AX
// 0x9 0x6

const CORNERS: [u8; 16] = [0, 1, 1, 0, 1, 0, 2, 1, 1, 2, 0, 1, 0, 1, 1, 0];

struct Region {
    crops: AHashSet<(i64, i64)>,
    min_col: i64,
    max_col: i64,
    min_row: i64,
    max_row: i64,
}

impl Region {
    fn area(&self) -> usize {
        self.crops.len()
    }

    fn perimeter(&self) -> usize {
        self.crops
            .iter()
            .map(|(row, col)| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .filter(|(drow, dcol)| {
                        let probe_row = *row + *drow;
                        let probe_col = *col + *dcol;
                        !self.crops.contains(&(probe_row, probe_col))
                    })
                    .count()
            })
            .sum()
    }

    fn full_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn corner_code(&self, topleft: (i64, i64)) -> usize {
        let top_left_present = self.crops.contains(&(topleft));
        let (row, col) = topleft;
        let top_right_present = self.crops.contains(&(row, col + 1));
        let bottom_left_present = self.crops.contains(&(row + 1, col));
        let bottom_right_present = self.crops.contains(&(row + 1, col + 1));

        usize::from(top_left_present) << 3
            | usize::from(top_right_present) << 2
            | usize::from(bottom_left_present) << 1
            | usize::from(bottom_right_present)
    }

    fn corner_count(&self) -> usize {
        let mut count = 0;
        for row in self.min_row - 1..=self.max_row {
            for col in self.min_col - 1..=self.max_col {
                count += usize::from(CORNERS[self.corner_code((row, col))]);
            }
        }
        count
    }

    fn discounted_price(&self) -> usize {
        self.area() * self.corner_count()
    }
}

fn pop_from_set(set: &mut AHashSet<(i64, i64)>) -> Option<(i64, i64)> {
    if let Some(item) = set.iter().next() {
        let item = *item;
        set.take(&item)
    } else {
        None
    }
}

impl Input {
    fn all_crops(&self) -> impl Iterator<Item = char> {
        self.grid.values().copied().collect::<AHashSet<_>>().into_iter()
    }

    fn all_regions_for_crop(&self, crop: char) -> Vec<Region> {
        let crops = self
            .grid
            .iter()
            .filter_map(
                |((row, col), in_grid)| {
                    if crop == *in_grid {
                        Some((*row, *col))
                    } else {
                        None
                    }
                },
            )
            .collect::<AHashSet<(i64, i64)>>();
        let mut crops_to_place = crops.clone();
        let mut regions = Vec::new();
        while !crops_to_place.is_empty() {
            let mut region = AHashSet::new();
            let mut min_row = i64::MAX;
            let mut min_col = i64::MAX;
            let mut max_row = i64::MIN;
            let mut max_col = i64::MIN;
            let mut spots_to_check =
                AHashSet::from([pop_from_set(&mut crops_to_place).expect("list should not be empty")]);
            let mut already_checked = AHashSet::new();
            while !spots_to_check.is_empty() {
                let spot = pop_from_set(&mut spots_to_check).expect("list should not be empty");
                already_checked.insert(spot);
                if crops.contains(&spot) {
                    max_row = max_row.max(spot.0);
                    min_row = min_row.min(spot.0);
                    max_col = max_col.max(spot.1);
                    min_col = min_col.min(spot.1);
                    region.insert(spot);
                    spots_to_check.extend(
                        [(1, 0), (-1, 0), (0, 1), (0, -1)]
                            .iter()
                            .map(|(drow, dcol)| (*drow + spot.0, *dcol + spot.1))
                            .filter(|spot| !already_checked.contains(spot)),
                    );
                    crops_to_place.remove(&spot);
                }
            }

            regions.push(Region {
                crops: region,
                min_col,
                max_col,
                min_row,
                max_row,
            });
        }
        regions
    }

    fn full_price(&self, crop: char) -> usize {
        self.all_regions_for_crop(crop).iter().map(Region::full_price).sum()
    }

    fn discounted_price(&self, crop: char) -> usize {
        self.all_regions_for_crop(crop)
            .iter()
            .map(Region::discounted_price)
            .sum()
    }
}

fn part1(input: &Input) -> usize {
    input.all_crops().map(|crop| input.full_price(crop)).sum()
}

fn part2(input: &Input) -> usize {
    input.all_crops().map(|crop| input.discounted_price(crop)).sum()
}

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
    use test_case::test_case;

    static SAMPLE1: &str = indoc::indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};
    static SAMPLE2: &str = indoc::indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};
    static SAMPLE3: &str = indoc::indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test_case(SAMPLE1 => SAMPLE1.to_string(); "small sample")]
    #[test_case(SAMPLE2 => SAMPLE2.to_string(); "inclusion sample")]
    #[test_case(SAMPLE3 => SAMPLE3.to_string(); "big sample")]
    fn parse(inp: &str) -> String {
        let input = inp.parse::<Input>().unwrap();
        format!("{input}")
    }

    #[test_case("A", 'A' => 4; "one crop")]
    #[test_case("AB", 'A' => 4; "two neighbors - A")]
    #[test_case("AB", 'B' => 4; "two neighbors - B")]
    #[test_case(SAMPLE2, 'X' => 16; "inclusion - X")]
    #[test_case(SAMPLE2, 'O' => 36; "inclusion - O")]
    fn perimeter(input: &str, crop: char) -> usize {
        let grid = input.parse::<Input>().unwrap();
        grid.all_regions_for_crop(crop).iter().map(Region::perimeter).sum()
    }

    #[test_case(SAMPLE2, 'X' => 4; "inclusion sample - X")]
    #[test_case(SAMPLE2, 'O' => 21; "inclusion sample - O")]
    fn area(input: &str, crop: char) -> usize {
        let grid = input.parse::<Input>().unwrap();
        grid.all_regions_for_crop(crop).iter().map(Region::area).sum()
    }

    #[test_case(SAMPLE2 => AHashSet::from(['O', 'X']); "inclusion")]
    fn crops(input: &str) -> AHashSet<char> {
        let grid = input.parse::<Input>().unwrap();
        grid.all_crops().collect::<AHashSet<_>>()
    }

    #[test_case("A" => 4; "one crop")]
    #[test_case("AB" => 8; "two crop")]
    #[test_case(SAMPLE1 => 140; "small sample")]
    #[test_case(SAMPLE2 => 772; "inclusion sample")]
    #[test_case(SAMPLE3 => 1930; "bigger sample")]
    fn part1_sample(inp: &str) -> usize {
        part1(&inp.parse::<Input>().unwrap())
    }

    static SAMPLE4: &str = indoc::indoc! {"
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE
    "};

    static SAMPLE5: &str = indoc::indoc! {"
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
    "};

    #[test_case("A" => 4; "one char")]
    #[test_case(SAMPLE1 => 80; "small sample")]
    #[test_case(SAMPLE2 => 436; "inclusion sample")]
    #[test_case(SAMPLE3 => 1206; "bigger sample")]
    #[test_case(SAMPLE4 => 236; "e-shaped sample")]
    #[test_case(SAMPLE5 => 368; "kitty corner sample")]
    fn part2_sample(inp: &str) -> usize {
        part2(&inp.parse::<Input>().unwrap())
    }
}
