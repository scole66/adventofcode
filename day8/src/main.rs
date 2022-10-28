use ahash::AHashSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead};

type SegmentPattern = String;
type OutputValue = String;

fn parse_line(line: &str) -> Option<(Vec<SegmentPattern>, Vec<OutputValue>)> {
    lazy_static! {
        static ref VALIDATE: Regex =
            Regex::new(r"^(?P<patterns>(?:[a-g]{2,7} ){10})\|(?P<values>(?: [a-g]{2,7}){4})$").unwrap();
    }
    VALIDATE.captures(line).map(|caps| {
        (
            caps.name("patterns")
                .unwrap()
                .as_str()
                .split(' ')
                .filter_map(|s| match s {
                    "" => None,
                    s => Some(s.to_string()),
                })
                .collect::<Vec<SegmentPattern>>(),
            caps.name("values")
                .unwrap()
                .as_str()
                .split(' ')
                .filter_map(|s| match s {
                    "" => None,
                    s => Some(s.to_string()),
                })
                .collect::<Vec<OutputValue>>(),
        )
    })
}

fn count_easy_digits(input: &[(Vec<SegmentPattern>, Vec<OutputValue>)]) -> usize {
    input
        .iter()
        .map(|(_, values)| values)
        .map(|values| values.iter().filter(|&s| [2, 3, 4, 7].contains(&s.len())).count())
        .sum()
}

fn decode(patterns: &[SegmentPattern], values: &[OutputValue]) -> u32 {
    //   0:      1:      2:      3:      4:
    //  ####    ....    ####    ####    ....
    // #    #  .    #  .    #  .    #  #    #
    // #    #  .    #  .    #  .    #  #    #
    //  ....    ....    ####    ####    ####
    // #    #  .    #  #    .  .    #  .    #
    // #    #  .    #  #    .  .    #  .    #
    //  ####    ....    ####    ####    ....
    //
    //   5:      6:      7:      8:      9:
    //  ####    ####    ####    ####    ####
    // #    .  #    .  .    #  #    #  #    #
    // #    .  #    .  .    #  #    #  #    #
    //  ####    ####    ....    ####    ####
    // .    #  #    #  .    #  #    #  .    #
    // .    #  #    #  .    #  #    #  .    #
    //  ####    ####    ....    ####    ####

    // Pattern Length   | Possible number
    //    2             |    1
    //    3             |    7
    //    4             |    4
    //    5             |    2, 3, 5
    //    6             |    0, 6, 9
    //    7             |    8

    let mut pats = Vec::from(patterns);
    pats.sort_by_key(|s| s.len());

    let one: AHashSet<char> = AHashSet::from_iter(pats[0].chars());
    let seven: AHashSet<char> = AHashSet::from_iter(pats[1].chars());
    let four: AHashSet<char> = AHashSet::from_iter(pats[2].chars());
    let eight: AHashSet<char> = AHashSet::from_iter(pats[9].chars());
    let two_three_five: [AHashSet<char>; 3] = [
        AHashSet::from_iter(pats[3].chars()),
        AHashSet::from_iter(pats[4].chars()),
        AHashSet::from_iter(pats[5].chars()),
    ];
    let three = two_three_five.iter().find(|set| one.is_subset(set)).unwrap();
    let two_five = two_three_five.iter().filter(|&set| set != three).collect::<Vec<_>>();
    assert_eq!(two_five.len(), 2);
    let zero_six_nine: [AHashSet<char>; 3] = [
        AHashSet::from_iter(pats[6].chars()),
        AHashSet::from_iter(pats[7].chars()),
        AHashSet::from_iter(pats[8].chars()),
    ];
    let six = zero_six_nine.iter().find(|set| !set.is_superset(&one)).unwrap();
    let (five, two) = if six.is_superset(two_five[0]) {
        (two_five[0], two_five[1])
    } else {
        (two_five[1], two_five[0])
    };
    let zero_nine = zero_six_nine.iter().filter(|&set| set != six).collect::<Vec<_>>();
    assert_eq!(zero_nine.len(), 2);
    let zero = if four.is_subset(zero_nine[0]) {
        zero_nine[1]
    } else {
        zero_nine[0]
    };

    assert_eq!(values.len(), 4);

    values
        .iter()
        .map(|s| match AHashSet::<char>::from_iter(s.chars()) {
            s if s == *zero => 0,
            s if s == one => 1,
            s if s == *two => 2,
            s if s == *three => 3,
            s if s == four => 4,
            s if s == *five => 5,
            s if s == *six => 6,
            s if s == seven => 7,
            s if s == eight => 8,
            _ => 9,
        })
        .reduce(|accum, digit| accum * 10 + digit)
        .unwrap()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let input: Vec<(Vec<SegmentPattern>, Vec<OutputValue>)> = stdin
        .lock()
        .lines()
        .filter_map(|res| res.ok())
        .filter_map(|s| parse_line(&s))
        .collect();

    // Part 1: In the output values, how often do the digits 1, 4, 7, and 8 appear?
    println!(
        "Part 1: Easy Digits appear {} times in the output values",
        count_easy_digits(&input)
    );

    // Part 2: Sum the decoded values
    let answer: u32 = input.iter().map(|(patterns, values)| decode(patterns, values)).sum();
    println!("Part 2: Sum of decoded values: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test_case::test_case;

    static SAMPLE: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn part1() {
        let input = SAMPLE.iter().filter_map(|&s| parse_line(s)).collect::<Vec<_>>();
        assert_eq!(count_easy_digits(&input), 26);
    }

    #[test]
    fn part2() {
        let input = SAMPLE.iter().filter_map(|&s| parse_line(s)).collect::<Vec<_>>();
        assert_eq!(
            input
                .iter()
                .map(|(patterns, values)| decode(patterns, values))
                .collect::<Vec<_>>(),
            vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315]
        );
    }
}
