//! # Solution for Advent of Code 2015 Day 13:
//!
//! Ref: [Advent of Code 2015 Day 13](https://adventofcode.com/2015/day/13)
//!
use ahash::AHashMap;
use anyhow::{anyhow, Error, Result};
use combinations::Permutation;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

struct Datum {
    receiver: String,
    influencer: String,
    happiness_delta: i64,
}

impl FromStr for Datum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<receiver>[A-Za-z]+) would (?P<gl>gain|lose) (?P<val>0|[1-9][0-9]*) happiness units? by sitting next to (?P<influencer>[A-Za-z]+)\.$").unwrap()
        });
        let caps = PATTERN.captures(s).ok_or_else(|| anyhow!("Bad input line"))?;
        let receiver = caps["receiver"].to_string();
        let influencer = caps["influencer"].to_string();
        let happiness_delta = caps["val"].parse::<i64>()? * (if &caps["gl"] == "gain" { 1 } else { -1 });

        Ok(Datum { receiver, influencer, happiness_delta })
    }
}

struct SeatingMatrix {
    matrix: AHashMap<String, AHashMap<String, i64>>,
}

impl FromStr for SeatingMatrix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matrix = AHashMap::new();
        for datum in s.lines().map(|line| line.parse::<Datum>()) {
            let Datum { receiver, influencer, happiness_delta } = datum?;
            let inf_copy = influencer.clone();
            matrix
                .entry(receiver)
                .and_modify(|inner: &mut AHashMap<String, i64>| {
                    inner.insert(influencer, happiness_delta);
                })
                .or_insert_with(|| AHashMap::from_iter([(inf_copy, happiness_delta)]));
        }

        Ok(SeatingMatrix { matrix })
    }
}

impl SeatingMatrix {
    fn best_seating_value(&self) -> i64 {
        let people = self.matrix.keys().collect::<Vec<_>>();
        // We actually don't need all the permutations, since this is a circular pattern. (a-b-c has the same
        // result as b-c-a). We can hold one of the items in the same location. So this asks for the
        // permutations on [1..] (i.e. skipping the first), and then adding that first back in just before we
        // make the sum.
        Permutation::new(&people[1..])
            .map(|mut perm| {
                perm.push(people[0]);
                perm.into_iter()
                    .circular_tuple_windows()
                    .map(|(left, middle, right)| self.matrix[middle][left] + self.matrix[middle][right])
                    .sum::<i64>()
            })
            .max()
            .expect("We have data")
    }
}

fn part1(input: &str) -> Result<i64> {
    let sm = input.parse::<SeatingMatrix>()?;
    Ok(sm.best_seating_value())
}

fn part2(input: &str) -> Result<i64> {
    let mut sm = input.parse::<SeatingMatrix>()?;
    let people = sm.matrix.keys().cloned().collect::<Vec<_>>();
    let new_row = AHashMap::from_iter(people.iter().map(|name| (name.clone(), 0)));
    sm.matrix.insert("Part 2".to_string(), new_row);
    for person in people {
        let entry = sm.matrix.get_mut(&person).expect("person should exist");
        entry.insert("Part 2".to_string(), 0);
    }
    Ok(sm.best_seating_value())
}

fn main() -> Result<()> {
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
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 330);
    }
}
