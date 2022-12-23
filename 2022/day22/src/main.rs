//! # Solution for Advent of Code 2022 Day 22: Monkey Map
//!
//! Ref: [Advent of Code 2022 Day 22](https://adventofcode.com/2022/day/22)
//!
#![allow(dead_code, unused_imports, unused_variables)]
use ahash::{AHashMap, AHashSet};
use anyhow::Context;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

enum OneOfSix {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    row: i64,
    col: i64,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Constraint {
    Free,
    Wall,
}
#[derive(Debug)]
struct Map {
    points: AHashMap<Point, Constraint>,
    top_left: Point,     // of bounding box. There might not be a key here
    bottom_right: Point, // ditto
}
impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = AHashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    ' ' => (),
                    '.' => {
                        map.insert(Point { col: col.try_into()?, row: row.try_into()? }, Constraint::Free);
                    }
                    '#' => {
                        map.insert(Point { col: col.try_into()?, row: row.try_into()? }, Constraint::Wall);
                    }
                    _ => {
                        anyhow::bail!("Bad character in map: {ch}");
                    }
                }
            }
        }

        // Calculate the bounding box (so we don't do it more often than we need to)
        let mut top = i64::MAX;
        let mut bottom = i64::MIN;
        let mut leftmost = i64::MAX;
        let mut rightmost = i64::MIN;
        for key in map.keys() {
            leftmost = leftmost.min(key.col);
            rightmost = rightmost.max(key.col);
            top = top.min(key.row);
            bottom = bottom.max(key.row);
        }

        Ok(Map {
            points: map,
            top_left: Point { col: leftmost, row: top },
            bottom_right: Point { col: rightmost, row: bottom },
        })
    }
}
use once_cell::sync::Lazy;
use std::ops::Range;
impl Map {
    fn start_location(&self) -> Option<Point> {
        let lowest_row = self.top_left.row;
        self.points
            .keys()
            .filter(|&item| item.row == lowest_row)
            .min_by_key(|&item| item.col)
            .copied()
    }

