//! # Solution for Advent of Code 2022 Day 13: Distress Signal
//!
//! Ref: [Advent of Code 2022 Day 13](https://adventofcode.com/2022/day/13)
//!
use std::fmt::Display;
use std::io::{self, Read};
use std::iter::{Iterator, Peekable};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Eq)]
enum Item {
    Number(i64),
    List(Vec<Item>),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Number(x) => x.fmt(f),
            Item::List(lst) => {
                write!(
                    f,
                    "[{}]",
                    lst.iter().map(|item| format!("{item}")).collect::<Vec<_>>().join(",")
                )
            }
        }
    }
}

impl Item {
    fn parse<T>(iter: &mut Peekable<T>) -> anyhow::Result<Item>
    where
        T: Iterator<Item = char>,
    {
        match iter.peek() {
            Some('[') => {
                // Start of sub-list. Discard the bracket.
                iter.next();
                let mut sublist = vec![];
                if iter.peek() == Some(&']') {
                    // an empty vector
                    iter.next();
                    return Ok(Item::List(sublist));
                }
                loop {
                    let list_item = Item::parse(iter)?;
                    sublist.push(list_item);
                    match iter.peek() {
                        Some(']') => {
                            // End of list.
                            iter.next();
                            return Ok(Item::List(sublist));
                        }
                        Some(',') => {
                            // Separator.
                            iter.next();
                        }
                        _ => {
                            anyhow::bail!("Bad parse in item");
                        }
                    }
                }
            }
            Some(&ch) if ch.is_ascii_digit() => {
                let mut number = String::from(ch);
                iter.next();
                loop {
                    let next = iter.peek();
                    match next {
                        Some(&ch) if ch.is_ascii_digit() => {
                            number.push(ch);
                            iter.next();
                        }
                        _ => {
                            return Ok(Item::Number(number.parse::<i64>()?));
                        }
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Bad parse in item")),
        }
    }
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ch_iter = s.chars().peekable();
        Item::parse(&mut ch_iter)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.partial_cmp(b),
            (Item::List(_), Item::Number(b)) => self.partial_cmp(&Item::List(vec![Item::Number(*b)])),
            (Item::Number(a), Item::List(_)) => Item::List(vec![Item::Number(*a)]).partial_cmp(other),
            (Item::List(a), Item::List(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.cmp(b),
            (Item::List(_), Item::Number(b)) => self.cmp(&Item::List(vec![Item::Number(*b)])),
            (Item::Number(a), Item::List(_)) => Item::List(vec![Item::Number(*a)]).cmp(other),
            (Item::List(a), Item::List(b)) => a.cmp(b),
        }
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    itertools::process_results(
        input
            .split("\n\n")
            .enumerate()
            .map(|(idx, pair)| {
                let mut line_iter = pair.lines();
                let line1 = line_iter.next().ok_or_else(|| anyhow::anyhow!("pair missing"))?;
                let line2 = line_iter
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("second of pair missing"))?;
                if line_iter.next().is_some() {
                    anyhow::bail!("Too many items for a pair")
                }
                let left = line1.parse::<Item>()?;
                let right = line2.parse::<Item>()?;

                Ok((idx, left < right))
            })
            .filter(|r| r.is_err() || r.as_ref().expect("not error case").1)
            .map(|r| r.map(|(idx, _)| idx + 1)),
        |iter| iter.sum(),
    )
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut items = itertools::process_results(
        input
            .lines()
            .filter(|&line| !line.is_empty())
            .map(|line| line.parse::<Item>()),
        |iter| iter.collect::<Vec<_>>(),
    )?;

    let first_divider = "[[2]]".parse::<Item>().expect("good input string");
    let second_divider = "[[6]]".parse::<Item>().expect("good input string");
    items.push(first_divider.clone());
    items.push(second_divider.clone());

    items.sort();

    let first_idx = 1 + items
        .iter()
        .position(|x| x == &first_divider)
        .expect("item must exist as we explicity added it");
    let second_idx = 1 + items
        .iter()
        .position(|x| x == &second_divider)
        .expect("item must exist as we explicity added it");

    Ok(first_idx * second_idx)
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
    use std::cmp::Ordering;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 13);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 140);
    }

    #[test_case("322" => Item::Number(322); "one number")]
    #[test_case("[]" => Item::List(vec![]); "empty list")]
    #[test_case("[1,2,3,4]" => Item::List(vec![Item::Number(1), Item::Number(2), Item::Number(3), Item::Number(4)]); "simple list")]
    #[test_case("[1,[2,3],[[]]]" => Item::List(vec![Item::Number(1), Item::List(vec![Item::Number(2), Item::Number(3)]), Item::List(vec![Item::List(vec![])])]); "nested list")]
    #[test_case("[[],5]" => Item::List(vec![Item::List(vec![]), Item::Number(5)]); "empty list followed by something else")]
    fn convert(input: &str) -> Item {
        input.parse::<Item>().unwrap()
    }

    #[test_case("[1,1,3,1,1]", "[1,1,5,1,1]" => Some(Ordering::Less))]
    #[test_case("[[1],[2,3,4]]", "[[1],4]" => Some(Ordering::Less))]
    #[test_case("[9]", "[[8,7,6]]" => Some(Ordering::Greater))]
    #[test_case("[[4,4],4,4]", "[[4,4],4,4,4]" => Some(Ordering::Less))]
    #[test_case("[7,7,7,7]", "[7,7,7]" => Some(Ordering::Greater))]
    #[test_case("[]", "[3]" => Some(Ordering::Less))]
    #[test_case("[[[]]]", "[[]]" => Some(Ordering::Greater))]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]" => Some(Ordering::Greater))]
    fn partial_ord(left: &str, right: &str) -> Option<Ordering> {
        let litem = left.parse::<Item>().unwrap();
        let ritem = right.parse::<Item>().unwrap();

        litem.partial_cmp(&ritem)
    }
}
