//! # Solution for Advent of Code 2024 Day 16: Reindeer Maze
//!
//! Ref: [Advent of Code 2024 Day 16](https://adventofcode.com/2024/day/16)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

struct Input {
    map: AHashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Item {
    Wall,
    Start,
    End
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let items = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line
                    .chars()
                    .enumerate()
                    .filter_map(|(col, ch)| {
                        match ch {
                            '#' => Ok(Some(((row, col), Item::Wall))),
                            '.' => Ok(None),
                            'S' => Ok(Some(((row, col), Item::Start))),
                            'E' => Ok(Some(((row, col), Item::End))),
                            _ => bail!("Bad Map Item"),
                        }
                    })
            })
    }
}


fn part1(input: &Input) -> i64 {
    todo!()
}

fn part2(input: &Input) -> i64 {
    todo!()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = 0; //part2(&input);
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

    static SAMPLE: &str = indoc::indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    static SAMPLE2: &str = indoc::indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test_case(SAMPLE => 7036; "first sample")]
    #[test_case(SAMPLE2 => 11048; "second sample")]
    fn part1_sample(inp: &str) -> i64 {
        part1(&inp.parse::<Input>().unwrap())
    }

    #[test]
    #[should_panic]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 36);
    }
}
