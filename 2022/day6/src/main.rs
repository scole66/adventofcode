//! # Solution for Advent of Code 2022 Day 6: Supply Stacks
//!
//! Ref: [Advent of Code 2022 Day 6](https://adventofcode.com/2022/day/6)
//!
use ahash::AHashSet;
use itertools::Itertools;
use std::io::{self, Read};

fn part1(input: &str) -> usize {
    for (idx, (a, b, c, d)) in input.chars().tuple_windows::<(_, _, _, _)>().enumerate() {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return idx + 4;
        }
    }

    0
}

fn part2(input: &str) -> usize {
    for start_idx in 0..input.len() - 14 {
        let sample = &input[start_idx..start_idx + 14];
        let chset = AHashSet::from_iter(sample.chars());
        if chset.len() == 14 {
            return start_idx + 14;
        }
    }

    0
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    static SAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    static SAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    static SAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    static SAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test_case(SAMPLE1 => 7)]
    #[test_case(SAMPLE2 => 5)]
    #[test_case(SAMPLE3 => 6)]
    #[test_case(SAMPLE4 => 10)]
    #[test_case(SAMPLE5 => 11)]
    fn part1_sample(text: &str) -> usize {
        part1(text)
    }

    #[test_case(SAMPLE1 => 19)]
    #[test_case(SAMPLE2 => 23)]
    #[test_case(SAMPLE3 => 23)]
    #[test_case(SAMPLE4 => 29)]
    #[test_case(SAMPLE5 => 26)]
    fn part2_sample(text: &str) -> usize {
        part2(text)
    }
}