    fn next_spot(&self, from: Point, facing: Facing, is_cube: bool) -> (Point, Facing) {
        let Point { row, col } = from;
        let (column_delta, row_delta) = match facing {
            Facing::Up => (0, -1),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Right => (1, 0),
        };
        let probe = Point { row: row + row_delta, col: col + column_delta };
        if self.points.contains_key(&probe) {
            return (probe, facing);
        }

        if !is_cube {
            let compare = match facing {
                Facing::Up => |pt1: &&Point, pt2: &&Point| (*pt1).row.cmp(&(*pt2).row),
                Facing::Down => |pt1: &&Point, pt2: &&Point| (*pt2).row.cmp(&(*pt1).row),
                Facing::Left => |pt1: &&Point, pt2: &&Point| (*pt1).col.cmp(&(*pt2).col),
                Facing::Right => |pt1: &&Point, pt2: &&Point| (*pt2).col.cmp(&(*pt1).col),
            };
            let filter = match facing {
                Facing::Up | Facing::Down => |pt: &&Point, _: i64, col: i64| (*pt).col == col,
                Facing::Left | Facing::Right => |pt: &&Point, row: i64, _: i64| (*pt).row == row,
            };

            (
                *self
                    .points
                    .keys()
                    .filter(|pt| filter(pt, row, col))
                    .max_by(compare)
                    .unwrap(),
                facing,
            )
        } else {
            // Oh, there are _rules_ if this is a cube:
            println!("Bounding Points: {:?}; {:?}", self.top_left, self.bottom_right);
            assert_eq!(self.top_left.col, 0);
            assert_eq!(self.top_left.row, 0);
            assert_eq!((self.bottom_right.row + 1) % 3, 0);
            assert_eq!((self.bottom_right.col + 1) % 4, 0);
            assert_eq!((self.bottom_right.row + 1) / 3, (self.bottom_right.col + 1) / 4);

            let face_size = (self.bottom_right.row + 1) / 3;
            match (self.cube_face(from), facing) {
                (OneOfSix::One, Facing::Up) => {
                    assert_eq!(row, 0);
                    (Point { col: 3 * face_size - col - 1, row: face_size }, Facing::Down)
                }
                (OneOfSix::One, Facing::Down) => unreachable!(),
                (OneOfSix::One, Facing::Left) => {
                    assert_eq!(col, face_size * 2);
                    (Point { col: face_size + row, row: face_size }, Facing::Down)
                }
                (OneOfSix::One, Facing::Right) => {
                    assert_eq!(col, face_size * 3 - 1);
                    (
                        Point { col: face_size * 4 - 1, row: 3 * face_size - row - 1 },
                        Facing::Left,
                    )
                }
                (OneOfSix::Two, Facing::Up) => {
                    assert_eq!(row, face_size);
                    (Point { col: 3 * face_size - col - 1, row: 0 }, Facing::Down)
                }
                (OneOfSix::Two, Facing::Down) => {
                    assert_eq!(row, 2 * face_size - 1);
                    (
                        Point { col: 3 * face_size - col - 1, row: 3 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Two, Facing::Left) => {
                    assert_eq!(col, 0);
                    (
                        Point { col: 5 * face_size - 1 - row, row: 3 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Two, Facing::Right) => unreachable!(),
                (OneOfSix::Three, Facing::Up) => {
                    assert_eq!(row, face_size);
                    (Point { col: face_size * 2, row: col - face_size }, Facing::Right)
                }
                (OneOfSix::Three, Facing::Down) => {
                    assert_eq!(row, 2 * face_size - 1);
                    (
                        Point { col: 2 * face_size, row: 4 * face_size - col - 1 },
                        Facing::Right,
                    )
                }
                (OneOfSix::Three, Facing::Left)
                | (OneOfSix::Three, Facing::Right)
                | (OneOfSix::Four, Facing::Up)
                | (OneOfSix::Four, Facing::Down)
                | (OneOfSix::Four, Facing::Left) => {
                    unreachable!()
                }
                (OneOfSix::Four, Facing::Right) => {
                    assert_eq!(col, 3 * face_size - 1);
                    (Point { col: 5 * face_size - row - 1, row: 2 * face_size }, Facing::Down)
                }
                (OneOfSix::Five, Facing::Up) => unreachable!(),
                (OneOfSix::Five, Facing::Down) => {
                    assert_eq!(row, face_size * 3 - 1);
                    (
                        Point { col: 3 * face_size - col - 1, row: 2 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Five, Facing::Left) => {
                    assert_eq!(col, 2 * face_size);
                    (
                        Point { col: 4 * face_size - row - 1, row: 2 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Five, Facing::Right) => unreachable!(),
                (OneOfSix::Six, Facing::Up) => {
                    assert_eq!(row, 2 * face_size);
                    (
                        Point { col: 3 * face_size - 1, row: 5 * face_size - col - 1 },
                        Facing::Left,
                    )
                }
                (OneOfSix::Six, Facing::Down) => {
                    assert_eq!(row, 3 * face_size - 1);
                    (Point { col: 0, row: 5 * face_size - col - 1 }, Facing::Right)
                }
                (OneOfSix::Six, Facing::Left) => unreachable!(),
                (OneOfSix::Six, Facing::Right) => {
                    assert_eq!(col, 4 * face_size - 1);
                    (
                        Point { col: 3 * face_size - 1, row: 3 * face_size - row - 1 },
                        Facing::Left,
                    )
                }
            }
        }
    }

    fn cube_face(&self, pt: Point) -> OneOfSix {
        let map_width = self.bottom_right.col - self.top_left.col + 1;
        let map_height = self.bottom_right.row - self.top_left.row + 1;
        assert_eq!(map_width % 4, 0);
        assert_eq!(map_height % 3, 0);
        let hrange = (0..4)
            .into_iter()
            .map(|idx| (self.top_left.col + idx * map_width / 4..self.top_left.col + (idx + 1) * map_width / 4))
            .collect::<Vec<_>>();
        let vrange = (0..3)
            .into_iter()
            .map(|idx| (self.top_left.row + idx * map_height / 3..self.top_left.row + (idx + 1) * map_height / 3))
            .collect::<Vec<_>>();
        if hrange[0].contains(&pt.col) && vrange[1].contains(&pt.row) {
            OneOfSix::Two
        } else if hrange[2].contains(&pt.col) && vrange[0].contains(&pt.row) {
            OneOfSix::One
        } else if hrange[1].contains(&pt.col) && vrange[1].contains(&pt.row) {
            OneOfSix::Three
        } else if hrange[2].contains(&pt.col) && vrange[1].contains(&pt.row) {
            OneOfSix::Four
        } else if hrange[2].contains(&pt.col) && vrange[2].contains(&pt.row) {
            OneOfSix::Five
        } else if hrange[3].contains(&pt.col) && vrange[2].contains(&pt.row) {
            OneOfSix::Six
        } else {
            panic!("Not in the cube")
        }
    }

    fn do_motion(&self, motions: &Motions, is_cube: bool) -> Option<(Point, Facing)> {
        let mut location = self.start_location()?;
        let mut facing = Facing::Right;
        for instruction in motions.motions.iter() {
            match instruction {
                Motion::Right => facing = facing.turn_right(),
                Motion::Left => facing = facing.turn_left(),
                Motion::Forward(steps) => {
                    for _ in 0..*steps {
                        let (in_front, new_facing) = self.next_spot(location, facing, is_cube);
                        if self.points[&in_front] == Constraint::Free {
                            //println!("Moved to {in_front:?}");
                            location = in_front;
                            facing = new_facing;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Some((location, facing))
    }
}

#[derive(Debug)]
enum Motion {
    Right,
    Left,
    Forward(u32),
}
#[derive(Debug)]
struct Motions {
    motions: Vec<Motion>,
}
impl FromStr for Motions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::new();
        let chunks = s.split_inclusive(|c| "LR".contains(c));
        for chunk in chunks {
            match chunk.as_bytes()[chunk.len() - 1] {
                b'L' => {
                    result.push(Motion::Forward(chunk[..=chunk.len() - 2].parse::<u32>()?));
                    result.push(Motion::Left);
                }
                b'R' => {
                    result.push(Motion::Forward(chunk[..=chunk.len() - 2].parse::<u32>()?));
                    result.push(Motion::Right);
                }
                _ => {
                    result.push(Motion::Forward(chunk.parse::<u32>()?));
                }
            }
        }
        Ok(Motions { motions: result })
    }
}

struct Input {
    map: Map,
    motions: Motions,
}
impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_src, motions_src) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("badly formed input"))?;
        let map = map_src.parse::<Map>()?;
        let motions = motions_src.trim().parse::<Motions>()?;
        Ok(Input { map, motions })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}
impl Facing {
    fn turn_right(self) -> Self {
        match self {
            Facing::Up => Facing::Right,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Right => Facing::Down,
        }
    }
    fn turn_left(self) -> Self {
        match self {
            Facing::Up => Facing::Left,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Right => Facing::Up,
        }
    }
    fn score(self) -> i64 {
        match self {
            Facing::Up => 3,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Right => 0,
        }
    }
}

fn score(p: Point, f: Facing) -> i64 {
    let row = p.row + 1;
    let col = p.col + 1;
    let facing = f.score();
    1000 * row + 4 * col + facing
}

fn part1(input_str: &str) -> anyhow::Result<i64> {
    let Input { map, motions } = input_str.parse::<Input>()?;
    let (end_point, end_facing) = map
        .do_motion(&motions, false)
        .ok_or_else(|| anyhow::anyhow!("Empty map?"))?;
    Ok(score(end_point, end_facing))
}

fn part2(input_str: &str) -> anyhow::Result<i64> {
    let Input { map, motions } = input_str.parse::<Input>()?;
    let (end_point, end_facing) = map
        .do_motion(&motions, true)
        .ok_or_else(|| anyhow::anyhow!("Empty map?"))?;
    Ok(score(end_point, end_facing))
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
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 6032);
    }

    #[test_case("3R3R3" => (Point{row: 3, col: 11}, Facing::Left))]
    fn wraparound(motion: &str) -> (Point, Facing) {
        let Input { map, motions: _ } = SAMPLE.parse::<Input>().unwrap();
        let motions = motion.parse::<Motions>().unwrap();
        map.do_motion(&motions, false).unwrap()
    }

    #[test_case(Point{col:4,row:1}, Facing::Left => (Point{col:7,row:1}, Facing::Left))]
    #[test_case(Point{col:4,row:3}, Facing::Down => (Point{col:4,row:0}, Facing::Down))]
    #[test_case(Point{col:7,row:2}, Facing::Right => (Point{col:4,row:2}, Facing::Right))]
    #[test_case(Point{col:3,row:4}, Facing::Up => (Point{col:3,row:7}, Facing::Up))]
    fn next_spot(location: Point, facing: Facing) -> (Point, Facing) {
        let map = indoc::indoc! {"
                ....
                ....
                ....
                ....
            ....
            ....
            ....
            ....
        "}
        .parse::<Map>()
        .unwrap();
        map.next_spot(location, facing, false)
    }

    #[test_case(Point{col: 8, row: 0}, Facing::Up => (Point{col: 3, row: 4}, Facing::Down); "up from face 1 (on left)")]
    #[test_case(Point{col: 11, row: 0}, Facing::Up => (Point{col: 0, row: 4}, Facing::Down); "up from face 1 (on right)")]
    #[test_case(Point{col: 8, row: 0}, Facing::Left => (Point{col: 4, row: 4}, Facing::Down); "left from face 1 (on top)")]
    #[test_case(Point{col: 8, row: 3}, Facing::Left => (Point{col: 7, row: 4}, Facing::Down); "left from face 1 (on bottom)")]
    #[test_case(Point{col: 11, row: 0}, Facing::Right => (Point{col: 15, row: 11}, Facing::Left); "right from face 1 (on top)")]
    #[test_case(Point{col: 11, row: 3}, Facing::Right => (Point{col: 15, row: 8}, Facing::Left); "right from face 1 (on bottom)")]
    #[test_case(Point{col: 0, row: 4}, Facing::Up => (Point{col: 11, row: 0}, Facing::Down); "up from face 2 (on left)")]
    #[test_case(Point{col: 3, row: 4}, Facing::Up => (Point{col: 8, row: 0}, Facing::Down); "up from face 2 (on right)")]
    #[test_case(Point{col: 0, row: 7}, Facing::Down => (Point{col: 11, row: 11}, Facing::Up); "down from face 2 (on left)")]
    #[test_case(Point{col: 3, row: 7}, Facing::Down => (Point{col: 8, row: 11}, Facing::Up); "down from face 2 (on right)")]
    #[test_case(Point{col: 0, row: 4}, Facing::Left => (Point{col: 15, row: 11}, Facing::Up); "left from face 2 (on top)")]
    #[test_case(Point{col: 0, row: 7}, Facing::Left => (Point{col: 12, row: 11}, Facing::Up); "left from face 2 (on bottom)")]
    #[test_case(Point{col: 4, row: 4}, Facing::Up => (Point{col: 8, row: 0}, Facing::Right); "up from face 3 (on the left)")]
    #[test_case(Point{col: 7, row: 4}, Facing::Up => (Point{col: 8, row: 3}, Facing::Right); "up from face 3 (on the right)")]
    #[test_case(Point{col: 4, row: 7}, Facing::Down => (Point{col: 8, row: 11}, Facing::Right); "down from face 3 (on the left)")]
    #[test_case(Point{col: 7, row: 7}, Facing::Down => (Point{col: 8, row: 8}, Facing::Right); "down from face 3 (on the right)")]
    #[test_case(Point{col: 11, row: 4}, Facing::Right => (Point{col: 15, row: 8}, Facing::Down); "right from face 4 (on the top)")]
    #[test_case(Point{col: 11, row: 7}, Facing::Right => (Point{col: 12, row: 8}, Facing::Down); "right from face 4 (on the bottom)")]
    fn next_spot_cube(location: Point, facing: Facing) -> (Point, Facing) {
        let Input { map, motions: _ } = SAMPLE.parse::<Input>().unwrap();

        map.next_spot(location, facing, true)
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 5031);
    }
}
