//! # Solution for Advent of Code 2022 Day 5: Supply Stacks
//!
//! Ref: [Advent of Code 2022 Day 5](https://adventofcode.com/2022/day/5)
//!
use ahash::AHashMap;
use anyhow::{self, Context};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};

struct Input {
    stack_lines: Vec<String>,
    stack_identifiers: String,
    instruction_lines: Vec<String>,
}

struct Instruction {
    amount: usize,
    from: String,
    to: String,
}
struct Model {
    stacks: AHashMap<String, Vec<char>>,
    identifiers: Vec<String>,
    instructions: Vec<Instruction>,
}

static STACK_CONTENT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    // Matches an entire "initial stack" line.
    Regex::new(r"^(((   )|(\[[A-Z]\])) )*((   )|(\[[A-Z]\]))$").expect("Hand-rolled regex is valid")
});
static CONTENT_PIECE: Lazy<Regex> = Lazy::new(|| {
    // Matches one stack's worth of space in a line. Note that this is not a "whole line" regex; the intent is
    // to use it with captures_iter to individually capture the values across the whole line. (Captures on
    // STACK_CONTENT_PATTERN itself don't help, as the capture of a repeated element only captures the final
    // repetition.) Because of the "non-capturing" groups here, the only things you'll get from this are "   "
    // (the nothing-there value), or a one-character string with the box name.
    Regex::new(r"(?:(   )|(?:\[([A-Z])\])) ?").expect("Hand-rolled regex is valid")
});
static STACK_IDENTIFIER_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?: *([^ ]+))+ *$").expect("Hand-rolled regex is valid"));
static IDENTIFIER_PIECE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^ ]+").expect("Hand-rolled regex is valid"));
static INSTRUCTION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^move (?P<amount>0|(?:[1-9][0-9]*)) from (?P<from>[^ ]+) to (?P<to>[^ ]+)$")
        .expect("Hand-rolled regex is valid")
});

impl FromIterator<MaybeLine> for anyhow::Result<Input> {
    fn from_iter<T: IntoIterator<Item = MaybeLine>>(iter: T) -> Self {
        enum Expecting {
            ContentOrIdentifier,
            Blank,
            Instructions,
        }
        let mut stage = Expecting::ContentOrIdentifier;
        let mut stack_content = vec![];
        let mut instructions = vec![];
        let mut ids = None;
        for maybe in iter.into_iter() {
            let line = maybe.0?;
            match stage {
                Expecting::ContentOrIdentifier => {
                    if STACK_CONTENT_PATTERN.is_match(&line) {
                        stack_content.push(line);
                    } else if STACK_IDENTIFIER_PATTERN.is_match(&line) {
                        ids = Some(line);
                        stage = Expecting::Blank;
                    } else {
                        anyhow::bail!("Expect stack data or an identifier line");
                    }
                }
                Expecting::Blank => {
                    if line.is_empty() {
                        stage = Expecting::Instructions;
                    } else {
                        anyhow::bail!("Expected a blank line")
                    }
                }
                Expecting::Instructions => {
                    if INSTRUCTION_PATTERN.is_match(&line) {
                        instructions.push(line);
                    } else {
                        anyhow::bail!("Expected some instructions")
                    }
                }
            }
        }
        Ok(Input {
            stack_lines: stack_content,
            stack_identifiers: ids.ok_or_else(|| anyhow::anyhow!("stack identifiers missing"))?,
            instruction_lines: instructions,
        })
    }
}

