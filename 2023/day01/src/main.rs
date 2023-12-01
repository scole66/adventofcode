//! # Solution for Advent of Code 2023 Day 1: Trebuchet?!
//!
//! Ref: [Advent of Code 2023 Day 1](https://adventofcode.com/2023/day/1)
//!
use once_cell::sync::Lazy;
use regex::Regex;
use std::char;
use std::io::{self, Read};

fn decode(line: &str) -> u32 {
    let mut left = 100;
    let mut most_recent = 100;

    line.chars().filter(char::is_ascii_digit).for_each(|c| {
        if left >= 100 {
            left = c.to_digit(10).unwrap();
        }
        most_recent = c.to_digit(10).unwrap();
    });
    left * 10 + most_recent
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
    static TWO_DIGIT_PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine).*([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap()
    });
    static ONE_DIGIT_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap());
    TWO_DIGIT_PATTERN
        .captures(line)
        .map(|caps| 10 * to_digit(caps.get(1).unwrap().as_str()) + caps.get(2).map(|m| to_digit(m.as_str())).unwrap())
        .unwrap_or_else(|| {
            ONE_DIGIT_PATTERN
                .captures(line)
                .map(|caps| to_digit(caps.get(1).unwrap().as_str()) * 11)
                .unwrap_or(0)
        })
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
