//! # Solution for Advent of Code 2021 Day 18
//!
//! Ref: [Advent of Code 2021 Day 18](https://adventofcode.com/2021/day/18)
//!

use anyhow::{self, Context};
use std::fmt;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum PairValue {
    Number(i32),
    Pair(Pair),
}
impl PairValue {
    fn parse(iter: &mut Peekable<Chars>) -> anyhow::Result<PairValue> {
        let ch = iter
            .peek()
            .ok_or_else(|| anyhow::anyhow!("Expected a value, not end-of-string"))?;
        match *ch {
            '[' => Ok(PairValue::Pair(Pair::parse(iter)?)),
            '0'..='9' => Ok(PairValue::Number(PairValue::parse_number(iter)?)),
            _ => Err(anyhow::anyhow!("Invalid character for pair: {}", *ch)),
        }
    }
    fn parse_number(iter: &mut Peekable<Chars>) -> anyhow::Result<i32> {
        let mut value: i32 = 0;
        while let Some(ch) = iter.peek() {
            match ch.to_digit(10) {
                Some(val) => {
                    value = value
                        .checked_mul(10)
                        .and_then(|v| v.checked_add(i32::try_from(val).unwrap()))
                        .ok_or_else(|| anyhow::anyhow!("Integer overflow"))?;
                    iter.next();
                }
                None => {
                    break;
                }
            }
        }
        Ok(value)
    }
    fn magnitude(&self) -> i64 {
        match self {
            PairValue::Number(x) => *x as i64,
            PairValue::Pair(p) => p.magnitude(),
        }
    }
}
impl fmt::Display for PairValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PairValue::Number(x) => write!(f, "{x}"),
            PairValue::Pair(p) => write!(f, "{p}"),
        }
    }
}
impl TryFrom<&PairValue> for i32 {
    type Error = anyhow::Error;
    fn try_from(src: &PairValue) -> anyhow::Result<i32> {
        match src {
            PairValue::Number(n) => Ok(*n),
            PairValue::Pair(_) => Err(anyhow::anyhow!("Can't get number from pair {}", src)),
        }
    }
}
#[derive(Debug)]
struct Pair(Box<[PairValue; 2]>);
impl Pair {
    fn parse(iter: &mut Peekable<Chars>) -> anyhow::Result<Pair> {
        let ch = iter.next().ok_or_else(|| anyhow::anyhow!("Empty string"))?;
        if ch != '[' {
            anyhow::bail!("Pairs must start with the ‘[’ character (saw ‘{}’)", ch);
        }

        let left = PairValue::parse(iter)?;

        let separator = iter.next().ok_or_else(|| anyhow::anyhow!("Unterminated pair"))?;
        if separator != ',' {
            anyhow::bail!("Values are separated by the ‘,’ character (saw ‘{}’)", separator);
        }

        let right = PairValue::parse(iter)?;

        let terminator = iter.next().ok_or_else(|| anyhow::anyhow!("Unterminated pair"))?;
        if terminator != ']' {
            anyhow::bail!("Pairs must end with the ‘]’ character (saw ‘{}’)", terminator);
        }

        Ok(Pair(Box::new([left, right])))
    }

    fn magnitude(&self) -> i64 {
        3 * self.0[0].magnitude() + 2 * self.0[1].magnitude()
    }
}
impl TryFrom<&str> for Pair {
    type Error = anyhow::Error;
    fn try_from(src: &str) -> anyhow::Result<Self> {
        let mut iter = src.chars().peekable();
        Pair::parse(&mut iter)
    }
}
impl From<Pair2> for Pair {
    fn from(src: Pair2) -> Self {
        Self::from(&src)
    }
}
impl From<&Pair2> for Pair {
    fn from(src: &Pair2) -> Self {
        let repr = format!("{src}");
        Self::try_from(repr.as_str()).unwrap()
    }
}
impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.0[0], self.0[1])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PairPart {
    Open,
    Close,
    Number(i32),
}
impl fmt::Display for PairPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PairPart::Open => write!(f, "["),
            PairPart::Close => write!(f, "]"),
            PairPart::Number(n) => write!(f, "{n}"),
        }
    }
}
#[derive(Debug, Clone)]
struct Pair2(Vec<PairPart>);
impl TryFrom<&str> for Pair2 {
    type Error = anyhow::Error;
    fn try_from(src: &str) -> anyhow::Result<Self> {
        let p = Pair::try_from(src)?; // Essentially does all the syntax checking
        Ok(Self::from(p))
    }
}
impl From<Pair> for Pair2 {
    fn from(src: Pair) -> Self {
        Self::from(&src)
    }
}
impl From<&Pair> for Pair2 {
    fn from(src: &Pair) -> Self {
        let mut seq: Vec<PairPart> = vec![PairPart::Open];
        for idx in 0..2 {
            match &src.0[idx] {
                PairValue::Number(n) => {
                    seq.push(PairPart::Number(*n));
                }
                PairValue::Pair(p) => {
                    seq.extend(Self::from(p).0);
                }
            }
        }
        seq.push(PairPart::Close);
        Pair2(seq)
    }
}
impl Pair2 {
    fn add(&mut self, other: Pair2) {
        self.0.insert(0, PairPart::Open);
        self.0.extend(other.0);
        self.0.push(PairPart::Close);

        self.reduce()
    }

