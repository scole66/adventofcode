//! # Solution for Advent of Code 2023 Day 8: Haunted Wasteland
//!
//! Ref: [Advent of Code 2023 Day 8](https://adventofcode.com/2023/day/8)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Context, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Debug;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Step {
    Left,
    Right,
}

impl TryFrom<char> for Step {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("Bad Step character {value}")),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Id(String);

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Instruction {
    left: Id,
    right: Id,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    id: Id,
    left: Id,
    right: Id,
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static NODE_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(?<id>[^ ]{3}) = \((?<left>[^ ]{3}), (?<right>[^ ]{3})\)$").unwrap());
        let caps = NODE_PATTERN.captures(s).ok_or_else(|| anyhow!("Bad node: {s}"))?;
        let id = caps.name("id").unwrap().as_str();
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();
        Ok(Node { id: Id(String::from(id)), left: Id(String::from(left)), right: Id(String::from(right)) })
    }
}

#[derive(Debug)]
struct Input {
    instructions: Vec<Step>,
    network: AHashMap<Id, Instruction>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .ok_or_else(|| anyhow!("Blank input"))?
            .chars()
            .map(Step::try_from)
            .collect::<Result<Vec<_>>>()?;
        let blank = lines
            .next()
            .ok_or_else(|| anyhow!("Missing content after instructions"))?;
        if !blank.is_empty() {
            bail!("Line after instructions must be blank");
        }
        let network = lines
            .map(|line| {
                let node = line.parse::<Node>()?;
                Ok((node.id, Instruction { left: node.left, right: node.right }))
            })
            .collect::<Result<AHashMap<_, _>>>()?;
        Ok(Input { instructions, network })
    }
}

struct PathInfo {
    nodes_before_loop: usize,
    terminations_before_loop: usize,
    nodes_within_loop: usize,
    terminations_within_loop: usize,
}

impl Input {
    fn walk(&self) -> usize {
        let mut current = Id("AAA".to_string());
        let mut steps = 0;
        let mut instructions = self.instructions.iter().copied().cycle();
        while current.0 != "ZZZ" {
            let instr = instructions.next().unwrap();
            let node = self.network.get(&current).unwrap();
            current = match instr {
                Step::Left => node.left.clone(),
                Step::Right => node.right.clone(),
            };
            steps += 1;
        }
        steps
    }

    fn ghostwalk_failure(&self) -> usize {
        let mut steps = 0;
        let mut current = self
            .network
            .keys()
            .filter(|&key| key.0.ends_with('A'))
            .cloned()
            .collect::<Vec<_>>();
        let mut instructions = self.instructions.iter().copied().cycle();

        while current.iter().any(|id| !id.0.ends_with('Z')) {
            let instr = instructions.next().unwrap();
            let next_spots = current
                .into_iter()
                .map(|id| {
                    let node = self.network.get(&id).unwrap();
                    match instr {
                        Step::Left => node.left.clone(),
                        Step::Right => node.right.clone(),
                    }
                })
                .collect::<Vec<_>>();
            current = next_spots;
            steps += 1;
        }

        steps
    }

    fn ghostwalk_info(&self, start: Id) -> PathInfo {
        let mut instructions = self.instructions.iter().copied().cycle();
        let mut cache = AHashMap::<(Id, Step), Id>::new();
        let mut loop_found = false;
        let mut state = (start, instructions.next().unwrap());
        while !loop_found {
            let ptr_next = self.network.get(&state.0).unwrap();
            let next_id = match &state.1 {
                Step::Left => ptr_next.left.clone(),
                Step::Right => ptr_next.right.clone(),
            };

            if cache.contains_key(&state) {
                loop_found = true;
            } else {
                cache.insert(state, next_id.clone());
                state = (next_id, instructions.next().unwrap());
            }
        }
        todo!()
    }

    fn ghostwalk(&self) -> usize {
        //                                                              +------------------------------------------------------+
        //                                                              V                                                      |
        // start --> (nodes, any number of which might be terminators) --> (nodes, any number of which might be terminators) --+
        //
        // That first set, call them PRE_NODES, has P items. Within them are Q terminators. PRE_TERMS[n] is the index of
        // the nth terminator in PRE_NODES.
        //
        // The second set, call them LOOP_NODES has L items. Within them are M terminators. LOOP_TERMS[n] is the index
        // of the nth terminator in LOOP_NODES.

        // terminate(n) =
        //    1 <= n <= Q : PRE_TERMS[n]
        //    Q < n : LOOP_TERMS[(n-Q-1) % M + 1] + L*floor((n-Q-1)/M)

        // if PRE_TERMS is empty, and LOOP_TERMS has only the last index (P+L), then:
        // terminate(n) =
        //    P+L + L*(n-1) = P + L*n

        // if, in addition, PRE_NODES is empty, then P = Q = 0; M = 1; and:
        // terminate(n) = L*n

        todo!()
    }
}

fn part1(input: &Input) -> usize {
    input.walk()
}

fn part2(input: &Input) -> usize {
    input.ghostwalk()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    static SAMPLE2: &str = indoc::indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test_case(SAMPLE => 2)]
    #[test_case(SAMPLE2 => 6)]
    fn part1_sample(sample: &str) -> usize {
        let input = sample.parse::<Input>().unwrap();
        part1(&input)
    }

    static SAMPLE3: &str = indoc::indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part2_sample() {
        let input = SAMPLE3.parse::<Input>().unwrap();
        assert_eq!(part2(&input), 6);
    }
}
