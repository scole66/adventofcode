//! # Solution for Advent of Code 2022 Day 21: Monkey Math
//!
//! Ref: [Advent of Code 2022 Day 21](https://adventofcode.com/2022/day/21)
//!
use ahash::AHashMap;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Subtract),
            "/" => Ok(Op::Divide),
            "*" => Ok(Op::Multiply),
            _ => Err(anyhow::anyhow!("Bad opcode {s}")),
        }
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => '+',
                Op::Subtract => '-',
                Op::Multiply => '*',
                Op::Divide => '/',
            }
        )
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct MonkeyId(u32);
impl FromStr for MonkeyId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut chars_collected = 0;
        for ch in s.chars() {
            let ch_val = u8::try_from(ch)?;
            chars_collected += 1;
            if chars_collected > 4 {
                anyhow::bail!("Too many chars in identifier");
            }
            id = id << 8 | ch_val as u32;
        }
        if chars_collected != 4 {
            anyhow::bail!("Not enough chars in identifier");
        }
        Ok(MonkeyId(id))
    }
}
impl Display for MonkeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = self.0;
        let c1 = ((digits & 0xFF000000) >> 24) as u8 as char;
        let c2 = ((digits & 0x00FF0000) >> 16) as u8 as char;
        let c3 = ((digits & 0x0000FF00) >> 8) as u8 as char;
        let c4 = (digits & 0x000000FF) as u8 as char;
        write!(f, "{c1}{c2}{c3}{c4}")
    }
}
impl Debug for MonkeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.0;
        write!(f, "MonkeyId({id:x} [{}]", self)
    }
}

#[derive(Debug, Copy, Clone)]
enum InsnOp {
    Yell(i64),
    Op(Op, MonkeyId, MonkeyId),
    Bogus, // The default value, should never be executed.
}
impl FromStr for InsnOp {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let first_word = words.next().ok_or_else(|| anyhow::anyhow!("bad op parse"))?;
        let second_word = words.next();
        match second_word {
            None => {
                let value = first_word.parse::<i64>()?;
                Ok(InsnOp::Yell(value))
            }
            Some(operation) => {
                let third_word = words.next().ok_or_else(|| anyhow::anyhow!("bad op parse"))?;
                let left_id = first_word.parse::<MonkeyId>()?;
                let op = operation.parse::<Op>()?;
                let right_id = third_word.parse::<MonkeyId>()?;
                Ok(InsnOp::Op(op, left_id, right_id))
            }
        }
    }
}
impl Default for InsnOp {
    fn default() -> Self {
        InsnOp::Bogus
    }
}
impl Display for InsnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsnOp::Yell(v) => write!(f, "{v}"),
            InsnOp::Op(op, left, right) => write!(f, "{left} {op} {right}"),
            InsnOp::Bogus => write!(f, "bogus"),
        }
    }
}
struct Insn {
    id: MonkeyId,
    op: InsnOp,
}
impl FromStr for Insn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (before, after) = s.split_once(':').ok_or_else(|| anyhow::anyhow!("bad op parse"))?;
        let id = before.parse::<MonkeyId>()?;
        let first_after_byte = after.as_bytes()[0];
        if first_after_byte != b' ' {
            anyhow::bail!("bad op parse");
        }
        let op = after[1..].parse::<InsnOp>()?;
        Ok(Insn { id, op })
    }
}
impl Display for Insn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.op)
    }
}

#[derive(Debug, Default)]
struct Monkey {
    value: Option<i64>,
    instruction: InsnOp,
    watchers: Vec<MonkeyId>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Style {
    Monkey,
    Human,
}

#[derive(Default)]
struct Machine {
    monkeys: AHashMap<MonkeyId, Monkey>,
    notify_queue: VecDeque<MonkeyId>,
}
impl Machine {
    fn new() -> Self {
        Self::default()
    }

    fn run(&mut self, instructions: &[Insn], style: Style) -> Option<i64> {
        let root_id = "root".parse::<MonkeyId>().unwrap();
        for insn in instructions {
            self.execute(insn, style);
        }
        self.value(root_id)
    }

    fn value(&self, id: MonkeyId) -> Option<i64> {
        self.monkeys.get(&id).and_then(|x| x.value)
    }

