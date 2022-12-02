//! # Solution for Advent of Code 2022 Day 2
//!
//! Ref: [Advent of Code 2022 Day 2](https://adventofcode.com/2022/day/2)
//!

use anyhow::{self, Context};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};
use thiserror::Error;

struct ResultString(anyhow::Result<String>);
impl From<Result<String, std::io::Error>> for ResultString {
    /// Converts a `Result<String, std::io::Error>` into a `ResultString`
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}

#[derive(Debug)]
struct Strategy {
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    prompt: Choice,
    response: Response,
}

#[derive(Debug, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum Response {
    RockLose,
    PaperDraw,
    ScissorsWin,
}

#[derive(Debug, Error)]
enum RPSError {
    #[error("Not a valid choice selector")]
    BadSelector,
    #[error("Bad step format")]
    BadStep,
}

impl TryFrom<char> for Choice {
    type Error = RPSError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Choice::Rock),
            'B' => Ok(Choice::Paper),
            'C' => Ok(Choice::Scissors),
            _ => Err(RPSError::BadSelector),
        }
    }
}

impl TryFrom<char> for Response {
    type Error = RPSError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Response::RockLose),
            'Y' => Ok(Response::PaperDraw),
            'Z' => Ok(Response::ScissorsWin),
            _ => Err(RPSError::BadSelector),
        }
    }
}

impl TryFrom<&str> for Step {
    type Error = RPSError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static STEP_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(?P<prompt>[ABC]) (?P<response>[XYZ])$").expect("Hand-rolled regex is valid"));
        let captures = STEP_PATTERN.captures(value).ok_or(RPSError::BadStep)?;
        let prompt = Choice::try_from(
            captures
                .name("prompt")
                .expect("Regex guarantees a match")
                .as_str()
                .chars()
                .next()
                .expect("Regex guarantees a one char match"),
        )
        .expect("Regex guarantees a legit value");
        let response = Response::try_from(
            captures
                .name("response")
                .expect("Regex guarantees a match")
                .as_str()
                .chars()
                .next()
                .expect("Regex guarantees a one char match"),
        )
        .expect("Regex guarantees a legit value");
        Ok(Step { prompt, response })
    }
}

impl FromIterator<ResultString> for anyhow::Result<Strategy> {
    fn from_iter<T: IntoIterator<Item = ResultString>>(iter: T) -> Self {
        let mut strategy = vec![];
        for ResultString(res) in iter.into_iter() {
            let line = res?;
            let step = Step::try_from(line.as_str())?;
            strategy.push(step);
        }
        Ok(Strategy { steps: strategy })
    }
}

enum GameStyle {
    Naive,
    Sophisticated,
}

impl Choice {
    fn value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl Step {
    fn score(&self, style: GameStyle) -> i32 {
        let response = match style {
            GameStyle::Naive => match &self.response {
                Response::PaperDraw => Choice::Paper,
                Response::RockLose => Choice::Rock,
                Response::ScissorsWin => Choice::Scissors,
            },
            GameStyle::Sophisticated => match (&self.prompt, &self.response) {
                (Choice::Rock, Response::PaperDraw) => Choice::Rock,
                (Choice::Rock, Response::RockLose) => Choice::Scissors,
                (Choice::Rock, Response::ScissorsWin) => Choice::Paper,
                (Choice::Paper, Response::PaperDraw) => Choice::Paper,
                (Choice::Paper, Response::RockLose) => Choice::Rock,
                (Choice::Paper, Response::ScissorsWin) => Choice::Scissors,
                (Choice::Scissors, Response::RockLose) => Choice::Paper,
                (Choice::Scissors, Response::PaperDraw) => Choice::Scissors,
                (Choice::Scissors, Response::ScissorsWin) => Choice::Rock,
            },
        };
        self.outcome(response) + response.value()
    }

    fn outcome(&self, response: Choice) -> i32 {
        match (&self.prompt, response) {
            (Choice::Rock, Choice::Rock) => 3,
            (Choice::Rock, Choice::Paper) => 6,
            (Choice::Rock, Choice::Scissors) => 0,
            (Choice::Paper, Choice::Rock) => 0,
            (Choice::Paper, Choice::Paper) => 3,
            (Choice::Paper, Choice::Scissors) => 6,
            (Choice::Scissors, Choice::Rock) => 6,
            (Choice::Scissors, Choice::Paper) => 0,
            (Choice::Scissors, Choice::Scissors) => 3,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let strategy = stdin
        .lock()
        .lines()
        .map(ResultString::from)
        .collect::<anyhow::Result<Strategy>>()
        .context("Failed to parse puzzle input from stdin")?;

    // Part 1: Run the naive strategy
    let naive: i32 = strategy.steps.iter().map(|x| x.score(GameStyle::Naive)).sum();
    println!("Part1 score: {naive}");

    // Part 2: Run the sophisticated strategy
    let sophisticated: i32 = strategy.steps.iter().map(|x| x.score(GameStyle::Sophisticated)).sum();
    println!("Part2 score: {sophisticated}");

    Ok(())
}
