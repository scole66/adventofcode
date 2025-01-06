//! # Solution for Advent of Code 2023 Day 1: Trebuchet?!
//!
//! Ref: [Advent of Code 2023 Day 1](https://adventofcode.com/2023/day/1)
//!
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

fn to_digit(s: &str) -> Option<u32> {
    match s {
        x if x.starts_with('0') || x.starts_with("zero") => Some(0),
        x if x.starts_with('1') || x.starts_with("one") => Some(1),
        x if x.starts_with('2') || x.starts_with("two") => Some(2),
        x if x.starts_with('3') || x.starts_with("three") => Some(3),
        x if x.starts_with('4') || x.starts_with("four") => Some(4),
        x if x.starts_with('5') || x.starts_with("five") => Some(5),
        x if x.starts_with('6') || x.starts_with("six") => Some(6),
        x if x.starts_with('7') || x.starts_with("seven") => Some(7),
        x if x.starts_with('8') || x.starts_with("eight") => Some(8),
        x if x.starts_with('9') || x.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn decode_2(line: &str) -> u32 {
    let (_, left, right) = line
        .char_indices()
        .filter_map(|(idx, _)| to_digit(&line[idx..]))
        .fold((false, 0, 0), |(first_captured, first, _), next| {
            (true, if first_captured { first } else { next }, next)
        });
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
