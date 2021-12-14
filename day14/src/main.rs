//! # Solution for Advent of Code 2021 Day 14
//!
//! Ref: [Advent of Code 2021 Day 14](https://adventofcode.com/2021/day/14)
//!

#![allow(unused_imports, dead_code, unused_variables)]

use ahash::{AHashMap, AHashSet};
use anyhow::{self, Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Rule {
    leftright: String,
    insertion: char,
}

impl Rule {
    fn parse(line: &str) -> anyhow::Result<Rule> {
        lazy_static! {
            static ref RULE_PATTERN: Regex = Regex::new("^(?P<leftright>..) -> (?P<insertion>.)$").unwrap();
        }
        let captures = RULE_PATTERN
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("‘{}’ is not a valid rule", line))?;
        Ok(Rule {
            leftright: captures.name("leftright").unwrap().as_str().to_string(),
            insertion: captures.name("insertion").unwrap().as_str().chars().next().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Rules(AHashMap<String, char>);

impl Rules {
    fn apply(&self, template: String) -> anyhow::Result<String> {
        Ok(template
            .chars()
            .fold(Ok((None, String::new())), |accum: anyhow::Result<(Option<char>, String)>, ch| {
                let (prior, b) = accum?;
                let mut builder = b.clone();
                match prior {
                    None => builder.push(ch),
                    Some(p) => {
                        let key = [p, ch].iter().collect::<String>();
                        let insertion = self
                            .0
                            .get(&key)
                            .ok_or_else(|| anyhow::anyhow!("No rule for pair {}", key))?;
                        builder.push(*insertion);
                        builder.push(ch);
                    }
                }
                Ok((Some(ch), builder))
            })?
            .1)
    }
}

#[derive(Debug)]
struct Data {
    template: String,
    rules: Rules,
}

#[derive(Debug)]
struct ResultStringWrap(anyhow::Result<String>);
impl From<anyhow::Result<String>> for ResultStringWrap {
    fn from(src: anyhow::Result<String>) -> Self {
        Self(src)
    }
}
impl From<Result<String, std::io::Error>> for ResultStringWrap {
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}

impl FromIterator<ResultStringWrap> for anyhow::Result<Data> {
    fn from_iter<I: IntoIterator<Item = ResultStringWrap>>(iter: I) -> Self {
        let mut loading_rules = false;
        let mut template: Option<String> = None;
        let mut rules: AHashMap<String,char> = AHashMap::new();
        for ResultStringWrap(res) in iter.into_iter() {
            let line = res?;
            if !loading_rules {
                if line.is_empty() {
                    loading_rules = true;
                } else {
                    match template {
                        None => {
                            template = Some(line);
                        }
                        Some(_) => {
                            anyhow::bail!("Multiple templates detected. Only one is allowed.");
                        }
                    }
                }
            } else {
                let rule = Rule::parse(&line)?;
                rules.insert(rule.leftright, rule.insertion);
            }
        }

        if template.is_none() {
            anyhow::bail!("No template detected in input stream");
        }
        if rules.is_empty() {
            anyhow::bail!("No rules detected in input stream");
        }

        Ok(Data { template: template.unwrap(), rules: Rules(rules) })
    }
}



fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    // Finally learned how to abort an iter at the first error and return it. Now we get:
    //     $ cargo r -q < /dev/urandom
    //     Error: Failed to parse puzzle input from stdin
    //
    //     Caused by:
    //         stream did not contain valid UTF-8
    let input = stdin
        .lock()
        .lines()
        .map(ResultStringWrap::from)
        .into_iter()
        .collect::<anyhow::Result<Data>>()
        .context("Failed to parse puzzle input from stdin")?;

    // Part one: run the template repeatedly through the rules 10 times, then....
    let mut polymer = input.template.clone();
    for _ in 0..10 {
        polymer = input.rules.apply(polymer)?;
    }
    println!("Polymer after 10 iterations: {}", polymer);

    let mut counter : AHashMap<char, usize> = AHashMap::new();
    for ch in polymer.chars() {
        let val = counter.entry(ch).or_insert(0);
        *val += 1;
    }   
    let (&most_common_letter, &most_value) = counter.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    let (&least_common_letter, &least_value) = counter.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();

    println!("Part1: most common - least common: {}", most_value - least_value);

    Ok(())
}
