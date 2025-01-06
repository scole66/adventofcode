//! # Solution for Advent of Code 2023 Day 12: Hot Springs
//!
//! Ref: [Advent of Code 2023 Day 12](https://adventofcode.com/2023/day/12)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum State {
    Working,
    Broken,
    Unknown,
}
impl TryFrom<char> for State {
    type Error = Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        use State::*;
        Ok(match value {
            '.' => Working,
            '#' => Broken,
            '?' => Unknown,
            _ => bail!("Invalid Spring State {value}"),
        })
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Working => '.',
                State::Broken => '#',
                State::Unknown => '?',
            }
        )
    }
}

struct Map(Vec<State>);
impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars().map(State::try_from).collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for state in self.0.iter() {
            write!(f, "{state}")?;
        }
        Ok(())
    }
}
struct Row {
    map: Map,
    groupings: Vec<i64>,
}
impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_str, group_str) = s.split_once(' ').ok_or_else(|| anyhow!("row had no separator"))?;
        let map = map_str.parse::<Map>()?;
        let groupings = group_str
            .split(',')
            .map(|digits| digits.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Row { map, groupings })
    }
}
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.map, self.groupings.iter().join(","))
    }
}
impl Row {
    fn count_arrangements(&self) -> i64 {
        todo!()
    }
}

struct Input(Vec<Row>);
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse::<Row>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            writeln!(f, "{row}")?;
        }
        Ok(())
    }
}

fn part1(input: &Input) -> i64 {
    input.0.iter().map(|row| row.count_arrangements()).sum::<i64>()
}

fn part2(input: &Input) -> i64 {
    todo!()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    let input = input.parse::<Input>()?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn parse() {
        let input = SAMPLE.parse::<Input>().unwrap();
        let result = format!("{input}");
        assert_eq!(SAMPLE, result);
    }

    //#[test_case("???.### 1,1,3" => 1)]
    //#[test_case(".??..??...?##. 1,1,3" => 4)]
    //#[test_case("?#?#?#?#?#?#?#? 1,3,1,6" => 1)]
    //#[test_case("????.#...#... 4,1,1" => 1)]
    //#[test_case("????.######..#####. 1,6,5" => 4)]
    //#[test_case("?###???????? 3,2,1" => 10)]
    //fn count_arrangements(rowstr: &str) -> i64 {
    //    rowstr.parse::<Row>().unwrap().count_arrangements()
    //}

    //#[test]
    //fn part1_sample() {
    //    let input = SAMPLE.parse::<Input>().unwrap();
    //    assert_eq!(part1(&input), 21);
    //}

    #[test]
    #[should_panic]
    fn part2_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part2(&input), 36);
    }
}