impl TryFrom<&Input> for Model {
    type Error = anyhow::Error;
    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let mut stacks = AHashMap::new();
        let identifiers = IDENTIFIER_PIECE
            .captures_iter(&input.stack_identifiers)
            .map(|cap| cap[0].to_string())
            .collect::<Vec<_>>();
        for name in identifiers.iter() {
            stacks.insert(name.clone(), vec![]);
        }
        for stack_content in input.stack_lines.iter().rev() {
            for (item, stack_id) in CONTENT_PIECE
                .captures_iter(stack_content)
                .flat_map(|c| c.iter().skip(1).flatten().map(|m| m.as_str()).collect::<Vec<_>>())
                .zip(identifiers.iter())
            {
                if item != "   " {
                    let stack = stacks.get_mut(stack_id).expect("stack exists, as we created it, above");
                    stack.push(
                        item.chars()
                            .next()
                            .expect("Item Ids are by definition 1 char, enforced by regex"),
                    );
                }
            }
        }
        let mut instructions = vec![];
        for command in input.instruction_lines.iter() {
            let details = INSTRUCTION_PATTERN
                .captures(command)
                .expect("we already know these lines match");
            instructions.push(Instruction {
                amount: details
                    .name("amount")
                    .expect("regex guarantees this is here")
                    .as_str()
                    .parse::<usize>()?,
                from: details
                    .name("from")
                    .expect("regex guarantees this is here")
                    .as_str()
                    .to_string(),
                to: details
                    .name("to")
                    .expect("regex guarantees this is here")
                    .as_str()
                    .to_string(),
            })
        }
        Ok(Model {
            stacks,
            identifiers,
            instructions,
        })
    }
}

fn top_boxes(model: &Model) -> anyhow::Result<String> {
    let mut result = String::new();
    for id in model.identifiers.iter() {
        let stack = model.stacks.get(id).expect("constructor guarantee");
        if stack.is_empty() {
            anyhow::bail!("No Crates remaining in stack {}!", id);
        }
        let stacklen = stack.len();
        result.push(stack[stacklen - 1]);
    }

    Ok(result)
}

fn part1(input: &Input) -> anyhow::Result<String> {
    let mut model = Model::try_from(input)?;
    for insn in model.instructions.iter() {
        for _ in 0..insn.amount {
            let from_stack = model.stacks.get_mut(&insn.from).expect("constructor guarantee");
            let value = from_stack
                .pop()
                .ok_or_else(|| anyhow::anyhow!("No Crates remaining in stack {}!", insn.from))?;
            let to_stack = model.stacks.get_mut(&insn.to).expect("constructor guarantee");
            to_stack.push(value);
        }
    }

    top_boxes(&model)
}

fn part2(input: &Input) -> anyhow::Result<String> {
    let mut model = Model::try_from(input)?;
    for insn in model.instructions.iter() {
        let from_stack = model.stacks.get_mut(&insn.from).expect("constructor guarantee");
        if insn.amount > from_stack.len() {
            anyhow::bail!("Not enough crates in stack {}!", insn.from);
        }
        let split_index = from_stack.len() - insn.amount;
        let values = from_stack.split_off(split_index);
        let to_stack = model.stacks.get_mut(&insn.to).expect("constructor guarantee");
        to_stack.extend(values);
    }

    top_boxes(&model)
}

struct MaybeLine(anyhow::Result<String>);
impl From<Result<String, std::io::Error>> for MaybeLine {
    fn from(s: Result<String, std::io::Error>) -> Self {
        Self(s.map_err(anyhow::Error::from))
    }
}
impl From<&str> for MaybeLine {
    fn from(s: &str) -> Self {
        Self(Ok(s.to_string()))
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(MaybeLine::from)
        .collect::<anyhow::Result<Input>>()
        .context("Failed to parse puzzle input from stdin")?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn part1_sample() {
        let input = SAMPLE
            .lines()
            .map(MaybeLine::from)
            .collect::<anyhow::Result<Input>>()
            .unwrap();
        let result = part1(&input).unwrap();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_sample() {
        let input = SAMPLE
            .lines()
            .map(MaybeLine::from)
            .collect::<anyhow::Result<Input>>()
            .unwrap();
        let result = part2(&input).unwrap();
        assert_eq!(result, "MCD");
    }
}
