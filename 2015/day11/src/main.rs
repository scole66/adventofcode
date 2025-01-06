//! # Solution for Advent of Code 2015 Day 11: Corporate Policy
//!
//! Ref: [Advent of Code 2015 Day 11](https://adventofcode.com/2015/day/11)
//!
use anyhow::{anyhow, Error, Result};
use std::fmt::Display;
use std::io::{self, Read};
use std::str::FromStr;

struct Password {
    text: String,
}
impl FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.len() == 8 && trimmed.chars().all(|c| c.is_ascii_lowercase()) {
            Ok(Password {
                text: trimmed.to_string(),
            })
        } else {
            Err(anyhow!("Invalid password string \"{trimmed}\""))
        }
    }
}
impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.text.fmt(f)
    }
}
impl Password {
    fn is_valid(&self) -> bool {
        self.text
            .as_bytes()
            .windows(3)
            .any(|bytes| bytes[1] == bytes[0] + 1 && bytes[2] == bytes[1] + 1)
            && !self.text.as_bytes().contains(&b'i')
            && !self.text.as_bytes().contains(&b'l')
            && !self.text.as_bytes().contains(&b'o')
            && self
                .text
                .as_bytes()
                .windows(2)
                .enumerate()
                .filter_map(|(idx, bytes)| (bytes[0] == bytes[1]).then_some(idx))
                .scan(None, |state, idx| {
                    let rval = state.map(|old_idx| idx >= old_idx + 2).unwrap_or(true).then_some(idx);
                    *state = Some(idx);
                    Some(rval)
                })
                .flatten()
                .count()
                >= 2
    }

    fn next(&self) -> Option<Self> {
        let mut work = String::with_capacity(8);
        let mut carry = 1;
        for ch in self.text.chars().rev() {
            assert!(carry == 0 || carry == 1);
            match ch {
                'a'..='y' => {
                    work.push((ch as u8 + carry) as char);
                    carry = 0;
                }
                'z' => {
                    work.push(if carry == 0 { 'z' } else { 'a' });
                }
                _ => unreachable!(),
            }
        }
        if carry == 1 {
            None
        } else {
            Some(Password {
                text: work.chars().rev().collect(),
            })
        }
    }

    fn next_valid(&self) -> Option<Password> {
        let mut attempt = self.next();
        loop {
            if attempt.as_ref().map(|x| x.is_valid()).unwrap_or(true) {
                return attempt;
            }
            attempt = attempt.as_ref().and_then(|x| x.next());
        }
    }
}

fn part1(input: &str) -> Result<String> {
    let pw = input.parse::<Password>()?;

    let next_password = pw.next_valid();

    next_password
        .map(|pw| pw.to_string())
        .ok_or_else(|| anyhow!("Reached the last password, there's nothing left."))
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    let pw1 = part1(&input)?;
    println!("Part1: {pw1}");
    println!("Part2: {}", part1(&pw1)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aaaaaaaa" => Some("aaaaaaab".to_string()))]
    #[test_case("bbbaazzz" => Some("bbbabaaa".to_string()))]
    #[test_case("zzzzzzzz" => None)]
    fn next(src: &str) -> Option<String> {
        let pw = src.parse::<Password>().unwrap();
        pw.next().map(|s| s.to_string())
    }

    #[test_case("abcdefgh" => Some("abcdffaa".to_string()))]
    #[test_case("ghijklmn" => Some("ghjaabcc".to_string()))]
    fn next_valid(src: &str) -> Option<String> {
        let pw = src.parse::<Password>().unwrap();
        pw.next_valid().map(|s| s.to_string())
    }
}
