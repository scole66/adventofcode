//! # Solution for Advent of Code 2022 Day 11: Monkey in the Middle
//!
//! Ref: [Advent of Code 2022 Day 11](https://adventofcode.com/2022/day/11)
//!
use ahash::AHashMap;
use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use std::io::{self, BufRead};
use std::iter::Iterator;

static MONKEY_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^Monkey (?P<id>[0-9]+):$").expect("Hand-rolled regex is valid"));
static ITEMS_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^  Starting items: (?P<items>[0-9]+(?:, [0-9]+)*)$").unwrap());
static OPS_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^  Operation: new = old (?P<op>[*+]) (?P<val>0|[1-9][0-9]*|old)$").unwrap());
static TEST_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^  Test: divisible by (?P<val>[1-9][0-9]*)$").unwrap());
static REACTION_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^    If (?P<state>true|false): throw to monkey (?P<id>[0-9]+)$").unwrap());

#[derive(Debug)]
enum Operand {
    Old,
    Number(i64),
}

#[derive(Debug)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

#[derive(Debug)]
struct Reaction {
    truish: i64,
    falsish: i64,
}

#[derive(Debug)]
struct Monkey {
    id: i64,
    initial_items: Vec<i64>,
    items: Vec<i64>,
    operation: Operation,
    test_divisor: i64,
    reaction: Reaction,
    inspection_count: usize,
}

impl Monkey {
    fn reset(&mut self) {
        self.inspection_count = 0;
        self.items = self.initial_items.clone();
    }
}

struct RString(anyhow::Result<String>);
impl<T> From<Result<String, T>> for RString
where
    T: Error + Send + Sync + 'static,
{
    fn from(r: Result<String, T>) -> Self {
        RString(r.map_err(anyhow::Error::from))
    }
}
impl From<&str> for RString {
    fn from(s: &str) -> Self {
        RString(Ok(s.to_string()))
    }
}

fn parse_monkey(input: &mut impl Iterator<Item = RString>) -> anyhow::Result<Option<Monkey>> {
    // Swallow any blank lines
    let first_line = loop {
        let maybe_line = input.next();
        match maybe_line {
            None => return Ok(None),
            Some(rstr) => {
                let line = rstr.0?;
                if !line.is_empty() {
                    break line;
                }
            }
        }
    };
    // Monkey Identifier
    let monkey_id = MONKEY_PATTERN
        .captures(&first_line)
        .ok_or_else(|| anyhow::anyhow!("Not a monkey ID marker: \"{first_line}\""))?["id"]
        .parse::<i64>()?;

    // Starting Items
    let item_line = input
        .next()
        .ok_or_else(|| anyhow::anyhow!("Item list expected: saw end-of-chunk"))?
        .0?;
    let item_string = &ITEMS_PATTERN
        .captures(&item_line)
        .ok_or_else(|| anyhow::anyhow!("Item list expected: \"{item_line}\""))?["items"];
    let items = item_string
        .split(", ")
        .map(|num| num.parse::<i64>().map_err(anyhow::Error::from))
        .collect::<anyhow::Result<Vec<i64>>>()?;
    // Operation
    let operation_line = input
        .next()
        .ok_or_else(|| anyhow::anyhow!("Operation expected; saw end-of-chunk"))?
        .0?;
    let caps = OPS_PATTERN
        .captures(&operation_line)
        .ok_or_else(|| anyhow::anyhow!("Operation expected; saw \"{operation_line}\""))?;
    let value_str = &caps["val"];
    let operand = if value_str == "old" {
        Operand::Old
    } else {
        Operand::Number(value_str.parse::<i64>()?)
    };
    let operation = match &caps["op"] {
        "*" => Operation::Multiply(operand),
        _ => Operation::Add(operand),
    };
    // Test
    let test_line = input
        .next()
        .ok_or_else(|| anyhow::anyhow!("Test expected; saw end-of-chunk"))?
        .0?;
    let test_divisor = TEST_PATTERN
        .captures(&test_line)
        .ok_or_else(|| anyhow::anyhow!("Test expected; saw \"{test_line}\""))?["val"]
        .parse::<i64>()?;
    // First Reaction
    let reaction_line = input
        .next()
        .ok_or_else(|| anyhow::anyhow!("Reaction expected; saw end-of-chunk"))?
        .0?;
    let caps = REACTION_PATTERN
        .captures(&reaction_line)
        .ok_or_else(|| anyhow::anyhow!("Reaction expected; saw \"{reaction_line}\""))?;
    let first_state_truish = &caps["state"] == "true";
    let first_target = caps["id"].parse::<i64>()?;
    // Second Reaction
    let reaction_line = input
        .next()
        .ok_or_else(|| anyhow::anyhow!("Reaction expected; saw end-of-chunk"))?
        .0?;
    let caps = REACTION_PATTERN
        .captures(&reaction_line)
        .ok_or_else(|| anyhow::anyhow!("Reaction expected; saw \"{reaction_line}\""))?;
    let second_state_truish = &caps["state"] == "true";
    let second_target = caps["id"].parse::<i64>()?;

    if first_state_truish == second_state_truish {
        anyhow::bail!("Reactions must have different true/false markers");
    }

    let reaction = if first_state_truish {
        Reaction {
            truish: first_target,
            falsish: second_target,
        }
    } else {
        Reaction {
            truish: second_target,
            falsish: first_target,
        }
    };

    Ok(Some(Monkey {
        id: monkey_id,
        initial_items: items.clone(),
        items,
        operation,
        test_divisor,
        reaction,
        inspection_count: 0,
    }))
}

