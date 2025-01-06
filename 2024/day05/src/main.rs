//! # Solution for Advent of Code 2024 Day 5: Print Queue
//!
//! Ref: [Advent of Code 2024 Day 5](https://adventofcode.com/2024/day/5)
//!
use anyhow::{anyhow, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct OrderingRule {
    before: i64,
    after: i64,
}
impl FromStr for OrderingRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split('|');
        let before = iter
            .next()
            .map(str::parse)
            .ok_or_else(|| anyhow!("no items in rule"))??;
        let after = iter
            .next()
            .map(str::parse)
            .ok_or_else(|| anyhow!("no right item in rule"))??;
        Ok(OrderingRule { before, after })
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<i64>,
}
impl FromStr for Update {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let pages = s.split(',').map(str::parse).collect::<Result<Vec<_>, _>>()?;
        Ok(Update { pages })
    }
}
impl Update {
    fn violates_rule(&self, rule: &OrderingRule) -> bool {
        let OrderingRule { before, after } = rule;
        let mut before_seen = false;
        let mut after_seen = false;
        for page in &self.pages {
            if page == before {
                if after_seen {
                    return true;
                }
                before_seen = true;
            } else if page == after {
                if before_seen {
                    return false;
                }
                after_seen = true;
            }
        }
        false
    }
    fn is_correct(&self, rules: &[OrderingRule]) -> bool {
        rules.iter().all(|rule| !self.violates_rule(rule))
    }
    fn middle_page(&self) -> i64 {
        self.pages[self.pages.len() / 2]
    }
    fn correct(&self, rules: &[OrderingRule]) -> Self {
        let mut result = vec![];
        for page in &self.pages {
            let mut befores = vec![];
            let mut afters = vec![];
            for rule in rules {
                if *page == rule.before {
                    afters.push(rule.after);
                } else if *page == rule.after {
                    befores.push(rule.before);
                }
            }
            let mut earliest_before = None;
            for (idx, probe) in result.iter().enumerate() {
                if befores.contains(probe) {
                    earliest_before = Some(idx);
                    break;
                }
            }
            let mut latest_after = None;
            for (idx, probe) in result.iter().enumerate().rev() {
                if afters.contains(probe) {
                    latest_after = Some(idx);
                    break;
                }
            }
            match (earliest_before, latest_after) {
                (None, _) => {
                    result.push(*page);
                }
                (Some(_), None) => {
                    result.insert(0, *page);
                }
                (Some(_), Some(after)) => result.insert(after + 1, *page),
            }
        }
        Update { pages: result }
    }
}

#[derive(Debug)]
struct Input {
    rules: Vec<OrderingRule>,
    updates: Vec<Update>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rules = vec![];
        let mut iter = s.lines();
        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }
            rules.push(line.parse::<OrderingRule>()?);
        }
        let mut updates = vec![];
        for line in iter {
            updates.push(line.parse::<Update>()?);
        }
        Ok(Input { rules, updates })
    }
}

impl Input {
    fn part1(&self) -> i64 {
        self.updates
            .iter()
            .filter_map(|up| {
                if up.is_correct(&self.rules) {
                    Some(up.middle_page())
                } else {
                    None
                }
            })
            .sum()
    }
    fn part2(&self) -> i64 {
        self.updates
            .iter()
            .filter_map(|up| {
                if up.is_correct(&self.rules) {
                    None
                } else {
                    let corrected = up.correct(&self.rules);
                    Some(corrected.middle_page())
                }
            })
            .sum()
    }
}

fn part1(input: &Input) -> i64 {
    input.part1()
}

fn part2(input: &Input) -> i64 {
    input.part2()
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

    static SAMPLE: &str = indoc::indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn parse() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(input.rules.len(), 21);
        assert_eq!(input.updates.len(), 6);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 143);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 123);
    }
}
