//! # Solution for Advent of Code 2024 Day 17: Chronospatial Computer
//!
//! Ref: [Advent of Code 2024 Day 17](https://adventofcode.com/2024/day/17)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::io::{self, Read};
use std::str::FromStr;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    ADivide,     // adv
    BXorLiteral, // bxl
    BStore,      // bst
    JumpNotZero, // jnz
    BXorC,       // bxc
    Output,      // out
    BDivide,     // bdv
    CDivide,     // cdv
}

#[derive(Debug, Clone)]
struct Input {
    starting_a: i64,
    starting_b: i64,
    starting_c: i64,
    program: Vec<u8>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (registers, program) = s.split_once("\n\n").ok_or_else(|| anyhow!("bad input"))?;
        let mut starting_a = None;
        let mut starting_b = None;
        let mut starting_c = None;
        for line in registers.lines() {
            let (name, value) = line.split_once(": ").ok_or_else(|| anyhow!("bad input"))?;
            let value = value.parse::<i64>()?;
            match name {
                "Register A" => {
                    starting_a = Some(value);
                }
                "Register B" => {
                    starting_b = Some(value);
                }
                "Register C" => {
                    starting_c = Some(value);
                }
                _ => {
                    bail!("Bad input");
                }
            }
        }
        let starting_a = starting_a.ok_or_else(|| anyhow!("bad register name"))?;
        let starting_b = starting_b.ok_or_else(|| anyhow!("bad register name"))?;
        let starting_c = starting_c.ok_or_else(|| anyhow!("bad register name"))?;

        let (_, program) = program.split_once(": ").ok_or_else(|| anyhow!("bad input"))?;
        let program = program
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Input {
            starting_a,
            starting_b,
            starting_c,
            program,
        })
    }
}

struct Machine {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
    program: Box<[u8]>,
}

