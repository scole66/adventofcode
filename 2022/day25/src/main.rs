//! # Solution for Advent of Code 2022 Day 25: Full of Hot Air
//!
//! Ref: [Advent of Code 2022 Day 25](https://adventofcode.com/2022/day/25)
//!
use anyhow::{anyhow, Error, Result};
use std::fmt::Display;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

enum SnafuDigit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}
impl Display for SnafuDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnafuDigit::DoubleMinus => '=',
            SnafuDigit::Minus => '-',
            SnafuDigit::Zero => '0',
            SnafuDigit::One => '1',
            SnafuDigit::Two => '2',
        }
        .fmt(f)
    }
}
impl TryFrom<char> for SnafuDigit {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '=' => Ok(SnafuDigit::DoubleMinus),
            '-' => Ok(SnafuDigit::Minus),
            '0' => Ok(SnafuDigit::Zero),
            '1' => Ok(SnafuDigit::One),
            '2' => Ok(SnafuDigit::Two),
            _ => Err(anyhow!("Not a SNAFU digit: `{value}`")),
        }
    }
}
impl From<&SnafuDigit> for isize {
    fn from(value: &SnafuDigit) -> Self {
        match value {
            SnafuDigit::DoubleMinus => -2,
            SnafuDigit::Minus => -1,
            SnafuDigit::Zero => 0,
            SnafuDigit::One => 1,
            SnafuDigit::Two => 2,
        }
    }
}
struct Snafu {
    digits: Vec<SnafuDigit>,
}
impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu {
            digits: s.chars().map(SnafuDigit::try_from).collect::<Result<Vec<_>>>()?,
        })
    }
}
impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.digits.iter() {
            digit.fmt(f)?;
        }
        Ok(())
    }
}
impl From<&Snafu> for isize {
    fn from(value: &Snafu) -> Self {
        let mut result = 0;
        for digit in value.digits.iter() {
            let value = isize::from(digit);
            result = result * 5 + value;
        }
        result
    }
}
impl From<Snafu> for isize {
    fn from(value: Snafu) -> Self {
        isize::from(&value)
    }
}
impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        let mut result = vec![];
        let mut remaining = value;
        while remaining != 0 {
            let remainder = remaining.rem_euclid(5);
            match remainder {
                0 => {
                    result.push(SnafuDigit::Zero);
                    remaining = remaining.div_euclid(5);
                }
                1 => {
                    result.push(SnafuDigit::One);
                    remaining = remaining.div_euclid(5);
                }
                2 => {
                    result.push(SnafuDigit::Two);
                    remaining = remaining.div_euclid(5);
                }
                3 => {
                    result.push(SnafuDigit::DoubleMinus);
                    remaining = remaining.div_euclid(5) + 1;
                }
                4 => {
                    result.push(SnafuDigit::Minus);
                    remaining = remaining.div_euclid(5) + 1;
                }
                _ => unreachable!(),
            }
        }
        Snafu {
            digits: result.into_iter().rev().collect(),
        }
    }
}

fn part1(input: &str) -> anyhow::Result<String> {
    let numbers = input
        .lines()
        .map(|line| line.parse::<Snafu>())
        .collect::<Result<Vec<_>>>()?;
    let sum = numbers.iter().map(isize::from).sum::<isize>();
    Ok(Snafu::from(sum).to_string())
}

//fn part2(input: &str) -> anyhow::Result<usize> {
//    todo!()
//}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    //println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), "2=-1=0");
    }

    //#[test]
    //#[should_panic]
    //fn part2_sample() {
    //    assert_eq!(part2(SAMPLE).unwrap(), 36);
    //}

    mod snafu {
        use super::*;
        use test_case::test_case;

        #[test_case("1" => 1)]
        #[test_case("2" => 2)]
        #[test_case("1=" => 3)]
        #[test_case("1-" => 4)]
        #[test_case("10" => 5)]
        #[test_case("11" => 6)]
        #[test_case("12" => 7)]
        #[test_case("2=" => 8)]
        #[test_case("2-" => 9)]
        #[test_case("20" => 10)]
        #[test_case("1=0" => 15)]
        #[test_case("1-0" => 20)]
        #[test_case("1=11-2" => 2022)]
        #[test_case("1-0---0" => 12345)]
        #[test_case("1121-1110-1=0" => 314159265)]
        fn to_isize(snafu: &str) -> isize {
            let s = snafu.parse::<Snafu>().unwrap();
            isize::from(s)
        }

        #[test_case(1 => "1".to_string())]
        #[test_case(2 => "2".to_string())]
        #[test_case(3 => "1=".to_string())]
        #[test_case(4 => "1-".to_string())]
        #[test_case(5 => "10".to_string())]
        #[test_case(6 => "11".to_string())]
        #[test_case(7 => "12".to_string())]
        #[test_case(8 => "2=".to_string())]
        #[test_case(9 => "2-".to_string())]
        #[test_case(10 => "20".to_string())]
        #[test_case(15 => "1=0".to_string())]
        #[test_case(20 => "1-0".to_string())]
        #[test_case(2022 => "1=11-2".to_string())]
        #[test_case(12345 => "1-0---0".to_string())]
        #[test_case(314159265 => "1121-1110-1=0".to_string())]
        fn from_isize(val: isize) -> String {
            Snafu::from(val).to_string()
        }
    }
}
