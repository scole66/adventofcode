//! # Solution for Advent of Code 2022 Day 10: Cathode-Ray Tube
//!
//! Ref: [Advent of Code 2022 Day 10](https://adventofcode.com/2022/day/10)
//!

use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(noop|addx (?P<operand>-?(?:0|[1-9][0-9]*)))$").expect("Hand-rolled regex is valid")
        });
        let caps = PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Unexpected instruction: \"{s}\""))?;
        if &caps[0] == "noop" {
            Ok(Instruction::Noop)
        } else {
            Ok(Instruction::Addx(caps["operand"].parse::<isize>()?))
        }
    }
}

fn simulate(insns: &[Instruction]) -> Vec<isize> {
    let mut output_signal = vec![];
    let mut accumulator = 1;
    output_signal.push(accumulator); // no instructions have run at clock 0.

    for insn in insns {
        match insn {
            Instruction::Noop => {
                output_signal.push(accumulator);
            }
            Instruction::Addx(val) => {
                output_signal.push(accumulator);
                output_signal.push(accumulator);
                accumulator += val;
            }
        }
    }

    output_signal
}

fn part1(input: &str) -> anyhow::Result<isize> {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<anyhow::Result<Vec<Instruction>>>()?;

    // ... I think the right thing to do here is just run the dang simulation.
    let signal = simulate(&instructions);
    Ok(signal[20] * 20
        + signal[60] * 60
        + signal[100] * 100
        + signal[140] * 140
        + signal[180] * 180
        + signal[220] * 220)
}

fn part2(input: &str) -> anyhow::Result<String> {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<anyhow::Result<Vec<Instruction>>>()?;

    let signal = simulate(&instructions);

    let mut result = String::new();
    let mut render_column = 0;
    for sprite_middle in signal[1..].iter() {
        result.push(
            if sprite_middle - 1 <= render_column && render_column <= sprite_middle + 1 {
                '#'
            } else {
                ' '
            },
        );
        render_column += 1;
        if render_column > 39 {
            result.push('\n');
            render_column = 0;
        }
    }
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2:\n{}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 13140);
    }

    #[test]
    fn part2_sample() {
        static RENDERING: &str = indoc::indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "};

        let result = part2(SAMPLE).unwrap().replace(' ', ".");
        assert_eq!(result, RENDERING);
    }

    mod instruction_parse {
        use super::*;
        #[test]
        fn insn_noop() {
            assert!(matches!(&"noop".parse::<Instruction>().unwrap(), Instruction::Noop));
        }
        #[test]
        fn insn_addx_pos() {
            assert!(matches!(
                &"addx 10".parse::<Instruction>().unwrap(),
                Instruction::Addx(10)
            ));
        }
        #[test]
        fn insn_addx_onedigit() {
            assert!(matches!(
                &"addx 6".parse::<Instruction>().unwrap(),
                Instruction::Addx(6)
            ));
        }
        #[test]
        fn insn_addx_neg() {
            assert!(matches!(
                &"addx -28".parse::<Instruction>().unwrap(),
                Instruction::Addx(-28)
            ));
        }
    }
}
