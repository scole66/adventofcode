//! # Solution for Advent of Code 2015 Day 6
//!
//! Ref: [Advent of Code 2015 Day 6](https://adventofcode.com/2015/day/6)
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