    fn reduce(&mut self) {
        loop {
            let changes_made = self.explode();
            if changes_made {
                continue;
            }
            let changes_made = self.split();
            if !changes_made {
                break;
            }
        }
    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;
        let mut explode_location = None;
        for (idx, item) in self.0.iter().enumerate() {
            match item {
                PairPart::Open => {
                    depth += 1;
                    if depth == 5 {
                        explode_location = Some(idx);
                        break;
                    }
                }
                PairPart::Close => {
                    depth -= 1;
                }
                PairPart::Number(_) => {}
            }
        }

        if let Some(idx) = explode_location {
            // this one explodes.
            // self.0[idx..=idx+3]: [ left, right ]
            let left = match &self.0[idx + 1] {
                PairPart::Number(x) => *x,
                _ => unreachable!(),
            };
            let right = match &self.0[idx + 2] {
                PairPart::Number(x) => *x,
                _ => unreachable!(),
            };

            // Find previous number
            let prev_location = self.0[0..idx]
                .iter()
                .enumerate()
                .rfind(|&(_, part)| matches!(part, &PairPart::Number(_)))
                .map(|(idx, _)| idx);
            // Find next number
            let next_location = self.0[idx + 4..]
                .iter()
                .enumerate()
                .find(|&(_, part)| matches!(part, &PairPart::Number(_)))
                .map(|(n, _)| n + idx + 4);
            // Adjust previous number
            if let Some(idx) = prev_location {
                if let PairPart::Number(value) = self.0.get_mut(idx).unwrap() {
                    *value += left;
                }
            }
            // Adjust next number
            if let Some(idx) = next_location {
                if let PairPart::Number(value) = self.0.get_mut(idx).unwrap() {
                    *value += right;
                }
            }

            // replace idx..idx+4 with number(0)
            self.0[idx] = PairPart::Number(0);
            // Remove the next 3 elements
            self.0.copy_within(idx + 4.., idx + 1);
            self.0.truncate(self.0.len() - 3);

            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        let split_data = self.0.iter().enumerate().find_map(|(a, b)| {
            if let PairPart::Number(x) = b {
                if *x >= 10 {
                    Some((a, *x))
                } else {
                    None
                }
            } else {
                None
            }
        });

        match split_data {
            Some((idx, val)) => {
                self.0.insert(idx + 1, PairPart::Close);
                self.0.insert(idx + 1, PairPart::Number((val + 1) / 2));
                self.0.insert(idx + 1, PairPart::Number(val / 2));
                self.0[idx] = PairPart::Open;
                true
            }
            None => false,
        }
    }

    fn magnitude(&self) -> i64 {
        Pair::from(self).magnitude()
    }
}

impl fmt::Display for Pair2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut previous = None;
        for item in self.0.iter() {
            match item {
                PairPart::Number(_) | PairPart::Open => {
                    if !matches!(previous, None | Some(&PairPart::Open)) {
                        write!(f, ",")?;
                    }
                }
                PairPart::Close => {}
            }
            write!(f, "{item}")?;
            previous = Some(item);
        }
        Ok(())
    }
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(|r| r.map_err(anyhow::Error::from))
        .map(|r| r.and_then(|s| Pair2::try_from(s.as_str())))
        .collect::<anyhow::Result<Vec<_>>>()
        .and_then(|v| {
            if v.is_empty() {
                Err(anyhow::anyhow!("Must have at least one pair"))
            } else {
                Ok(v)
            }
        })
        .context("Failed to parse puzzle input from stdin")?;

    // Part 1: add all those up, get the magnitude.
    let mut sum = input[0].clone();
    for other in input[1..].iter().cloned() {
        sum.add(other);
    }
    println!("Part 1: Sum results in magnitude {}", sum.magnitude());

    let count = input.len();
    let mut max_magnitude = 0;
    for outer in 0..count {
        for inner in 0..count {
            if inner != outer {
                let mut sn_left = input[outer].clone();
                let sn_right = input[inner].clone();
                sn_left.add(sn_right);
                let mag = sn_left.magnitude();
                if mag > max_magnitude {
                    max_magnitude = mag;
                }
            }
        }
    }
    println!("Part 2: pairwise largest magnitude: {max_magnitude}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("[[[[[9,8],1],2],3],4]" => "[[[[0,9],2],3],4]"; "explode example 1")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]" => "[7,[6,[5,[7,0]]]]"; "explode example 2")]
    #[test_case("[[6,[5,[4,[3,2]]]],1]" => "[[6,[5,[7,0]]],3]"; "explode example 3")]
    #[test_case("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]" => "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"; "explode examples 4&5")]
    #[test_case("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]" => "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"; "explode+split example")]
    fn reduce(src: &str) -> String {
        let mut input = Pair2::try_from(src).unwrap();
        input.reduce();
        format!("{input}")
    }

    #[test_case(&["[1,1]","[2,2]","[3,3]","[4,4]"] => "[[[[1,1],[2,2]],[3,3]],[4,4]]"; "add example 1")]
    #[test_case(&["[1,1]","[2,2]","[3,3]","[4,4]","[5,5]"] => "[[[[3,0],[5,3]],[4,4]],[5,5]]"; "add example 2")]
    #[test_case(&["[1,1]","[2,2]","[3,3]","[4,4]","[5,5]","[6,6]"] => "[[[[5,0],[7,4]],[5,5]],[6,6]]"; "add example 3")]
    #[test_case(&[
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        "[7,[5,[[3,8],[1,4]]]]",
        "[[2,[2,2]],[8,[8,1]]]",
        "[2,9]",
        "[1,[[[9,3],9],[[9,0],[0,7]]]]",
        "[[[5,[7,4]],7],1]",
        "[[[[4,2],2],6],[8,7]]",
    ] => "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"; "big add example")]
    #[test_case(&[
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ] => "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"; "magnitude example")]
    fn add(src: &[&str]) -> String {
        let mut sn = Pair2::try_from(src[0]).unwrap();
        for other in src[1..].iter() {
            sn.add(Pair2::try_from(*other).unwrap());
        }
        format!("{sn}")
    }
}
