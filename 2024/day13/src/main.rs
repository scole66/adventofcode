//! # Solution for Advent of Code 2024 Day 13: Claw Contraption
//!
//! Ref: [Advent of Code 2024 Day 13](https://adventofcode.com/2024/day/13)
//!
use anyhow::{anyhow, bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

const A_COST: i64 = 3;
const B_COST: i64 = 1;

struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}
impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut button_a = None;
        let mut button_b = None;
        let mut prize = None;
        for line in s.lines() {
            let (name, value) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("Badly formed line: {line}"))?;
            let (xstr, ystr) = value
                .split_once(", ")
                .ok_or_else(|| anyhow!("Badly formed line: {line}"))?;
            if xstr.len() < 3 || ystr.len() < 3 || &xstr[0..1] != "X" || &ystr[0..1] != "Y" {
                bail!("Badly formed line: {line}");
            }
            let x = xstr[2..].parse::<i64>()?;
            let y = ystr[2..].parse::<i64>()?;
            let numbers = Some((x, y));
            match (name, &xstr[1..2], &ystr[1..2]) {
                ("Button A", "+", "+") => {
                    button_a = numbers;
                }
                ("Button B", "+", "+") => {
                    button_b = numbers;
                }
                ("Prize", "=", "=") => {
                    prize = numbers;
                }
                _ => {
                    bail!("Badly formed line: {line}");
                }
            }
        }
        Ok(Machine {
            button_a: button_a.ok_or_else(|| anyhow!("Parts missing from machine"))?,
            button_b: button_b.ok_or_else(|| anyhow!("Parts missing from machine"))?,
            prize: prize.ok_or_else(|| anyhow!("Parts missing from machine"))?,
        })
    }
}

//
// a, b are integers
// aA + bB = P
//
// bB = P - aA
// b = (P - aA) / B
// b = (P.0 - aA.0) / B.0 = (P.1 - aA.1) / B.1
//     B.1 * (P.0 - aA.0) = B.0 * (P.1 - aA.1)
//     B.1*P.0 - a (A.0*B.1) = B.0*P.1 - a (A.1*B.0)
//     B.1*P.0 - B.0*P.1 = a (A.0*B.1 - A.1*B.0)
//     a = (B.1*P.0 - B.0*P.1) / (A.0*B.1 - A.1*B.0)
//
// a = ((67 * 8400) - (22*5400)) / ((94 * 67) - (34 * 22)) = 80
// b = (8400 - 80*94) / 22 = 40

impl Machine {
    fn buttons(&self) -> Option<(i64, i64)> {
        let (adelta_x, adelta_y) = self.button_a;
        let (bdelta_x, bdelta_y) = self.button_b;
        let (px, py) = self.prize;

        let a_num = bdelta_y * px - bdelta_x * py;
        let a_den = adelta_x * bdelta_y - adelta_y * bdelta_x;

        if a_num % a_den != 0 {
            return None;
        }

        let a = a_num / a_den;

        let b_num = px - a * adelta_x;
        if b_num % bdelta_x != 0 {
            return None;
        }
        let b = b_num / bdelta_x;

        Some((a, b))
    }
}

fn cost(a_presses: i64, b_presses: i64) -> i64 {
    a_presses * A_COST + b_presses * B_COST
}

struct Input {
    machines: Vec<Machine>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            machines: s.split("\n\n").map(Machine::from_str).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn part1(input: &Input) -> i64 {
    input
        .machines
        .iter()
        .filter_map(Machine::buttons)
        .map(|(a, b)| cost(a, b))
        .sum()
}

const ERROR_AMOUNT: i64 = 10_000_000_000_000;

fn part2(input: &Input) -> i64 {
    input
        .machines
        .iter()
        .map(
            |Machine {
                 button_a,
                 button_b,
                 prize,
             }| Machine {
                button_a: *button_a,
                button_b: *button_b,
                prize: (prize.0 + ERROR_AMOUNT, prize.1 + ERROR_AMOUNT),
            },
        )
        .filter_map(|m| m.buttons())
        .map(|(a, b)| cost(a, b))
        .sum()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = part2(&input);
    let elapsed = start_time.elapsed();

    println!("Part1: {part1}");
    println!("Part2: {part2}");
    println!("Time: {elapsed:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test_case(indoc::indoc!("
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
        ") => Some((80, 40)); "first problem sample")]
    fn buttons(machine: &str) -> Option<(i64, i64)> {
        let machine = machine.parse::<Machine>().unwrap();
        machine.buttons()
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 480);
    }
}
