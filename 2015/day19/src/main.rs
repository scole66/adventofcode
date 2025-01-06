//! # Solution for Advent of Code 2015 Day 19: Medicine for Rudolph
//!
//! Ref: [Advent of Code 2015 Day 19](https://adventofcode.com/2015/day/19)
//!
use ahash::AHashSet;
use anyhow::{anyhow, bail, Context, Error, Result};
use astar::{search_astar, AStarNode};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

struct ReplacementRule {
    source: String,
    replacement: String,
}
impl FromStr for ReplacementRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("^(?P<source>[a-zA-Z]+) => (?P<replacement>[a-zA-Z]+)$").unwrap());
        let caps = PATTERN
            .captures(s)
            .ok_or_else(|| anyhow!("Not a valid replacement rule: \"{s}\""))?;
        Ok(ReplacementRule {
            source: caps["source"].to_string(),
            replacement: caps["replacement"].to_string(),
        })
    }
}

struct Data {
    rules: Vec<ReplacementRule>,
    target: String,
}
impl FromStr for Data {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static TARGET_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^[a-zA-Z]+$").unwrap());
        let (rule_expressions, target_str) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("rules and target should be separated by a blank line"))?;
        let rules = rule_expressions
            .lines()
            .map(|line| line.parse::<ReplacementRule>())
            .collect::<Result<Vec<_>>>()
            .context("replacement rules should be valid")?;
        let target = target_str.trim();
        if !TARGET_PATTERN.is_match(target) {
            bail!("invalid target (not alphabetic)");
        }
        Ok(Data {
            rules,
            target: target.to_string(),
        })
    }
}

fn split_at_nth<'a>(source: &'a str, delimiter: &str, n: usize) -> Option<(&'a str, &'a str)> {
    let mut scan = source;
    let mut matches_left = n - 1;
    let mut offset = 0;
    loop {
        let location = scan.find(delimiter).map(|loc| loc + offset);
        match location {
            None => {
                return None;
            }
            Some(loc) => {
                if matches_left == 0 {
                    return Some((&source[..loc], &source[loc + delimiter.len()..]));
                }
                matches_left -= 1;
                offset = loc + 1;
                scan = &source[offset..];
            }
        }
    }
}

fn distinct_replacements(rules: &Vec<ReplacementRule>, compound: &str) -> AHashSet<String> {
    let mut result = AHashSet::new();
    for rule in rules {
        let mut instance = 1;
        loop {
            match split_at_nth(compound, &rule.source, instance) {
                None => {
                    break;
                }
                Some((header, trailer)) => {
                    let compound = format!("{header}{}{trailer}", rule.replacement);
                    result.insert(compound);
                    instance += 1;
                }
            }
        }
    }
    result
}
fn reverse_replacements(rules: &Vec<ReplacementRule>, compound: &str) -> AHashSet<String> {
    let mut result = AHashSet::new();
    for rule in rules {
        let mut instance = 1;
        loop {
            match split_at_nth(compound, &rule.replacement, instance) {
                None => {
                    break;
                }
                Some((header, trailer)) => {
                    let compound = format!("{header}{}{trailer}", rule.source);
                    result.insert(compound);
                    instance += 1;
                }
            }
        }
    }
    result
}

impl Data {
    fn distinct_replacements(&self) -> AHashSet<String> {
        distinct_replacements(&self.rules, &self.target)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct SearchNode {
    compound: String,
}
struct MoleculeState {
    data: Data,
}
struct MoleculeNeighborIter {
    data: Vec<String>,
    next: usize,
}
impl Iterator for MoleculeNeighborIter {
    type Item = (SearchNode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.data.len() {
            None
        } else {
            self.next += 1;
            Some((
                SearchNode {
                    compound: self.data[self.next - 1].clone(),
                },
                1,
            ))
        }
    }
}

impl AStarNode for SearchNode {
    type Cost = usize;

    type AssociatedState = MoleculeState;

    fn heuristic(&self, goal: &Self, _state: &Self::AssociatedState) -> Self::Cost {
        let current_len = self.compound.len();
        let goal_len = goal.compound.len();
        current_len - goal_len + 1
    }

    fn neighbors(&self, state: &Self::AssociatedState) -> impl Iterator<Item = (Self, Self::Cost)> {
        let potentials = reverse_replacements(&state.data.rules, &self.compound)
            .into_iter()
            .filter(|potential| potential.len() <= state.data.target.len())
            .collect::<Vec<_>>();
        MoleculeNeighborIter {
            data: potentials,
            next: 0,
        }
    }

    fn goal_match(&self, goal: &Self, _state: &Self::AssociatedState) -> bool {
        self.compound == goal.compound
    }
}

fn part1(input: &str) -> Result<usize> {
    let data = input.parse::<Data>()?;

    Ok(data.distinct_replacements().len())
}

fn part2(input: &str) -> Result<usize> {
    let data = input.parse::<Data>()?;
    let state = MoleculeState { data };
    let path = search_astar(
        SearchNode {
            compound: state.data.target.clone(),
        },
        SearchNode {
            compound: "e".to_string(),
        },
        &state,
    );

    Ok(path.unwrap().len() - 1)
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    print!(indoc::indoc! {"
        Sometimes (most of the time?) this randomly picks a poor starting choice, and
        runs out of ram. When the stars align, it picks a good choice and returns in
        seconds. So if this seems to go on for too long, Ctrl-C and retry.
    "});
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        H => HO
        H => OH
        O => HH

        HOH
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 4);
    }

    static SAMPLE2: &str = indoc::indoc! {"
        e => H
        e => O
        H => HO
        H => OH
        O => HH

        HOHOHO
    "};

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE2).unwrap(), 6);
    }

    #[test_case("abcd--fghi--kmlm--asea", "--", 1 => Some(("abcd", "fghi--kmlm--asea")))]
    #[test_case("abcd--fghi--kmlm--asea", "--", 2 => Some(("abcd--fghi", "kmlm--asea")))]
    #[test_case("abcd--fghi--kmlm--asea", "--", 3 => Some(("abcd--fghi--kmlm", "asea")))]
    #[test_case("abcd--fghi--kmlm--asea", "--", 4 => None)]
    fn split_at_nth<'a>(src: &'a str, delim: &str, n: usize) -> Option<(&'a str, &'a str)> {
        super::split_at_nth(src, delim, n)
    }
}