struct Barrel {
    // Because the term for a collection of monkeys is _obviously_ a barrel.
    monkeys: AHashMap<i64, Monkey>,
    ids: Vec<i64>, // the sorted list of ids
    lcm: i64,      // Least common multiple of all the divisors.
}

fn parse_monkeys(iter: &mut impl Iterator<Item = RString>) -> anyhow::Result<Barrel> {
    let mut monkeys = AHashMap::new();
    loop {
        let monkey = parse_monkey(iter)?;
        match monkey {
            None => break,
            Some(monkey) => {
                monkeys.insert(monkey.id, monkey);
            }
        }
    }
    let mut ids = monkeys.keys().copied().collect::<Vec<_>>();
    ids.sort();
    let lcm = monkeys.values().map(|monkey| monkey.test_divisor).product();
    Ok(Barrel { monkeys, ids, lcm })
}

impl Barrel {
    fn round(&mut self, worry_divisor: Option<i64>) {
        for monkey_id in self.ids.iter() {
            let monkey = self.monkeys.get(monkey_id).unwrap();
            let items = monkey.items.clone(); // This gets cloned so we can keep it and let the monkey ref get dropped.
            for item in items {
                // For Rust mutability/ownership reasons, we need to get the monkey from the hash table each
                // iteration. (We get a mutable ref to our target monkey at the bottom of the loop; in order to do
                // that, all immutable refs need to be out of scope, which means we can't hold the current monkey
                // between iterations.)
                let monkey = self.monkeys.get(monkey_id).unwrap();
                let worry_level = match &monkey.operation {
                    Operation::Add(operand) => match operand {
                        Operand::Old => item + item,
                        Operand::Number(v) => item + v,
                    },
                    Operation::Multiply(operand) => match operand {
                        Operand::Old => item * item,
                        Operand::Number(v) => item * v,
                    },
                };
                let adjusted_worry = match worry_divisor {
                    Some(divisor) => worry_level / divisor,
                    None => worry_level % self.lcm,
                };

                let target = if adjusted_worry % monkey.test_divisor == 0 {
                    monkey.reaction.truish
                } else {
                    monkey.reaction.falsish
                };
                let target = self.monkeys.get_mut(&target).unwrap();
                target.items.push(adjusted_worry);
            }
            let monkey = self.monkeys.get_mut(monkey_id).unwrap();
            monkey.inspection_count += monkey.items.len();
            monkey.items.clear();
        }
    }

    fn reset(&mut self) {
        self.monkeys.values_mut().for_each(|monkey| monkey.reset());
    }

    fn monkey_business(&self) -> usize {
        let mut stats = self
            .monkeys
            .values()
            .map(|monkey| monkey.inspection_count)
            .collect::<Vec<_>>();
        stats.sort();
        stats.iter().rev().take(2).product()
    }
}

fn part1(input: &mut Barrel) -> usize {
    // 20 rounds
    for _ in 0..20 {
        input.round(Some(3));
    }
    input.monkey_business()
}

fn part2(barrel: &mut Barrel) -> usize {
    // 10,000 rounds
    for _ in 0..10000 {
        barrel.round(None);
    }
    barrel.monkey_business()
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input_iter = stdin.lock().lines().map(RString::from);
    let mut barrel = parse_monkeys(&mut input_iter)?;

    println!("Part1: {}", part1(&mut barrel));
    barrel.reset();
    println!("Part2: {}", part2(&mut barrel));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    #[test]
    fn part1_sample() {
        let mut iter = SAMPLE.lines().map(RString::from);
        let mut monkeys = parse_monkeys(&mut iter).unwrap();
        assert_eq!(part1(&mut monkeys), 10605);
    }

    #[test]
    fn part2_sample() {
        let mut iter = SAMPLE.lines().map(RString::from);
        let mut barrel = parse_monkeys(&mut iter).unwrap();
        assert_eq!(part2(&mut barrel), 2713310158);
    }
}
