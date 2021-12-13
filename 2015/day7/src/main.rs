//! # Solution for Advent of Code 2015 Day 7
//!
//! Ref: [Advent of Code 2015 Day 7](https://adventofcode.com/2015/day/7)
//!

use ahash::AHashMap;
use ahash::AHashSet;
use anyhow::{self, Context};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Value {
    Identifier(String),
    Number(u64),
}
#[derive(Debug, PartialEq)]
enum Gate {
    And(Value, Value, String),
    Or(Value, Value, String),
    Not(Value, String),
    Lshift(Value, u64, String),
    Rshift(Value, u64, String),
    Identity(Value, String),
}

impl TryFrom<&str> for Gate {
    type Error = anyhow::Error;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref GATE_PATTERN: Regex = {
                let id_pattern = |s| format!("(?P<{}_id>[a-z]+)", s);
                let number_pattern = |s| format!("(?P<{}_num>0|[1-9][0-9]*)", s);
                let input_pattern = |s| format!("(?:{}|{})", id_pattern(s), number_pattern(s));
                let input_part: String = format!("(?:(?P<lone_identifier>{})|(?P<two_arg_insn>{} (?P<insn>AND|OR|LSHIFT|RSHIFT) {})|(?:NOT (?P<complement>{})))", input_pattern("lone"), input_pattern("left"), input_pattern("right"), input_pattern("not"));
                let gate_pattern: String = format!("(?:{} -> (?P<output>{}))", input_part, id_pattern("output"));
                Regex::new(&gate_pattern).unwrap()
            };
        }

        let captures = GATE_PATTERN
            .captures(src)
            .ok_or_else(|| anyhow::anyhow!("Cannot parse ‘{}’ as a valid gate description", src))?;

        fn parse_value(captures: &Captures, id: &str) -> anyhow::Result<Value> {
            if let Some(id_val) = captures.name(&format!("{}_id", id)) {
                Ok(Value::Identifier(id_val.as_str().to_string()))
            } else {
                let num = captures
                    .name(&format!("{}_num", id))
                    .unwrap()
                    .as_str()
                    .parse::<u64>()
                    .context("This integer is too large for a gate description")?;
                Ok(Value::Number(num))
            }
        }

        if captures.name("complement").is_some() {
            let val = parse_value(&captures, "not")?;
            return Ok(Gate::Not(val, captures.name("output").unwrap().as_str().to_string()));
        }

        if captures.name("lone_identifier").is_some() {
            let val = parse_value(&captures, "lone")?;
            return Ok(Gate::Identity(
                val,
                captures.name("output").unwrap().as_str().to_string(),
            ));
        }

        assert!(captures.name("two_arg_insn").is_some());

        let left_val = parse_value(&captures, "left")?;
        let right_val = parse_value(&captures, "right")?;
        let output = captures.name("output").unwrap().as_str().to_string();

        match captures.name("insn").unwrap().as_str() {
            "AND" => Ok(Gate::And(left_val, right_val, output)),
            "OR" => Ok(Gate::Or(left_val, right_val, output)),
            "LSHIFT" => match right_val {
                Value::Identifier(id_val) => Err(anyhow::anyhow!(
                    "Right argument to LSHIFT must be an integer (was {})",
                    id_val
                )),
                Value::Number(num) => Ok(Gate::Lshift(left_val, num, output)),
            },
            "RSHIFT" => match right_val {
                Value::Identifier(id_val) => Err(anyhow::anyhow!(
                    "Right argument to RSHIFT must be an integer (was {})",
                    id_val
                )),
                Value::Number(num) => Ok(Gate::Rshift(left_val, num, output)),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Circuit {
    signals: AHashMap<String, Option<u64>>,
    gates: Vec<Gate>,
    overrides: AHashSet<String>,
}

struct StringWrap(String);
impl From<&str> for StringWrap {
    fn from(src: &str) -> Self {
        Self(src.to_string())
    }
}
impl From<String> for StringWrap {
    fn from(src: String) -> Self {
        Self(src)
    }
}

impl FromIterator<StringWrap> for anyhow::Result<Circuit> {
    fn from_iter<I: IntoIterator<Item = StringWrap>>(iter: I) -> Self {
        let gates = Vec::new();
        let signals = AHashMap::new();
        let mut circuit = Circuit { signals, gates, overrides: AHashSet::new() };

        for s in iter.into_iter() {
            let gate = Gate::try_from(s.0.as_str())?;
            circuit.add(gate);
        }

        Ok(circuit)
    }
}

impl Circuit {
    fn add(&mut self, gate: Gate) {
        match &gate {
            Gate::Identity(Value::Identifier(id), name)
            | Gate::Not(Value::Identifier(id), name)
            | Gate::And(Value::Number(_), Value::Identifier(id), name)
            | Gate::And(Value::Identifier(id), Value::Number(_), name)
            | Gate::Or(Value::Number(_), Value::Identifier(id), name)
            | Gate::Or(Value::Identifier(id), Value::Number(_), name)
            | Gate::Lshift(Value::Identifier(id), _, name)
            | Gate::Rshift(Value::Identifier(id), _, name) => {
                self.signals.insert(id.clone(), None);
                self.signals.insert(name.clone(), None);
            }
            Gate::Identity(Value::Number(_), name)
            | Gate::Not(Value::Number(_), name)
            | Gate::And(Value::Number(_), Value::Number(_), name)
            | Gate::Or(Value::Number(_), Value::Number(_), name)
            | Gate::Lshift(Value::Number(_), _, name)
            | Gate::Rshift(Value::Number(_), _, name) => {
                self.signals.insert(name.clone(), None);
            }
            Gate::And(Value::Identifier(left), Value::Identifier(right), name)
            | Gate::Or(Value::Identifier(left), Value::Identifier(right), name) => {
                self.signals.insert(left.clone(), None);
                self.signals.insert(right.clone(), None);
                self.signals.insert(name.clone(), None);
            }
        }
        self.gates.push(gate);
    }

    fn run(&mut self, initial_values: AHashMap<String, u64>) {
        for val_ref in self.signals.values_mut() {
            *val_ref = None;
        }
        self.overrides = AHashSet::new();
        for (key, val) in initial_values {
            self.overrides.insert(key.clone());
            self.signals.insert(key, Some(val));
        }

        loop {
            let mut changes_seen = false;
            for g in self.gates.iter() {
                match g {
                    Gate::Identity(Value::Identifier(id), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(value) = self.signals[id] {
                            let dest = self.signals.get_mut(name).unwrap();
                            if dest.is_none() {
                                *dest = Some(value);
                                changes_seen = true;
                            } else {
                                assert_eq!(*dest, Some(value));
                            }
                        }
                    }
                    Gate::Identity(Value::Number(num), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        let dest = self.signals.get_mut(name).unwrap();
                        if dest.is_none() {
                            *dest = Some(*num);
                            changes_seen = true;
                        } else {
                            assert_eq!(*dest, Some(*num));
                        }
                    }
                    Gate::Not(Value::Identifier(id), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(value) = self.signals[id] {
                            let dest = self.signals.get_mut(name).unwrap();
                            let result = value ^ 0xFFFF;
                            if dest.is_none() {
                                *dest = Some(result);
                                changes_seen = true;
                            } else {
                                assert_eq!(*dest, Some(result));
                            }
                        }
                    }
                    Gate::Not(Value::Number(num), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        let dest = self.signals.get_mut(name).unwrap();
                        let result = *num ^ 0xFFFF;
                        if dest.is_none() {
                            *dest = Some(result);
                            changes_seen = true;
                        } else {
                            assert_eq!(*dest, Some(result));
                        }
                    }
                    Gate::And(Value::Identifier(left), Value::Identifier(right), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(left_value) = self.signals[left] {
                            if let Some(right_value) = self.signals[right] {
                                let dest = self.signals.get_mut(name).unwrap();
                                let result = left_value & right_value;
                                if dest.is_none() {
                                    *dest = Some(result);
                                    changes_seen = true;
                                } else {
                                    assert_eq!(*dest, Some(result));
                                }
                            }
                        }
                    }
                    Gate::And(Value::Number(num), Value::Identifier(id), name)
                    | Gate::And(Value::Identifier(id), Value::Number(num), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(value) = self.signals[id] {
                            let dest = self.signals.get_mut(name).unwrap();
                            let result = num & value;
                            if dest.is_none() {
                                *dest = Some(result);
                                changes_seen = true;
                            } else {
                                assert_eq!(*dest, Some(result));
                            }
                        }
                    }
                    Gate::And(Value::Number(left), Value::Number(right), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        let dest = self.signals.get_mut(name).unwrap();
                        let result = left & right;
                        if dest.is_none() {
                            *dest = Some(result);
                            changes_seen = true;
                        } else {
                            assert_eq!(*dest, Some(result));
                        }
                    }
                    Gate::Or(Value::Identifier(left), Value::Identifier(right), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(left_value) = self.signals[left] {
                            if let Some(right_value) = self.signals[right] {
                                let dest = self.signals.get_mut(name).unwrap();
                                let result = left_value | right_value;
                                if dest.is_none() {
                                    *dest = Some(result);
                                    changes_seen = true;
                                } else {
                                    assert_eq!(*dest, Some(result));
                                }
                            }
                        }
                    }
                    Gate::Or(Value::Number(num), Value::Identifier(id), name)
                    | Gate::Or(Value::Identifier(id), Value::Number(num), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(value) = self.signals[id] {
                            let dest = self.signals.get_mut(name).unwrap();
                            let result = num | value;
                            if dest.is_none() {
                                *dest = Some(result);
                                changes_seen = true;
                            } else {
                                assert_eq!(*dest, Some(result));
                            }
                        }
                    }
                    Gate::Or(Value::Number(left), Value::Number(right), name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        let dest = self.signals.get_mut(name).unwrap();
                        let result = left | right;
                        if dest.is_none() {
                            *dest = Some(result);
                            changes_seen = true;
                        } else {
                            assert_eq!(*dest, Some(result));
                        }
                    }
                    Gate::Lshift(Value::Identifier(id), shift_amt, name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(value) = self.signals[id] {
                            let dest = self.signals.get_mut(name).unwrap();
                            let result = value << *shift_amt;
                            if dest.is_none() {
                                *dest = Some(result);
                                changes_seen = true;
                            } else {
                                assert_eq!(*dest, Some(result));
                            }
                        }
                    }
                    Gate::Lshift(Value::Number(num), shift_amt, name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        let dest = self.signals.get_mut(name).unwrap();
                        let result = *num << *shift_amt;
                        if dest.is_none() {
                            *dest = Some(result);
                            changes_seen = true;
                        } else {
                            assert_eq!(*dest, Some(result));
                        }
                    }
                    Gate::Rshift(Value::Identifier(id), shift_amt, name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        if let Some(value) = self.signals[id] {
                            let dest = self.signals.get_mut(name).unwrap();
                            let result = value >> *shift_amt;
                            if dest.is_none() {
                                *dest = Some(result);
                                changes_seen = true;
                            } else {
                                assert_eq!(*dest, Some(result));
                            }
                        }
                    }
                    Gate::Rshift(Value::Number(num), shift_amt, name) => {
                        if self.overrides.contains(name) {
                            continue;
                        }
                        let dest = self.signals.get_mut(name).unwrap();
                        let result = *num >> *shift_amt;
                        if dest.is_none() {
                            *dest = Some(result);
                            changes_seen = true;
                        } else {
                            assert_eq!(*dest, Some(result));
                        }
                    }
                }
            }
            if !changes_seen {
                break;
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|res| res.map(StringWrap))
        .collect::<Result<Vec<_>, _>>()?;

    let mut circuit = lines.into_iter().collect::<anyhow::Result<Circuit>>()?;

    circuit.run(AHashMap::new());

    let a_signal = *circuit.signals.get(&"a".to_string()).unwrap();
    let a_repr = match a_signal {
        None => "--".to_string(),
        Some(x) => format!("{}", x),
    };
    println!("Part 1: Value of signal \"a\": {}", a_repr);

    let mut overrides = AHashMap::new();
    overrides.insert("b".to_string(), a_signal.unwrap());

    circuit.run(overrides);

    let a_signal = *circuit.signals.get(&"a".to_string()).unwrap();
    let a_repr = match a_signal {
        None => "--".to_string(),
        Some(x) => format!("{}", x),
    };
    println!("Part 2: Value of signal \"a\": {}", a_repr);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("NOT a -> b" => Ok(Gate::Not(Value::Identifier("a".to_string()), "b".to_string())); "NOT id")]
    #[test_case("NOT 10 -> b" => Ok(Gate::Not(Value::Number(10), "b".to_string())); "NOT num")]
    #[test_case("NOT 19999999999999999999999999999999999999999999999990 -> b" => Err("This integer is too large for a gate description".to_string()); "NOT invalid")]
    fn gate_try_from(s: &str) -> Result<Gate, String> {
        Gate::try_from(s).map_err(|e| format!("{}", e))
    }
}
