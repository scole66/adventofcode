//! # Solution for Advent of Code 2015 Day 5
//!
//! Ref: [Advent of Code 2015 Day 5](https://adventofcode.com/2015/day/5)
//!
//! ## --- Day 5: Doesn't He Have Intern-Elves For This? ---
//!
//! Santa needs help figuring out which strings in his text file are naughty or nice.
//!
//! A **nice string** is one with all of the following properties:
//!
//! * It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or `aeiouaeiouaeiou`.
//! * It contains at least one letter that appears twice in a row, like `xx`, `abcdde` (`dd`), or `aabbccdd` (`aa`,
//!   `bb`, `cc`, or `dd`).
//! * It does not contain the strings `ab`, `cd`, `pq`, or `xy`, even if they are part of one of the other requirements.
//!
//! For example:
//!
//! * `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`), a double letter (`...dd...`),
//!   and none of the disallowed substrings.
//! * `aaa` is nice because it has at least three vowels and a double letter, even though the letters used by different
//!   rules overlap.
//! * `jchzalrnumimnmhp` is naughty because it has no double letter.
//! * `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
//! * `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
//!
//! How many strings are nice?
//!
//! ## --- Part Two ---
//!
//! Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or
//! nice. None of the old rules apply, as they are all clearly ridiculous.
//!
//! Now, a nice string is one with all of the following properties:
//!
//! * It contains a pair of any two letters that appears at least twice in the string without overlapping, like `xyxy`
//!   (`xy`) or `aabcdefgaa` (`aa`), but not like `aaa` (`aa`, but it overlaps).
//! * It contains at least one letter which repeats with exactly one letter between them, like `xyx`, `abcdefeghi`
//!   (`efe`), or even `aaa`.
//!
//! For example:
//!
//! * `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter that repeats with exactly
//!   one letter between them (`zxz`).
//! * `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with one between, even though
//!   the letters used by each rule overlap.
//! * `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single letter between them.
//! * `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`), but no pair that appears
//!   twice.
//!
//! How many strings are nice under these new rules?

use std::io;

/// Determines whether a word is nice or not, based on the Part 1 rules.
///
/// * It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or `aeiouaeiouaeiou`.
/// * It contains at least one letter that appears twice in a row, like `xx`, `abcdde` (`dd`), or `aabbccdd` (`aa`,
///   `bb`, `cc`, or `dd`).
/// * It does not contain the strings `ab`, `cd`, `pq`, or `xy`, even if they are part of one of the other requirements.
///
/// Note the the definition of "word" is rather loose. This function treats the input string as a "word", even if it's
/// full of blanks.
///
/// # Examples:
/// ```
///     assert!(is_nice("ugknbfddgicrmopn"));
/// ```
fn is_nice(word: &str) -> bool {
    let mut vowels_seen = 0;
    let mut previous_char: Option<char> = None;
    let mut double_seen = false;
    for ch in word.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&ch) {
            vowels_seen += 1;
        }
        if Some(ch) == previous_char {
            double_seen = true;
        }
        if ch == 'b' && previous_char == Some('a')
            || ch == 'd' && previous_char == Some('c')
            || ch == 'q' && previous_char == Some('p')
            || ch == 'y' && previous_char == Some('x')
        {
            return false;
        }
        previous_char = Some(ch);
    }
    vowels_seen >= 3 && double_seen
}

/// Given a list of strings, answer the question posed in Part 1.
fn part1(lines: &[String]) {
    let num_nice = lines.iter().filter(|&line| is_nice(line)).count();
    println!("Part1: There are {num_nice} nice strings.");
}

/// Determine whether a word is nice or not, based on the Part 2 rules.
///
/// * It contains a pair of any two letters that appears at least twice in the string without overlapping, like `xyxy`
///   (`xy`) or `aabcdefgaa` (`aa`), but not like `aaa` (`aa`, but it overlaps).
/// * It contains at least one letter which repeats with exactly one letter between them, like `xyx`, `abcdefeghi`
///   (`efe`), or even `aaa`.
///
/// Note the the definition of "word" is rather loose. This function treats the input string as a "word", even if it's
/// full of blanks.
///
/// # Examples:
/// ```
///     assert!(is_super_nice("qjhvhtzxzqqjkmpb"));
///     assert!(is_super_nice("xxyxx"));
///     assert!(!is_super_nice("uurcxstgmygtbstg"));
///     assert!(!is_super_nice("ieodomkazucvgmuy"));
///     assert!(!is_super_nice("qpnxkuldeiituggg"));
/// ```
fn is_super_nice(s: &str) -> bool {
    let mut one_back: Option<char> = None;
    let mut previous_index: Option<usize> = None;
    let mut two_back: Option<char> = None;
    let mut saw_two_pairs = false;
    let mut saw_aba_style = false;
    for (idx, ch) in s.char_indices() {
        if !saw_two_pairs {
            if let Some(previous) = one_back {
                let needle = [previous, ch].into_iter().collect::<String>();
                let needle_len_in_bytes = needle.len();
                let first_occurrance = s.find(&needle).unwrap();
                if first_occurrance + needle_len_in_bytes <= previous_index.unwrap() {
                    saw_two_pairs = true;
                }
            }
        }

        if two_back == Some(ch) {
            saw_aba_style = true;
        }
        if saw_aba_style && saw_two_pairs {
            return true;
        }

        two_back = one_back;
        one_back = Some(ch);
        previous_index = Some(idx);
    }

    false
}

/// Given a list of strings, answer the question posed in Part 2.
fn part2(lines: &[String]) {
    let num_nice = lines.iter().filter(|&line| is_super_nice(line)).count();
    println!("Part2: There are {num_nice} nice (v2) strings");
}

fn main() -> io::Result<()> {
    let mut lines = Vec::<String>::new();

    loop {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        lines.push(buffer.trim().to_string());
    }

    part1(&lines);
    part2(&lines);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ugknbfddgicrmopn" => true)]
    #[test_case("aaa" => true)]
    #[test_case("jchzalrnumimnmhp" => false)]
    #[test_case("haegwjzuvuyypxyu" => false)]
    #[test_case("dvszwmarrgswjxmb" => false)]
    fn is_nice(s: &str) -> bool {
        super::is_nice(s)
    }

    #[test_case("qjhvhtzxzqqjkmpb" => true)]
    #[test_case("xxyxx" => true)]
    #[test_case("uurcxstgmygtbstg" => false)]
    #[test_case("ieodomkazucvgmuy" => false)]
    #[test_case("qpnxkuldeiituggg" => false)]
    fn is_super_nice(s: &str) -> bool {
        super::is_super_nice(s)
    }
}
