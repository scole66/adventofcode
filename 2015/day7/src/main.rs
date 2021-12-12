//! # Solution for Advent of Code 2015 Day 7
//!
//! Ref: [Advent of Code 2015 Day 7](https://adventofcode.com/2015/day/7)
//!

use std::io::{self, BufRead};
use ahash::AHashMap;
//use ahash::AHashSet;
use anyhow::{self};


enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Not(String, String),
    Lshift(String, u64, String),
    Rshift(String, u64, String),
    Identity(String, String),
}

impl TryFrom<&str> for Gate {
    type Error = anyhow::Error;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

const ID_PATTERN: &str = r"([a-z]+)";
const NUMBER_PATTERN: &str = r"([0-9]+)";
const INPUT_PATTERN: &str = "(NUMBER_PATTERN|ID_PATTERN)";
const INPUT_PART: &str = "(INPUT_PATTERN|(INPUT_PATTERN (AND|OR|LSHIFT|RSHIFT) INPUT_PATTERN)|(NOT INPUT_PATTERN))";
const GATE_PATTERN: &str = "(INPUT_PART -> ID_PATTERN)";



struct Circuit {
    signals: AHashMap<String, u64>,
    gates: Vec<Gate>,
}

impl<S> FromIterator<S> for Circuit
where
    S: AsRef<str>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self
    where
        S: AsRef<str>,
    {
        let gates = Vec::new();
        let signals  = AHashMap::new();
        let mut circuit = Circuit { signals, gates };


        for s in iter.into_iter() {
            let gate = Gate::try_from(s.as_ref()).unwrap();
            circuit.add(gate);
        }

        circuit
    }
}

impl Circuit {
    fn add(&mut self, _gate: Gate) {
        todo!()
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(|res| res.ok()).collect::<Vec<_>>();

    
}