    fn handle_notifications(&mut self) {
        while let Some(listener) = self.notify_queue.pop_front() {
            let insn = self.monkeys.get(&listener).unwrap();
            if let InsnOp::Op(op, left, right) = insn.instruction {
                let left = self.value(left);
                let right = self.value(right);
                if let (Some(vl), Some(vr)) = (left, right) {
                    // we have enough info to give our monkey a value
                    let val = match op {
                        Op::Add => vl + vr,
                        Op::Subtract => vl - vr,
                        Op::Multiply => vl * vr,
                        Op::Divide => vl / vr,
                    };
                    self.monkeys.entry(listener).and_modify(|monkey| {
                        println!("Monkey {listener} yells {val}!");
                        monkey.value = Some(val);
                        for watcher in monkey.watchers.iter() {
                            self.notify_queue.push_back(*watcher);
                        }
                    });
                }
            }
        }
    }

    fn execute(&mut self, insn: &Insn, style: Style) {
        println!("Executing \"{insn}\"");
        let id = insn.id;
        match insn.op {
            InsnOp::Yell(val) => {
                if !(style == Style::Human && id == "humn".parse::<MonkeyId>().unwrap()) {
                    let monkey = self.monkeys.entry(id).or_default();
                    println!("Monkey {id} yells {val}!");

                    monkey.value = Some(val);
                    for watcher in monkey.watchers.iter() {
                        self.notify_queue.push_back(*watcher);
                    }
                }
            }
            InsnOp::Op(_, left, right) => {
                let monkey = self.monkeys.entry(id).or_default();
                monkey.instruction = insn.op;
                let left_monkey = self.monkeys.entry(left).or_default();
                left_monkey.watchers.push(id);
                let right_monkey = self.monkeys.entry(right).or_default();
                right_monkey.watchers.push(id);
                // Put the new monkey in the notifier list; our dependencies might already be resolved.
                self.notify_queue.push_back(id);
            }
            InsnOp::Bogus => unreachable!(),
        }
        self.handle_notifications();
    }

    fn humanity(&self) -> i64 {
        let root = "root".parse::<MonkeyId>().unwrap();
        let humn = "humn".parse::<MonkeyId>().unwrap();
        let root_instruction = self.monkeys.get(&root).unwrap().instruction;
        assert!(matches!(root_instruction, InsnOp::Op(..)));
        if let InsnOp::Op(_, left, right) = root_instruction {
            let (mut known_value, mut sought_id) = match (self.value(left), self.value(right)) {
                (Some(val), None) => (val, right),
                (None, Some(val)) => (val, left),
                _ => unreachable!(),
            };
            while sought_id != humn {
                let instruction = self.monkeys.get(&sought_id).unwrap().instruction;
                assert!(matches!(instruction, InsnOp::Op(..)));
                if let InsnOp::Op(op, left, right) = instruction {
                    (known_value, sought_id) = match (self.value(left), self.value(right)) {
                        (Some(val), None) => match op {
                            Op::Add => (known_value - val, right),
                            Op::Subtract => (val - known_value, right),
                            Op::Multiply => {
                                assert_eq!(known_value % val, 0);
                                (known_value / val, right)
                            }
                            Op::Divide => {
                                assert_eq!(val % known_value, 0);
                                (val / known_value, right)
                            }
                        },
                        (None, Some(val)) => match op {
                            Op::Add => (known_value - val, left),
                            Op::Subtract => (known_value + val, left),
                            Op::Multiply => {
                                assert_eq!(known_value % val, 0);
                                (known_value / val, left)
                            }
                            Op::Divide => (known_value * val, left),
                        },
                        _ => unreachable!(),
                    };
                }
            }
            return known_value;
        }
        unreachable!()
    }
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Insn>())
        .collect::<anyhow::Result<Vec<Insn>>>()?;
    let mut machine = Machine::new();
    match machine.run(&instructions, Style::Monkey) {
        Some(val) => Ok(val),
        None => {
            anyhow::bail!("No value for root monkey")
        }
    }
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Insn>())
        .collect::<anyhow::Result<Vec<Insn>>>()?;
    let mut machine = Machine::new();
    let rcode = machine.run(&instructions, Style::Human);
    assert!(rcode.is_none());
    Ok(machine.humanity())
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 152);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 301);
    }

    #[test_case("root" => 0x726f6f74)]
    fn monkey_id(text: &str) -> u32 {
        let mid = text.parse::<MonkeyId>().unwrap();
        mid.0
    }

    #[test_case("root".parse::<MonkeyId>().unwrap() => String::from("root"))]
    fn monkey_display(id: MonkeyId) -> String {
        format!("{id}")
    }
}
