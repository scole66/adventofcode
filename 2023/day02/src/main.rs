//! # Solution for Advent of Code 2023 Day 2: Cube Conundrum
//!
//! Ref: [Advent of Code 2023 Day 2](https://adventofcode.com/2023/day/2)
//!
use anyhow::{bail, Error, Result};
use std::cmp::max;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Kind {
    Red,
    Green,
    Blue,
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => bail!("invalid Kind: {s}"),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Presentation {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Presentation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|piece| match piece.trim().split_once(' ') {
                None => bail!("invalid presentation: {piece}"),
                Some((snum, kind)) => {
                    let num = snum.parse::<u32>()?;
                    let kind = kind.parse::<Kind>()?;
                    match kind {
                        Kind::Red => Ok((num, 0, 0)),
                        Kind::Green => Ok((0, num, 0)),
                        Kind::Blue => Ok((0, 0, num)),
                    }
                }
            })
            .try_fold((0, 0, 0), |(ir, ig, ib), res| {
                let (r, g, b) = res?;
                Ok::<(u32, u32, u32), Self::Err>((ir + r, ig + g, ib + b))
            })
            .map(|(red, green, blue)| Presentation { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    presentations: Vec<Presentation>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            None => bail!("invalid game: {s}"),
            Some((head, tail)) => Ok(Game {
                id: match head.split_once(' ') {
                    None => bail!("invalid game head: {head}"),
                    Some((tag, id)) => {
                        if tag != "Game" {
                            bail!("invalid game tag: {tag}");
                        }
                        id.parse::<u32>()?
                    }
                },
                presentations: tail
                    .split(';')
                    .map(|s| s.parse::<Presentation>())
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
        }
    }
}

impl Game {
    fn valid(&self, red_limit: u32, green_limit: u32, blue_limit: u32) -> bool {
        self.presentations
            .iter()
            .all(|Presentation { red, green, blue }| *red <= red_limit && *green <= green_limit && *blue <= blue_limit)
    }
    fn id(&self) -> usize {
        self.id as usize
    }
    fn power(&self) -> usize {
        let (max_red, max_green, max_blue) = self.presentations.iter().fold(
            (0, 0, 0),
            |(red_a, green_a, blue_a), Presentation { red, green, blue }| {
                (max(red_a, *red), max(green_a, *green), max(blue_a, *blue))
            },
        );
        max_red as usize * max_green as usize * max_blue as usize
    }
}

fn part1(games: &[Game]) -> usize {
    games
        .iter()
        .filter(|&g| g.valid(12, 13, 14))
        .map(Game::id)
        .sum::<usize>()
}

fn part2(games: &[Game]) -> usize {
    games.iter().map(Game::power).sum::<usize>()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let games = input
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<Game>>>()?;

    println!("Part1: {}", part1(&games));
    println!("Part2: {}", part2(&games));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part1_sample() {
        let games = SAMPLE
            .lines()
            .map(|line| line.parse::<Game>())
            .collect::<Result<Vec<Game>>>()
            .unwrap();
        assert_eq!(part1(&games), 8);
    }

    #[test]
    fn part2_sample() {
        let games = SAMPLE
            .lines()
            .map(|line| line.parse::<Game>())
            .collect::<Result<Vec<Game>>>()
            .unwrap();
        assert_eq!(part2(&games), 2286);
    }

    #[test_case("3 red, 6 blue, 20 green" => Ok(Presentation{ red: 3, green: 20, blue: 6 }); "all 3")]
    #[test_case("something entirely different" => Err("invalid digit found in string".to_string()); "some error")]
    fn presentation_from_str(input: &str) -> Result<Presentation, String> {
        input.parse::<Presentation>().map_err(|e| e.to_string())
    }
}