impl From<Input> for Machine {
    fn from(value: Input) -> Self {
        Self {
            register_a: value.starting_a,
            register_b: value.starting_b,
            register_c: value.starting_c,
            instruction_pointer: 0,
            program: value.program.into_boxed_slice(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RunResult {
    Output(i64),
    Halt,
    Nothing,
}

#[derive(Debug, thiserror::Error)]
enum ExecutionError {
    #[error("Bad combo operand")]
    BadComboOperand,
    #[error("bad opcode")]
    BadOpcode,
}

impl Machine {
    fn combo_operand(&self, operand: u8) -> Result<i64, ExecutionError> {
        match operand {
            0..=3 => Ok(i64::from(operand)),
            4 => Ok(self.register_a),
            5 => Ok(self.register_b),
            6 => Ok(self.register_c),
            _ => Err(ExecutionError::BadComboOperand),
        }
    }

    fn run_instruction(&mut self) -> Result<RunResult, ExecutionError> {
        if self.instruction_pointer >= self.program.len() - 1 {
            return Ok(RunResult::Halt);
        }
        let insn = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];
        let mut output = None;
        match insn {
            0 => {
                // adv
                let numerator = self.register_a;
                let denominator = 1_i64 << self.combo_operand(operand)?;
                let result = numerator.div_euclid(denominator);
                self.register_a = result;
                self.instruction_pointer += 2;
            }
            1 => {
                // bxl
                let result = self.register_b ^ i64::from(operand);
                self.register_b = result;
                self.instruction_pointer += 2;
            }
            2 => {
                // bst
                let result = self.combo_operand(operand)?.rem_euclid(8);
                self.register_b = result;
                self.instruction_pointer += 2;
            }
            3 => {
                // jnz
                let should_jump = self.register_a != 0;
                if should_jump {
                    self.instruction_pointer = usize::from(operand);
                } else {
                    self.instruction_pointer += 2;
                }
            }
            4 => {
                // bxc
                self.register_b ^= self.register_c;
                self.instruction_pointer += 2;
            }
            5 => {
                // out
                let result = self.combo_operand(operand)?.rem_euclid(8);
                output = Some(result);
                self.instruction_pointer += 2;
            }
            6 => {
                // bdv
                let numerator = self.register_a;
                let denominator = 1_i64 << self.combo_operand(operand)?;
                let result = numerator.div_euclid(denominator);
                self.register_b = result;
                self.instruction_pointer += 2;
            }
            7 => {
                // cdv
                let numerator = self.register_a;
                let denominator = 1_i64 << self.combo_operand(operand)?;
                let result = numerator.div_euclid(denominator);
                self.register_c = result;
                self.instruction_pointer += 2;
            }
            _ => return Err(ExecutionError::BadOpcode),
        }

        Ok(output.map_or(RunResult::Nothing, RunResult::Output))
    }

    fn run_program(&mut self) -> Result<String, ExecutionError> {
        let mut output = Vec::new();
        self.instruction_pointer = 0;
        loop {
            match self.run_instruction()? {
                RunResult::Output(out) => {
                    output.push(out);
                }
                RunResult::Halt => {
                    break;
                }
                RunResult::Nothing => {}
            }
        }

        Ok(output.iter().map(i64::to_string).join(","))
    }
}

fn part1(input: &Input) -> Result<String> {
    let mut machine = Machine::from(input.clone());
    machine.run_program().map_err(Error::from)
}

fn part2(input: &Input) -> Result<i64> {
    let mut initial_reg_a = 0;
    loop {
        let attempts = [0, 1, 2, 3, 4, 5, 6, 7]
            .par_iter()
            .filter_map(|delta| {
                let mut machine = Machine::from(input.clone());
                machine.register_a = initial_reg_a + delta;
                let output = machine.run_program().unwrap();
                if output == machine.program.iter().map(u8::to_string).join(",") {
                    return Some(initial_reg_a + delta);
                }
                None
            }).collect::<Vec<_>>();
        if let Some(result) = attempts.first() {
            return Ok(*result);
        }
        initial_reg_a += 8;
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input)?;
    let part2 = part2(&input)?;
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
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    static SAMPLE2: &str = indoc::indoc! {"
        Register A: 0
        Register B: 0
        Register C: 9

        Program: 2,6
    "};

    static SAMPLE3: &str = indoc::indoc! {"
        Register A: 10
        Register B: 0
        Register C: 0

        Program: 5,0,5,1,5,4
    "};

    static SAMPLE4: &str = indoc::indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    static SAMPLE5: &str = indoc::indoc! {"
        Register A: 0
        Register B: 29
        Register C: 0

        Program: 1,7
    "};

    static SAMPLE6: &str = indoc::indoc! {"
        Register A: 0
        Register B: 2024
        Register C: 43690

        Program: 4,0
    "};

    static PART2_SAMPLE: &str = indoc::indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test_case(SAMPLE2 => Ok(("".to_string(), 0, 1, 9)); "first insn example")]
    #[test_case(SAMPLE3 => Ok(("0,1,2".to_string(), 10, 0, 0)); "second insn example")]
    #[test_case(SAMPLE4 => Ok(("4,2,5,6,7,7,7,7,3,1,0".to_string(), 0, 0, 0)); "another example")]
    #[test_case(SAMPLE5 => Ok(("".to_string(), 0, 26, 0)); "bxl example")]
    #[test_case(SAMPLE6 => Ok(("".to_string(), 0, 44354, 43690)); "bxc example")]
    fn run(code: &str) -> Result<(String, i64, i64, i64), String> {
        let input = code.parse::<Input>().map_err(|e| e.to_string())?;
        let mut machine = Machine::from(input);
        let output = machine.run_program().map_err(|e| e.to_string())?;
        Ok((output, machine.register_a, machine.register_b, machine.register_c))
    }

    #[test_case(SAMPLE => Ok("4,6,3,5,6,3,5,2,1,0".to_string()); "original sample")]
    fn part1_sample(input: &str) -> Result<String, String> {
        part1(&input.parse::<Input>().unwrap()).map_err(|err| err.to_string())
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&PART2_SAMPLE.parse::<Input>().unwrap()).unwrap(), 117440);
    }
}
