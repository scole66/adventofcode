//! # Solution for Advent of Code 2021 Day 19
//!
//! Ref: [Advent of Code 2021 Day 19](https://adventofcode.com/2021/day/19)
//!
#![allow(unused_imports, dead_code, unused_variables)]

use ahash::AHashSet;
use anyhow::{self, Context};
use once_cell::sync::Lazy;
use regex::Regex;
//use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coords(i32, i32, i32);

impl TryFrom<String> for Coords {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let potentials: Vec<_> = value.split(',').collect();
        if potentials.len() != 3 {
            Err(anyhow::anyhow!("Coordinates must be three things; saw: '{value}'"))
        } else {
            fn cvt(item: &str) -> Result<i32, anyhow::Error> {
                item.parse::<i32>().map_err(|err| anyhow::anyhow!(err.to_string()))
            }
            Ok(Coords(cvt(potentials[0])?, cvt(potentials[1])?, cvt(potentials[2])?))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Scanner {
    id: String,
    beacons: AHashSet<Coords>,
}

impl Scanner {
    fn new(name: &str) -> Self {
        Scanner { id: name.to_string(), beacons: AHashSet::new() }
    }

    fn distances(&self) -> Vec<f64> {
        for head in self.beacons.iter() {
            for tail in 
        }
    }

    fn shared_beacons(&self, other: &Scanner) -> AHashSet<Coords> {
        
    }
}

/// A NewType wrapping an `anyhow::Result<String>`
///
/// This is really nothing more than a new type created so that we can implement what would otherwise be
/// `FromIterator<anyhow::Result<String>> for anyhow::Result<Scanner>`.
#[derive(Debug)]
struct LineResult(anyhow::Result<String>);
impl From<anyhow::Result<String>> for LineResult {
    /// Converts an `anyhow::Result<String>` into a `LineResult`
    fn from(src: anyhow::Result<String>) -> Self {
        Self(src)
    }
}
impl From<Result<String, std::io::Error>> for LineResult {
    /// Converts a `Result<String, std::io::Error>` into a `LineResult`
    fn from(src: Result<String, std::io::Error>) -> Self {
        Self(src.map_err(anyhow::Error::from))
    }
}
impl From<&str> for LineResult {
    fn from(src: &str) -> Self {
        Self(Ok(src.to_string()))
    }
}

impl FromIterator<LineResult> for anyhow::Result<Vec<Scanner>> {
    fn from_iter<T: IntoIterator<Item = LineResult>>(iter: T) -> Self {
        static SCANNER_ID_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^--- scanner (?P<id>.+) ---$").unwrap());
        let mut scanners: Vec<Scanner> = vec![];
        let mut collecting_points = false;
        for LineResult(res) in iter.into_iter() {
            let line = res?;
            if collecting_points {
                if line.is_empty() {
                    collecting_points = false;
                } else {
                    let coords = Coords::try_from(line)?;
                    let idx = scanners.len() - 1;
                    assert!(
                        !scanners.is_empty(),
                        "Coding Error: scanners array cannot be empty here"
                    );
                    scanners[idx].beacons.insert(coords);
                }
            } else {
                let id = SCANNER_ID_PATTERN
                    .captures(&line)
                    .ok_or_else(|| anyhow::anyhow!("cannot parse '{line}' as a scanner identifier"))?
                    .name("id")
                    .expect("'id' must be present if regex matched")
                    .as_str();
                scanners.push(Scanner::new(id));
                collecting_points = true;
            }
        }
        Ok(scanners)
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct UnderSea {
    beacons: AHashSet<Coords>,
    scanners: AHashSet<Coords>,
}

impl UnderSea {
    fn beacon_count(&self) -> usize {
        self.beacons.len()
    }
    fn sensor_count(&self) -> usize {
        self.scanners.len()
    }
    fn new() -> Self {
        Self::default()
    }
}

fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    fn inner<T: Clone>(items: &mut [T], size: usize, results: &mut Vec<Vec<T>>) {
        if size == 1 {
            results.push(items.to_vec())
        } else {
            for i in 0..size {
                inner(items, size - 1, results);
                let swap_idx = if size % 2 == 1 { 0 } else { i };
                items.swap(swap_idx, size - 1);
            }
        }
    }
    let mut results = vec![];
    let mut items = items.to_vec();
    let size = items.len();
    inner(&mut items, size, &mut results);
    results
}

fn variations(original: &[[i8; 3]]) -> Vec<Vec<i8>> {
    let mut result = vec![];
    result.push(original.iter().flatten().cloned().collect::<Vec<i8>>());
    // The other items are ones where two of the '1's have been changed to '-1'.
    for target in 1..=3 {
        let mut one_count = 0;
        result.push(
            original
                .iter()
                .flatten()
                .map(|&digit| match digit {
                    1 => {
                        one_count += 1;
                        if one_count == target {
                            1
                        } else {
                            -1
                        }
                    }
                    _ => digit,
                })
                .collect::<Vec<i8>>(),
        );
    }
    result
}

fn construct_facing_matrices() -> [[i8; 9]; 24] {
    let mut result = [[0_i8; 9]; 24];

    let mut row_offset = 0;
    for matrix in permutations(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]) {
        for (row, facing) in variations(&matrix).into_iter().enumerate() {
            assert_eq!(facing.len(), 9);
            assert!(row < 4);
            for (column, digit) in facing.into_iter().enumerate() {
                result[row + row_offset][column] = digit;
            }
        }
        row_offset += 4;
    }

    result
}

static FACES: Lazy<[[i8; 9]; 24]> = Lazy::new(construct_facing_matrices);

struct WorkItem {
    beacons: AHashSet<Coords>,
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(LineResult::from)
        .collect::<anyhow::Result<Vec<Scanner>>>()
        .context("Failed to parse input from stdin")?;

    Ok(())
}

#[cfg(test)]
#[test]
fn scanner_from_string_array() {
    let source = vec![
        "--- scanner 0 ---",
        "404,-588,-901",
        "528,-643,409",
        "-838,591,734",
        "",
        "--- scanner 1 ---",
        "686,422,578",
        "605,423,415",
        "515,917,-361",
    ];

    let converted = source
        .into_iter()
        .map(LineResult::from)
        .collect::<anyhow::Result<Vec<Scanner>>>()
        .unwrap();

    assert_eq!(
        converted,
        vec![
            Scanner {
                id: "0".to_string(),
                beacons: AHashSet::from_iter(
                    vec![Coords(404, -588, -901), Coords(528, -643, 409), Coords(-838, 591, 734)].into_iter()
                )
            },
            Scanner {
                id: "1".to_string(),
                beacons: AHashSet::from_iter(
                    vec![Coords(686, 422, 578), Coords(605, 423, 415), Coords(515, 917, -361)].into_iter()
                )
            }
        ]
    );
}

#[test]
fn header_error() {
    let source = vec!["invalid syntax", "drives people crazy"];

    let converted = source
        .into_iter()
        .map(LineResult::from)
        .collect::<anyhow::Result<Vec<Scanner>>>()
        .unwrap_err();

    assert_eq!(
        converted.to_string(),
        "cannot parse 'invalid syntax' as a scanner identifier"
    );
}

#[test]
fn count_error() {
    let source = vec!["--- scanner bob ---", "1,2,3", "4,5"];

    let converted = source
        .into_iter()
        .map(LineResult::from)
        .collect::<anyhow::Result<Vec<Scanner>>>()
        .unwrap_err();

    assert_eq!(converted.to_string(), "Coordinates must be three things; saw: '4,5'");
}

#[test]
fn integer_error() {
    let source = vec!["--- scanner bob ---", "1,2,3", "4,5,6", "-3,-11,elephant"];

    let converted = source
        .into_iter()
        .map(LineResult::from)
        .collect::<anyhow::Result<Vec<Scanner>>>()
        .unwrap_err();

    assert_eq!(converted.to_string(), "invalid digit found in string");
}

mod permutations {
    #[test]
    fn permutations() {
        let input = &[1, 2, 3];
        let output = super::permutations(input);
        assert_eq!(output.len(), 6);
        assert!(output.contains(&vec![1, 2, 3]));
        assert!(output.contains(&vec![2, 1, 3]));
        assert!(output.contains(&vec![3, 2, 1]));
        assert!(output.contains(&vec![1, 3, 2]));
        assert!(output.contains(&vec![2, 3, 1]));
        assert!(output.contains(&vec![3, 1, 2]));
    }
}

mod variations {
    #[test]
    fn variations() {
        let input = vec![[0, 1, 0], [1, 0, 0], [0, 0, 1]];
        let output = super::variations(&input);
        assert_eq!(output.len(), 4);
        assert!(output.contains(&vec![0, 1, 0, 1, 0, 0, 0, 0, 1]));
        assert!(output.contains(&vec![0, 1, 0, -1, 0, 0, 0, 0, -1]));
        assert!(output.contains(&vec![0, -1, 0, 1, 0, 0, 0, 0, -1]));
        assert!(output.contains(&vec![0, -1, 0, -1, 0, 0, 0, 0, 1]));
    }
}

mod intersection {
    use super::*;
    use indoc::indoc;
    #[test]
    fn from_problem_statement() {
        let input = indoc! {"
            --- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401
            
            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390
        "}
        .lines()
        .map(LineResult::from)
        .collect::<anyhow::Result<Vec<Scanner>>>()
        .unwrap();

        let result = input[0].shared_beacons(&input[1]);
        let expected = vec![
            Coords(-618, -824, -621),
            Coords(-537, -823, -458),
            Coords(-447, -329, 318),
            Coords(404, -588, -901),
            Coords(544, -627, -890),
            Coords(528, -643, 409),
            Coords(-661, -816, -575),
            Coords(390, -675, -793),
            Coords(423, -701, 434),
            Coords(-345, -311, 381),
            Coords(459, -707, 401),
            Coords(-485, -357, 347),
        ];

        assert_eq!(result.len(), expected.len());
        assert!(expected.iter().all(|coord| result.contains(coord)));
    }
}
