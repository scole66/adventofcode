//! # Solution for Advent of Code 2021 Day 14
//!
//! Ref: [Advent of Code 2021 Day 14](https://adventofcode.com/2021/day/14)
//!

use ahash::AHashMap;
use anyhow::{self, Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead};

/// One rule
///
/// This is a one-to-one transformation from the input. A line like:
/// ```text
///     AB -> C
/// ```
///  means that every pair `AB` becomes the triple `ACB`. But that's not quite how we store it; it turns out to be
///  better for us if we "prepare" new pairs ahead of time. So rather than repreenting the right hand side as `C` (or
///  even `ACB`), we store it as `["AC", "CB"]`.
#[derive(Debug)]
struct Rule {
    leftright: String,
    newpairs: [String; 2],
}

impl Rule {
    /// Attempt to convert one input line into a pair insertion rule
    ///
    /// Lines should look like
    /// ```text
    /// XY -> Z
    /// ```
    /// That is: two chars, a blank, a 2-character arrow, another blank, and then one more character. If they don't, an
    /// error is returned.
    ///
    /// The newline char is reserved for internal use; but the rest of Unicode is available. In particular,
    /// ```text
    /// 🐔🐓 -> 🐣
    /// ```
    /// actually works.
    fn parse(line: &str) -> anyhow::Result<Rule> {
        lazy_static! {
            static ref RULE_PATTERN: Regex = Regex::new("^(?P<left>.)(?P<right>.) -> (?P<insertion>.)$").unwrap();
        }
        let captures = RULE_PATTERN
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("‘{}’ is not a valid rule", line))?;
        let insertion = captures.name("insertion").unwrap().as_str().chars().next().unwrap();
        let left = captures.name("left").unwrap().as_str().chars().next().unwrap();
        let right = captures.name("right").unwrap().as_str().chars().next().unwrap();
        Ok(Rule {
            leftright: [left, right].iter().collect::<String>(),
            newpairs: [
                [left, insertion].iter().collect::<String>(),
                [insertion, right].iter().collect::<String>(),
            ],
        })
    }
}

/// Template start/end marker
const BOOKEND: char = '\n'; // newlines don't generally appear _within_ lines.

/// The state of the system
///
/// Ultimately, the state here is the count of letter pairs. Which is all this structure really is.
#[derive(Debug)]
struct PairCounts(AHashMap<String, i64>);
impl From<String> for PairCounts {
    /// Count the pairs in a String
    ///
    /// This is how we get the initial state of the polymer from the input template string. Note that the map contains
    /// pairs with "Bookends"; these are "imaginary" pairs that help with the final tallying.
    fn from(src: String) -> Self {
        let mut prior = BOOKEND;
        let mut map: AHashMap<String, i64> = AHashMap::new();
        for ch in src.chars().chain(String::from(BOOKEND).chars()) {
            let key = [prior, ch].iter().collect::<String>();
            let count = map.entry(key).or_insert(0);
            *count += 1;
            prior = ch;
        }

        Self(map)
    }
}
impl PairCounts {
    /// Count the individual letters that make up all the pairs
    ///
    /// This counts up all the individual characters in the pairs mentioned in the pair counts to return how many
    /// actual letters there are in a [LetterCounts] map. Any [BOOKEND] characters are not counted.
    ///
    /// Example:
    /// ```
    /// let pairs = PairCounts::from("NNBNCS");
    /// let counts = pairs.counts();
    /// assert_eq!(counts.0, AHashMap::from([('N', 3), ('B', 1), ('C', 1), ('S', 1)]));
    /// ```
    fn counts(&self) -> LetterCounts {
        let mut map = AHashMap::<char, i64>::new();
        for (key, value) in self.0.iter() {
            for ch in key.chars() {
                let counter = map.entry(ch).or_insert(0);
                *counter += value;
            }
        }
        // That counted everything twice, so reduce from there, and remove the entry for the BOOKEND.
        map.remove(&BOOKEND);
        for value in map.values_mut() {
            *value /= 2;
        }
        LetterCounts(map)
    }
}
struct LetterCounts(AHashMap<char, i64>);
impl LetterCounts {
    fn most_frequent(&self) -> Option<(char, i64)> {
        self.0.iter().max_by(|x, y| x.1.cmp(y.1)).map(|(c, v)| (*c, *v))
    }
    fn least_frequent(&self) -> Option<(char, i64)> {
        self.0.iter().min_by(|x, y| x.1.cmp(y.1)).map(|(c, v)| (*c, *v))
    }
}

#[derive(Debug)]
struct Rules(AHashMap<String, [String; 2]>);

impl Rules {
    fn apply(&self, state: &mut PairCounts) -> anyhow::Result<()> {
        let entries = state.0.iter().map(|(s, v)| (s.clone(), *v)).collect::<Vec<_>>();
        for (key, count) in entries {
            if key.contains(BOOKEND) {
                continue;
            }
            let pair = self
                .0
                .get(&key)
                .ok_or_else(|| anyhow::anyhow!("No rule for pair {}", key))?;
            *state.0.entry(key).or_insert(0) -= count;
            *state.0.entry(pair[0].clone()).or_insert(0) += count;
            *state.0.entry(pair[1].clone()).or_insert(0) += count;
        }
        Ok(())
    }
}

/// Processed input data
///
/// This structure holds representations of the input data, lightly processed into forms that are useful for the
/// calculations required to find the puzzle solutions.
///
/// Two fields here:
/// * `template` is the String the puzzle refers to as the "polymer template".
/// * `rules` is a representation (as a hash map) of the "pair insertion rules".
#[derive(Debug)]
struct Data {
    template: String,
    rules: Rules,
}

/// A NewType wrapping an `anyhow::Result<String>`
///
/// This is really nothing more than a new type created so that we can implement what would otherwise be
/// `FromIterator<anyhow::Result<String>> for anyhow::Result<Data>`.
#[derive(Debug)]
struct ResultStringWrap(anyhow::Result<String>);
impl From<anyhow::Result<String>> for ResultStringWrap {
    /// Converts an `anyhow::Result<String>` into a `ResultStringWrap`
    fn from(src: anyhow::Result<String>) -> Self {
        Self(src)
    }
}
impl From<Result<String, std::io::Error>> for ResultStringWrap {
    /// Converts a `Result<String, std::io::Error>` into a `ResultStringWrap`
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}

impl FromIterator<ResultStringWrap> for anyhow::Result<Data> {
    fn from_iter<I: IntoIterator<Item = ResultStringWrap>>(iter: I) -> Self {
        let mut loading_rules = false;
        let mut template: Option<String> = None;
        let mut rules: AHashMap<String, [String; 2]> = AHashMap::new();
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
                rules.insert(rule.leftright, rule.newpairs);
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
    let mut polymer = PairCounts::from(input.template.clone());
    for _ in 0..10 {
        input.rules.apply(&mut polymer)?;
    }

    let counts = polymer.counts();
    let most_value = counts.most_frequent().unwrap().1;
    let least_value = counts.least_frequent().unwrap().1;

    println!("Part1: most common - least common: {}", most_value - least_value);

    for _ in 10..40 {
        input.rules.apply(&mut polymer)?;
    }

    let counts = polymer.counts();
    let most_value = counts.most_frequent().unwrap().1;
    let least_value = counts.least_frequent().unwrap().1;

    println!("Part2: most common - least common: {}", most_value - least_value);
    Ok(())
}
