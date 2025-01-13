//! # Solution for Advent of Code 2015 Day 6
//!
//! Ref: [Advent of Code 2015 Day 6](https://adventofcode.com/2015/day/6)
//!
//! ## --- Day 6: Probably a Fire Hazard ---
//!
//! Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided
//! to deploy one million lights in a 1000x1000 grid.
//!
//! Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the
//! ideal lighting configuration.
//!
//! Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at `0,0`, `0,999`,
//! `999,999`, and `999,0`. The instructions include whether to `turn on`, `turn off`, or `toggle` various inclusive
//! ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a
//! coordinate pair like `0,0 through 2,2` therefore refers to 9 lights in a 3x3 square. The lights all start turned
//! off.
//!
//! To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent
//! you in order.
//!
//! For example:
//!
//! * `turn on 0,0 through 999,999` would turn on (or leave on) every light.
//! * `toggle 0,0 through 999,0` would toggle the first line of 1000 lights, turning off the ones that were on, and
//!   turning on the ones that were off.
//! * `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.
//!
//! After following the instructions, **how many lights are lit?**
//!
//! ## --- Part Two ---
//!
//! You just finish implementing your winning light pattern when you realize you mistranslated Santa's message from
//! Ancient Nordic Elvish.
//!
//! The light grid you bought actually has individual brightness controls; each light can have a brightness of zero or
//! more. The lights all start at zero.
//!
//! The phrase `turn on` actually means that you should increase the brightness of those lights by `1`.
//!
//! The phrase `turn off` actually means that you should decrease the brightness of those lights by `1`, to a minimum
//! of zero.
//!
//! The phrase `toggle` actually means that you should increase the brightness of those lights by `2`.
//!
//! What is the **total brightness** of all lights combined after following Santa's instructions?
//!
//! For example:
//!
//! * `turn on 0,0 through 0,0` would increase the total brightness by `1`.
//! * `toggle 0,0 through 999,999` would increase the total brightness by `2000000`.

use anyhow::{self};
use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::ops::{Index, IndexMut};

struct Lights(Box<[u32]>);

impl Lights {
    pub fn new() -> Self {
        Lights(vec![0_u32; 1_000_000].into_boxed_slice())
    }

    fn to_index(pos: (u32, u32)) -> usize {
        let (row, col) = pos;
        row as usize * 1_000 + col as usize
    }

    fn alter(&mut self, action: LightAction, rect: (u32, u32, u32, u32)) {
        let mut fn_act: Box<dyn FnMut((u32, u32))> = match action {
            LightAction::Lighten => Box::new(|pos| self[pos] = 1),
            LightAction::Darken => Box::new(|pos| self[pos] = 0),
            LightAction::Toggle => Box::new(|pos| self[pos] = 1 - self[pos]),
        };
        for pos in VisitRect::new(rect) {
            fn_act(pos);
        }
    }

    fn adjust(&mut self, action: LightAction, rect: (u32, u32, u32, u32)) {
        let mut fn_act: Box<dyn FnMut((u32, u32))> = match action {
            LightAction::Lighten => Box::new(|pos| self[pos] += 1),
            LightAction::Darken => Box::new(|pos| self[pos] = self[pos].saturating_sub(1)),
            LightAction::Toggle => Box::new(|pos| self[pos] += 2),
        };
        for pos in VisitRect::new(rect) {
            fn_act(pos);
        }
    }

    fn brightness(&self) -> usize {
        VisitRect::new((0, 0, 999, 999)).map(|pos| self[pos] as usize).sum()
    }
}

impl Index<(u32, u32)> for Lights {
    type Output = u32;
    fn index(&self, pos: (u32, u32)) -> &Self::Output {
        let index = Lights::to_index(pos);
        &self.0[index]
    }
}

impl IndexMut<(u32, u32)> for Lights {
    fn index_mut(&mut self, pos: (u32, u32)) -> &mut Self::Output {
        let index = Lights::to_index(pos);
        &mut self.0[index]
    }
}

#[derive(Debug)]
struct VisitRect {
    column: u32,
    row: u32,
    bottom_edge: u32,
    left_edge: u32,
    right_edge: u32,
}

impl VisitRect {
    fn new(rect: (u32, u32, u32, u32)) -> Self {
        let (x1, y1, x2, y2) = rect;
        let (left, right) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let (top, bottom) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        VisitRect {
            column: left,
            row: top,
            bottom_edge: bottom,
            left_edge: left,
            right_edge: right,
        }
    }
}

impl Iterator for VisitRect {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.bottom_edge {
            None
        } else {
            let result = (self.column, self.row);

            self.column += 1;
            if self.column > self.right_edge {
                self.column = self.left_edge;
                self.row += 1;
            }
            Some(result)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LightAction {
    Lighten,
    Darken,
    Toggle,
}

impl TryFrom<&str> for LightAction {
    type Error = anyhow::Error;
    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "turn on" => Ok(LightAction::Lighten),
            "turn off" => Ok(LightAction::Darken),
            "toggle" => Ok(LightAction::Toggle),
            _ => Err(anyhow::anyhow!("This is not a valid action: {}", item)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: LightAction,
    rect: (u32, u32, u32, u32),
}

fn process(lines: &[String]) -> Vec<Instruction> {
    lazy_static! {
        static ref INSTRUCTION_RE: Regex = Regex::new(
            r"^(?P<action>turn on|turn off|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)\s*$"
        )
        .unwrap();
    }
    lines
        .iter()
        .filter_map(|line| INSTRUCTION_RE.captures(line.as_str()))
        .map(|caps| Instruction {
            action: LightAction::try_from(caps.name("action").unwrap().as_str()).unwrap(),
            rect: (
                caps.name("x1").unwrap().as_str().parse().unwrap(),
                caps.name("y1").unwrap().as_str().parse().unwrap(),
                caps.name("x2").unwrap().as_str().parse().unwrap(),
                caps.name("y2").unwrap().as_str().parse().unwrap(),
            ),
        })
        .collect()
}

fn main() -> io::Result<()> {
    let mut lines = Vec::<String>::new();

    loop {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        lines.push(buffer.trim().to_string());
    }

    let instruction_sequence = process(&lines);
    {
        let mut grid = Lights::new();
        for instruction in &instruction_sequence {
            let Instruction { action, rect } = instruction;
            grid.alter(*action, *rect);
        }

        println!("Part 1: {} lights lit.", grid.brightness());
    }
    {
        let mut grid = Lights::new();
        for instruction in instruction_sequence {
            let Instruction { action, rect } = instruction;
            grid.adjust(action, rect);
        }

        println!("Part 2: {} brightness.", grid.brightness());
    }

    Ok(())
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use test_case::test_case;
//}
