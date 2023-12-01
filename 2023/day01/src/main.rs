//! # Solution for Advent of Code 2023 Day 1: Trebuchet?!
//!
//! Ref: [Advent of Code 2023 Day 1](https://adventofcode.com/2023/day/1)
//!
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};

fn decode(line: &str) -> u32 {
    let (_, first, last) = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold((false, 0, 0), |(first_captured, first, _), next| {
            (true, if first_captured { first } else { next }, next)
        });
    first * 10 + last
}

fn to_digit(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => unreachable!(),
    }
}

fn decode_2(line: &str) -> u32 {
    static DIGIT_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap());
    static TRAILING_DIGIT_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r".*([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap());
    let left = DIGIT_PATTERN
        .captures(line)
        .map(|caps| to_digit(caps.get(1).unwrap().as_str()))
        .unwrap_or(0);
    let right = TRAILING_DIGIT_PATTERN
        .captures(line)
        .map(|caps| to_digit(caps.get(1).unwrap().as_str()))
        .unwrap_or(0);
    left * 10 + right
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    let part1 = input.lines().map(decode).sum::<u32>();

    println!("Part1: {part1}");

    let part2 = input.lines().map(decode_2).sum::<u32>();

    println!("Part2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = indoc::indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        let result = input.lines().map(decode).sum::<u32>();
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_sample() {
        let input = indoc::indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        let result = input.lines().map(decode_2).sum::<u32>();
        assert_eq!(result, 281);
    }

    #[test]
    fn twone() {
        assert_eq!(decode_2("twone"), 21);
    }
}
