//! # Solution for Advent of Code 2021 Day 10
//!
//! Ref: [Advent of Code 2021 Day 10](https://adventofcode.com/2021/day/10)
//!

use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq)]
enum LineResult {
    Incomplete(String),
    Corrupted(char),
}

fn corresponding(ch: char) -> char {
    match ch {
        ')' => '(',
        '}' => '{',
        '>' => '<',
        ']' => '[',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        '[' => ']',
        _ => ch,
    }
}

fn parse_line(s: impl AsRef<str>) -> LineResult {
    let mut processor: Vec<char> = vec![];
    for ch in s.as_ref().chars() {
        match ch {
            '(' | '{' | '<' | '[' => {
                processor.push(ch);
            }
            ')' | '}' | '>' | ']' => {
                if processor[processor.len() - 1] != corresponding(ch) {
                    return LineResult::Corrupted(ch);
                } else {
                    processor.pop();
                }
            }
            _ => {
                return LineResult::Corrupted(ch);
            }
        }
    }
    let mut result = String::new();
    while let Some(ch) = processor.pop() {
        result.push(corresponding(ch));
    }
    LineResult::Incomplete(result)
}

fn corrupted_points(res: &LineResult) -> u64 {
    match res {
        LineResult::Corrupted(')') => 3,
        LineResult::Corrupted(']') => 57,
        LineResult::Corrupted('}') => 1197,
        LineResult::Corrupted('>') => 25137,
        _ => 0,
    }
}

fn incomplete_points(res: LineResult) -> u64 {
    if let LineResult::Incomplete(finishers) = res {
        finishers.chars().fold(0, |accum, x| {
            accum * 5
                + match x {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
        })
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(|res| res.ok()).collect::<Vec<String>>();

    let points: u64 = lines.iter().map(|s| corrupted_points(&parse_line(s))).sum();
    println!("Part1: {points} syntax error points");

    let mut incomplete_points: Vec<u64> = lines
        .iter()
        .map(parse_line)
        .filter(|lr| matches!(lr, &LineResult::Incomplete(_)))
        .map(incomplete_points)
        .collect();
    incomplete_points.sort_unstable();
    println!(
        "Part2: incomplete score (middle): {}",
        incomplete_points[incomplete_points.len() / 2]
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn corrupted_points() {
        let p: u64 = SAMPLE.iter().map(|s| super::corrupted_points(&parse_line(s))).sum();
        assert_eq!(p, 26397);
    }
    #[test]
    fn parsing() {
        let results: Vec<LineResult> = SAMPLE.iter().map(parse_line).collect();
        assert_eq!(
            results,
            vec![
                LineResult::Incomplete("}}]])})]".to_string()),
                LineResult::Incomplete(")}>]})".to_string()),
                LineResult::Corrupted('}'),
                LineResult::Incomplete("}}>}>))))".to_string()),
                LineResult::Corrupted(')'),
                LineResult::Corrupted(']'),
                LineResult::Incomplete("]]}}]}]}>".to_string()),
                LineResult::Corrupted(')'),
                LineResult::Corrupted('>'),
                LineResult::Incomplete("])}>".to_string())
            ]
        );
    }
    #[test]
    fn incomplete_points() {
        let p: Vec<u64> = SAMPLE
            .iter()
            .map(|s| super::incomplete_points(parse_line(s)))
            .filter(|&n| n > 0)
            .collect();
        assert_eq!(p, vec![288957, 5566, 1480781, 995444, 294]);
    }
}
