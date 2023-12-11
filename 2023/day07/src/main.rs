//! # Solution for Advent of Code 2023 Day 7: Camel Cards
//!
//! Ref: [Advent of Code 2023 Day 7](https://adventofcode.com/2023/day/7)
//!
use anyhow::{anyhow, bail, Error, Result};
use counter::Counter;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand(String);

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static HAND_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[23456789TJQKA]{5}$").unwrap());
        if !HAND_PATTERN.is_match(s) {
            bail!("Bad hand {s}");
        }
        Ok(Hand(String::from(s)))
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let chars = self.0.chars().collect::<Vec<_>>();
        let counts = chars.into_iter().collect::<Counter<_>>();
        if counts.values().any(|count| *count == 5) {
            return HandType::FiveOfAKind;
        }
        if counts.values().any(|count| *count == 4) {
            return HandType::FourOfAKind;
        }
        if counts.values().any(|count| *count == 3) {
            if counts.values().any(|count| *count == 2) {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
        let count_counts = counts.values().copied().collect::<Counter<_>>();
        if count_counts[&2] == 2 {
            return HandType::TwoPair;
        }
        if counts.values().any(|count| *count == 2) {
            return HandType::OnePair;
        }
        HandType::HighCard
    }

    fn joker_hand_type(&self) -> HandType {
        let chars = self.0.chars().collect::<Vec<_>>();
        let mut counts = chars.into_iter().collect::<Counter<_>>();
        let jokers = counts[&'J'];
        counts[&'J'] = 0;
        if counts.values().any(|&count| count + jokers == 5) {
            return HandType::FiveOfAKind;
        }
        if counts.values().any(|&count| count + jokers == 4) {
            return HandType::FourOfAKind;
        }
        let threes = counts
            .iter()
            .filter(|(_, &count)| count + jokers == 3)
            .map(|(&label, _)| label)
            .collect::<Vec<_>>();
        for &probe in threes.iter() {
            if counts.iter().any(|(&label, &count)| count == 2 && label != probe) {
                return HandType::FullHouse;
            }
        }
        if !threes.is_empty() {
            return HandType::ThreeOfAKind;
        }
        let twos = counts
            .iter()
            .filter(|(_, &count)| count + jokers == 2)
            .map(|(&label, _)| label)
            .collect::<Vec<_>>();
        for &probe in twos.iter() {
            if counts.iter().any(|(&label, &count)| count == 2 && probe != label) {
                return HandType::TwoPair;
            }
        }
        if !twos.is_empty() {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Label {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Label {
    type Error = Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        use Label::*;
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Ten),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            _ => Err(anyhow!("Bad card")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum LabelJoker {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for LabelJoker {
    type Error = Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        use LabelJoker::*;
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Joker),
            'T' => Ok(Ten),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            _ => Err(anyhow!("Bad card")),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        match self_type.cmp(&other_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let self_chars = self
                    .0
                    .chars()
                    .map(|ch| Label::try_from(ch).unwrap())
                    .collect::<Vec<_>>();
                let other_chars = other
                    .0
                    .chars()
                    .map(|ch| Label::try_from(ch).unwrap())
                    .collect::<Vec<_>>();
                self_chars.cmp(&other_chars)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct JokerHand(Hand);

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.0.joker_hand_type();
        let other_type = other.0.joker_hand_type();
        match self_type.cmp(&other_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let self_chars = self
                    .0
                     .0
                    .chars()
                    .map(|ch| LabelJoker::try_from(ch).unwrap())
                    .collect::<Vec<_>>();
                let other_chars = other
                    .0
                     .0
                    .chars()
                    .map(|ch| LabelJoker::try_from(ch).unwrap())
                    .collect::<Vec<_>>();
                self_chars.cmp(&other_chars)
            }
        }
    }
}

impl From<Hand> for JokerHand {
    fn from(value: Hand) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
struct PlayerState {
    hand: Hand,
    bid: i64,
}

impl FromStr for PlayerState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Badly formed player state {s}"))?;
        let hand = hand.parse::<Hand>()?;
        let bid = bid.parse::<i64>()?;
        Ok(PlayerState { hand, bid })
    }
}

#[derive(Debug)]
struct Input(Vec<PlayerState>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            s.lines()
                .map(|line| line.parse::<PlayerState>())
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

fn part1(input: &Input) -> i64 {
    let mut hands = input.0.iter().collect::<Vec<_>>();
    hands.sort_by_key(|&ps| &ps.hand);
    hands
        .iter()
        .enumerate()
        .map(|(index, &ps)| (index as i64 + 1) * ps.bid)
        .sum::<i64>()
}

fn part2(input: &Input) -> i64 {
    let mut hands = input.0.iter().collect::<Vec<_>>();
    hands.sort_by_key(|&ps| JokerHand::from(ps.hand.clone()));
    hands
        .iter()
        .enumerate()
        .map(|(index, &ps)| (index as i64 + 1) * ps.bid)
        .sum::<i64>()
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
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part1_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn part2_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        assert_eq!(part2(&input), 5905);
    }

    // Five of a kind, where all five cards have the same label: AAAAA
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // High card, where all cards' labels are distinct: 23456

    #[test_case("AAAAA" => HandType::FiveOfAKind)]
    #[test_case("AA8AA" => HandType::FourOfAKind)]
    #[test_case("23332" => HandType::FullHouse)]
    #[test_case("TTT98" => HandType::ThreeOfAKind)]
    #[test_case("23432" => HandType::TwoPair)]
    #[test_case("A23A4" => HandType::OnePair)]
    #[test_case("23456" => HandType::HighCard)]
    fn hand_type(hand: &str) -> HandType {
        let hand = hand.parse::<Hand>().unwrap();
        hand.hand_type()
    }
}
