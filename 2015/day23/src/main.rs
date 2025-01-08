//! # Solution for Advent of Code 2015 Day 23: Opening the Turing Lock
//!
//! Ref: [Advent of Code 2015 Day 23](https://adventofcode.com/2015/day/23)
//!
use anyhow::{anyhow, bail, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum RegisterId {
    A,
    B,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    Halve(RegisterId),
    Triple(RegisterId),
    Increment(RegisterId),
    Jump(i64),
    JumpEven(RegisterId, i64),
    JumpOne(RegisterId, i64),
}

#[derive(Debug)]
struct Machine {
    program: Box<[Instruction]>,
    pc: usize,
    register_a: usize,
    register_b: usize,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program: program.into_boxed_slice(),
            pc: 0,
            register_a: 0,
            register_b: 0,
        }
    }

    fn run_one(&mut self) -> Option<()> {
        let insn = *self.program.get(self.pc)?;
        self.pc += 1;
        match insn {
            Instruction::Halve(register_id) => match register_id {
                RegisterId::A => {
                    self.register_a >>= 1;
                }
                RegisterId::B => {
                    self.register_b >>= 1;
                }
            },
            Instruction::Triple(register_id) => match register_id {
                RegisterId::A => {
                    self.register_a *= 3;
                }
                RegisterId::B => {
                    self.register_b *= 3;
                }
            },
            Instruction::Increment(register_id) => match register_id {
                RegisterId::A => {
                    self.register_a += 1;
                }
                RegisterId::B => {
                    self.register_b += 1;
                }
            },
            Instruction::Jump(offset) => {
                if offset >= 0 {
                    self.pc += usize::try_from(offset).unwrap() - 1;
                } else {
                    self.pc -= usize::try_from(-offset).unwrap() + 1;
                }
            }
            Instruction::JumpEven(register_id, offset) => {
                let is_even = match register_id {
                    RegisterId::A => self.register_a,
                    RegisterId::B => self.register_b,
                } & 1
                    == 0;
                if is_even {
                    if offset >= 0 {
                        self.pc += usize::try_from(offset).unwrap() - 1;
                    } else {
                        self.pc -= usize::try_from(-offset).unwrap() + 1;
                    }
                }
            }
            Instruction::JumpOne(register_id, offset) => {
                let is_one = match register_id {
                    RegisterId::A => self.register_a,
                    RegisterId::B => self.register_b,
                } == 1;
                if is_one {
                    if offset >= 0 {
                        self.pc += usize::try_from(offset).unwrap() - 1;
                    } else {
                        self.pc -= usize::try_from(-offset).unwrap() + 1;
                    }
                }
            }
        }
        Some(())
    }

    fn run(&mut self) {
        while self.run_one().is_some() {}
    }
}

struct Input {
    program: Vec<Instruction>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Input {
            program: s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (insn, operand) = s.split_once(' ').ok_or_else(|| anyhow!("invalid operand"))?;
        match insn {
            "hlf" => {
                let register = operand.parse::<RegisterId>()?;
                Ok(Self::Halve(register))
            }
            "tpl" => {
                let register = operand.parse::<RegisterId>()?;
                Ok(Self::Triple(register))
            }
            "inc" => {
                let register = operand.parse::<RegisterId>()?;
                Ok(Self::Increment(register))
            }
            "jmp" => {
                let offset = operand.parse::<i64>()?;
                Ok(Self::Jump(offset))
            }
            "jie" => {
                let (register_str, offset_str) = operand.split_once(", ").ok_or_else(|| anyhow!("bad instruction"))?;
                let register = register_str.parse::<RegisterId>()?;
                let offset = offset_str.parse::<i64>()?;
                Ok(Self::JumpEven(register, offset))
            }
            "jio" => {
                let (register_str, offset_str) = operand.split_once(", ").ok_or_else(|| anyhow!("bad instruction"))?;
                let register = register_str.parse::<RegisterId>()?;
                let offset = offset_str.parse::<i64>()?;
                Ok(Self::JumpOne(register, offset))
            }
            _ => bail!("bad instruction"),
        }
    }
}

impl FromStr for RegisterId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => bail!("bad register"),
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut machine = Machine::new(input.program.clone());
    machine.run();
    machine.register_b
}

fn part2(input: &Input) -> usize {
    let mut machine = Machine::new(input.program.clone());
    machine.register_a = 1;
    machine.run();
    machine.register_b
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
