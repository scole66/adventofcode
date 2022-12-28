//! # Solution for Advent of Code 2022 Day 22: Monkey Map
//!
//! Ref: [Advent of Code 2022 Day 22](https://adventofcode.com/2022/day/22)
//!
use ahash::AHashMap;
use anyhow::{anyhow, bail, Error, Result};
use once_cell::sync::Lazy;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Clone, Copy)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FoldingStyle {
    Sample,
    Actual,
}
#[derive(Debug)]
struct Map {
    points: AHashMap<Point, Constraint>,
    face_size: i64,
    folding_style: FoldingStyle,
}
impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, ch)| match ch {
                    ' ' => None,
                    '.' => Some((|| {
                        Ok((Point { col: col.try_into()?, row: row.try_into()? }, Constraint::Free))
                    })()),
                    '#' => Some((|| {
                        Ok((Point { col: col.try_into()?, row: row.try_into()? }, Constraint::Wall))
                    })()),
                    _ => Some(Err(anyhow!("Bad character in map: {ch}"))),
                })
            })
            .collect::<Result<AHashMap<_, _>>>()?;

        // Calculate the bounding box (so we don't do it more often than we need to)
        let (leftmost, rightmost, top, bottom) = map.keys().fold(
            (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
            |(leftmost, rightmost, top, bottom), key| {
                (
                    leftmost.min(key.col),
                    rightmost.max(key.col),
                    top.min(key.row),
                    bottom.max(key.row),
                )
            },
        );

        // Calculate "face" size (for the cube problem). This is the shortest run of continuous characters in
        // either the rows or columns. All other run lengths should be multiples of this.
        let face_size = (top..=bottom)
            .map(|row| {
                map.keys()
                    .filter_map(|key| (key.row == row).then_some(key.col))
                    .fold((i64::MAX, i64::MIN), |(leftmost, rightmost), column| {
                        (leftmost.min(column), rightmost.max(column))
                    })
            })
            .map(|(smallest, largest)| 1 + largest - smallest)
            .min()
            .expect("The map should not be an empty string");

        // Validate that all rows have values between min & max and that their widths are multiples.
        for row in top..=bottom {
            let (first, last) = map
                .keys()
                .filter_map(|key| (key.row == row).then_some(key.col))
                .fold((i64::MAX, i64::MIN), |(smallest, largest), col| {
                    (smallest.min(col), largest.max(col))
                });
            for col in first..=last {
                if !map.contains_key(&Point { col, row }) {
                    bail!("Map has holes");
                }
            }
            if (last + 1 - first) % face_size != 0 {
                bail!(
                    "Map is ill-sized (face_size is {face_size}, row has width {})",
                    last - first + 1
                );
            }
        }
        for col in leftmost..=rightmost {
            let (first, last) = map
                .keys()
                .filter_map(|key| (key.col == col).then_some(key.row))
                .fold((i64::MAX, i64::MIN), |(smallest, largest), row| {
                    (smallest.min(row), largest.max(row))
                });
            for row in first..=last {
                if !map.contains_key(&Point { col, row }) {
                    bail!("Map has holes");
                }
            }
            if (last + 1 - first) % face_size != 0 {
                bail!(
                    "Map is ill-sized (face_size is {face_size}, column has height {})",
                    last - first + 1
                );
            }
        }

        // The resulting pattern should be foldable into a cube. There's probably a general purpose way to
        // confirm that's the case, but we only actually need to deal with the pattern from the sample, and
        // the pattern in my input, so I'm not gonna bother. We do need to figure out which of those patterns
        // it is, though.
        const SAMPLE_PROBES: [Point; 6] = [
            Point { row: 0, col: 2 },
            Point { row: 1, col: 0 },
            Point { row: 1, col: 1 },
            Point { row: 1, col: 2 },
            Point { row: 2, col: 2 },
            Point { row: 2, col: 3 },
        ];
        const ACTUAL_PROBES: [Point; 6] = [
            Point { row: 0, col: 1 },
            Point { row: 0, col: 2 },
            Point { row: 1, col: 1 },
            Point { row: 2, col: 0 },
            Point { row: 2, col: 1 },
            Point { row: 3, col: 0 },
        ];

        let folding_style = [
            (&SAMPLE_PROBES, FoldingStyle::Sample),
            (&ACTUAL_PROBES, FoldingStyle::Actual),
        ]
        .into_iter()
        .filter_map(|(probes, tag)| {
            probes
                .iter()
                .all(|pt| {
                    map.contains_key(&Point {
                        row: (face_size * pt.row) + face_size / 2,
                        col: (face_size * pt.col) + face_size / 2,
                    })
                })
                .then_some(tag)
        })
        .next()
        .ok_or_else(|| anyhow!("Map doesn't have a known fold"))?;

        Ok(Map { points: map, face_size, folding_style })
    }
}

impl Map {
    fn start_location(&self) -> Option<Point> {
        let lowest_row = 0;
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
                Facing::Up => |pt1: &&Point, pt2: &&Point| pt1.row.cmp(&pt2.row),
                Facing::Down => |pt1: &&Point, pt2: &&Point| pt2.row.cmp(&pt1.row),
                Facing::Left => |pt1: &&Point, pt2: &&Point| pt1.col.cmp(&pt2.col),
                Facing::Right => |pt1: &&Point, pt2: &&Point| pt2.col.cmp(&pt1.col),
            };
            let filter = match facing {
                Facing::Up | Facing::Down => |pt: &&Point, _: i64, col: i64| pt.col == col,
                Facing::Left | Facing::Right => |pt: &&Point, row: i64, _: i64| pt.row == row,
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
            let face_size = self.face_size;
            match (self.cube_face(from), facing, self.folding_style) {
                (OneOfSix::One, Facing::Up, FoldingStyle::Sample) => {
                    assert_eq!(row, 0);
                    (Point { col: 3 * face_size - col - 1, row: face_size }, Facing::Down)
                }
                (OneOfSix::One, Facing::Down, FoldingStyle::Sample) => unreachable!(),
                (OneOfSix::One, Facing::Left, FoldingStyle::Sample) => {
                    assert_eq!(col, face_size * 2);
                    (Point { col: face_size + row, row: face_size }, Facing::Down)
                }
                (OneOfSix::One, Facing::Right, FoldingStyle::Sample) => {
                    assert_eq!(col, face_size * 3 - 1);
                    (
                        Point { col: face_size * 4 - 1, row: 3 * face_size - row - 1 },
                        Facing::Left,
                    )
                }
                (OneOfSix::Two, Facing::Up, FoldingStyle::Sample) => {
                    assert_eq!(row, face_size);
                    (Point { col: 3 * face_size - col - 1, row: 0 }, Facing::Down)
                }
                (OneOfSix::Two, Facing::Down, FoldingStyle::Sample) => {
                    assert_eq!(row, 2 * face_size - 1);
                    (
                        Point { col: 3 * face_size - col - 1, row: 3 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Two, Facing::Left, FoldingStyle::Sample) => {
                    assert_eq!(col, 0);
                    (
                        Point { col: 5 * face_size - 1 - row, row: 3 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Two, Facing::Right, FoldingStyle::Sample) => unreachable!(),
                (OneOfSix::Three, Facing::Up, FoldingStyle::Sample) => {
                    assert_eq!(row, face_size);
                    (Point { col: face_size * 2, row: col - face_size }, Facing::Right)
                }
                (OneOfSix::Three, Facing::Down, FoldingStyle::Sample) => {
                    assert_eq!(row, 2 * face_size - 1);
                    (
                        Point { col: 2 * face_size, row: 4 * face_size - col - 1 },
                        Facing::Right,
                    )
                }
                (OneOfSix::Three, Facing::Left, FoldingStyle::Sample)
                | (OneOfSix::Three, Facing::Right, FoldingStyle::Sample)
                | (OneOfSix::Four, Facing::Up, FoldingStyle::Sample)
                | (OneOfSix::Four, Facing::Down, FoldingStyle::Sample)
                | (OneOfSix::Four, Facing::Left, FoldingStyle::Sample) => {
                    unreachable!()
                }
                (OneOfSix::Four, Facing::Right, FoldingStyle::Sample) => {
                    assert_eq!(col, 3 * face_size - 1);
                    (Point { col: 5 * face_size - row - 1, row: 2 * face_size }, Facing::Down)
                }
                (OneOfSix::Five, Facing::Up, FoldingStyle::Sample) => unreachable!(),
                (OneOfSix::Five, Facing::Down, FoldingStyle::Sample) => {
                    assert_eq!(row, face_size * 3 - 1);
                    (
                        Point { col: 3 * face_size - col - 1, row: 2 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Five, Facing::Left, FoldingStyle::Sample) => {
                    assert_eq!(col, 2 * face_size);
                    (
                        Point { col: 4 * face_size - row - 1, row: 2 * face_size - 1 },
                        Facing::Up,
                    )
                }
                (OneOfSix::Five, Facing::Right, FoldingStyle::Sample) => unreachable!(),
                (OneOfSix::Six, Facing::Up, FoldingStyle::Sample) => {
                    assert_eq!(row, 2 * face_size);
                    (
                        Point { col: 3 * face_size - 1, row: 5 * face_size - col - 1 },
                        Facing::Left,
                    )
                }
                (OneOfSix::Six, Facing::Down, FoldingStyle::Sample) => {
                    assert_eq!(row, 3 * face_size - 1);
                    (Point { col: 0, row: 5 * face_size - col - 1 }, Facing::Right)
                }
                (OneOfSix::Six, Facing::Left, FoldingStyle::Sample) => unreachable!(),
                (OneOfSix::Six, Facing::Right, FoldingStyle::Sample) => {
                    assert_eq!(col, 4 * face_size - 1);
                    (
                        Point { col: 3 * face_size - 1, row: 3 * face_size - row - 1 },
                        Facing::Left,
                    )
                }
                (OneOfSix::One, Facing::Up, FoldingStyle::Actual) => {
                    assert_eq!(row, 0);
                    (Point { col: 0, row: col + 2 * face_size }, Facing::Right)
                }
                (OneOfSix::One, Facing::Down, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::One, Facing::Left, FoldingStyle::Actual) => {
                    assert_eq!(col, face_size);
                    (Point { col: 0, row: 3 * face_size - row - 1 }, Facing::Right)
                }
                (OneOfSix::One, Facing::Right, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Two, Facing::Up, FoldingStyle::Actual) => {
                    assert_eq!(row, 0);
                    (Point { col: col - 2 * face_size, row: 4 * face_size - 1 }, Facing::Up)
                }
                (OneOfSix::Two, Facing::Down, FoldingStyle::Actual) => {
                    assert_eq!(row, face_size - 1);
                    (Point { col: 2 * face_size - 1, row: col - face_size }, Facing::Left)
                }
                (OneOfSix::Two, Facing::Left, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Two, Facing::Right, FoldingStyle::Actual) => {
                    let (lr, _lc) = (row, col - 2 * face_size);
                    let to_offset = Point { row: 2 * face_size, col: face_size }; // face 4
                    (
                        Point { col: to_offset.col + face_size - 1, row: (face_size - 1 - lr) + to_offset.row },
                        Facing::Left,
                    )
                }
                (OneOfSix::Three, Facing::Up, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Three, Facing::Down, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Three, Facing::Left, FoldingStyle::Actual) => {
                    (Point { col: row - face_size, row: 2 * face_size }, Facing::Down)
                }
                (OneOfSix::Three, Facing::Right, FoldingStyle::Actual) => (
                    Point { col: (row - face_size) + 2 * face_size, row: face_size - 1 },
                    Facing::Up,
                ),
                (OneOfSix::Four, Facing::Up, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Four, Facing::Down, FoldingStyle::Actual) => {
                    let (_lr, lc) = (row - 2 * face_size, col - face_size);
                    (Point { col: face_size - 1, row: lc + 3 * face_size }, Facing::Left)
                }
                (OneOfSix::Four, Facing::Left, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Four, Facing::Right, FoldingStyle::Actual) => {
                    let (lr, _lc) = (row - 2 * face_size, col - face_size);
                    let to_offset = Point { row: 0, col: 2 * face_size }; // face 2
                    (
                        Point { col: to_offset.col + face_size - 1, row: (face_size - 1 - lr) + to_offset.row },
                        Facing::Left,
                    )
                }
                (OneOfSix::Five, Facing::Up, FoldingStyle::Actual) => {
                    let (_lr, lc) = (row - 2 * face_size, col);
                    let (to_ofs_row, to_ofs_col) = (face_size, face_size); // face 3
                    (Point { col: to_ofs_col, row: lc + to_ofs_row }, Facing::Right)
                }
                (OneOfSix::Five, Facing::Down, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Five, Facing::Left, FoldingStyle::Actual) => {
                    let (lr, _lc) = (row - 2 * face_size, col);
                    let (to_ofs_row, to_ofs_col) = (0, face_size); // face 1
                    (
                        Point { row: (face_size - 1 - lr) + to_ofs_row, col: to_ofs_col },
                        Facing::Right,
                    )
                }
                (OneOfSix::Five, Facing::Right, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Six, Facing::Up, FoldingStyle::Actual) => unreachable!(),
                (OneOfSix::Six, Facing::Down, FoldingStyle::Actual) => {
                    let (_lr, lc) = (row - 3 * face_size, col);
                    let to_offset = Point { row: 0, col: 2 * face_size }; // face 2
                    (Point { row: to_offset.row, col: lc + to_offset.col }, Facing::Down)
                }
                (OneOfSix::Six, Facing::Left, FoldingStyle::Actual) => {
                    let (lr, _lc) = (row - 3 * face_size, col);
                    let to_offset = Point { row: 0, col: face_size }; // face 1
                    (Point { row: to_offset.row, col: lr + to_offset.col }, Facing::Down)
                }
                (OneOfSix::Six, Facing::Right, FoldingStyle::Actual) => {
                    let (lr, _lc) = (row - 3 * face_size, col);
                    let to_offset = Point { row: 2 * face_size, col: face_size }; // face 4
                    (
                        Point { row: to_offset.row + face_size - 1, col: lr + to_offset.col },
                        Facing::Up,
                    )
                }
            }
        }
    }

    fn cube_face(&self, pt: Point) -> OneOfSix {
        let normalized = Point { row: pt.row / self.face_size, col: pt.col / self.face_size };

        static FACE_DEFINITIONS: Lazy<AHashMap<(Point, FoldingStyle), OneOfSix>> = Lazy::new(|| {
            AHashMap::from_iter([
                ((Point { row: 0, col: 2 }, FoldingStyle::Sample), OneOfSix::One),
                ((Point { row: 1, col: 0 }, FoldingStyle::Sample), OneOfSix::Two),
                ((Point { row: 1, col: 1 }, FoldingStyle::Sample), OneOfSix::Three),
                ((Point { row: 1, col: 2 }, FoldingStyle::Sample), OneOfSix::Four),
                ((Point { row: 2, col: 2 }, FoldingStyle::Sample), OneOfSix::Five),
                ((Point { row: 2, col: 3 }, FoldingStyle::Sample), OneOfSix::Six),
                ((Point { row: 0, col: 1 }, FoldingStyle::Actual), OneOfSix::One),
                ((Point { row: 0, col: 2 }, FoldingStyle::Actual), OneOfSix::Two),
                ((Point { row: 1, col: 1 }, FoldingStyle::Actual), OneOfSix::Three),
                ((Point { row: 2, col: 0 }, FoldingStyle::Actual), OneOfSix::Five),
                ((Point { row: 2, col: 1 }, FoldingStyle::Actual), OneOfSix::Four),
                ((Point { row: 3, col: 0 }, FoldingStyle::Actual), OneOfSix::Six),
            ])
        });
        *FACE_DEFINITIONS
            .get(&(normalized, self.folding_style))
            .expect("Point should be in cube")
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
    type Err = Error;

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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_src, motions_src) = s.split_once("\n\n").ok_or_else(|| anyhow!("badly formed input"))?;
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

fn part1(input_str: &str) -> Result<i64> {
    let Input { map, motions } = input_str.parse::<Input>()?;
    let (end_point, end_facing) = map.do_motion(&motions, false).ok_or_else(|| anyhow!("Empty map?"))?;
    Ok(score(end_point, end_facing))
}

fn part2(input_str: &str) -> Result<i64> {
    let Input { map, motions } = input_str.parse::<Input>()?;
    let (end_point, end_facing) = map.do_motion(&motions, true).ok_or_else(|| anyhow!("Empty map?"))?;
    Ok(score(end_point, end_facing))
}

fn main() -> Result<()> {
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

    static OTHER_FOLD: &str = indoc::indoc! {"
            ........
            ........
            ........
            ........
            ....
            ....
            ....
            ....
        ........
        ........
        ........
        ........
        ....
        ....
        ....
        ....
    "};

    #[test_case(Point{col: 3, row: 15}, Facing::Right => (Point {col: 7, row: 11}, Facing::Up); "right from face 6")]
    #[test_case(Point{col: 3, row: 15}, Facing::Down => (Point {col: 11, row: 0}, Facing::Down); "down from face 6")]
    #[test_case(Point{col: 0, row: 15}, Facing::Left => (Point {col: 7, row: 0}, Facing::Down); "left from face 6")]
    #[test_case(Point{col: 0, row: 11}, Facing::Left => (Point {col: 4, row: 0}, Facing::Right); "left from face 5")]
    #[test_case(Point{col: 3, row: 8}, Facing::Up => (Point {col: 4, row: 7}, Facing::Right); "up from face 5")]
    #[test_case(Point{col: 7, row: 11}, Facing::Down => (Point {col: 3, row: 15}, Facing::Left); "down from face 4")]
    #[test_case(Point{col: 7, row: 11}, Facing::Right => (Point {col: 11, row: 0}, Facing::Left); "right from face 4")]
    #[test_case(Point{col: 7, row: 7}, Facing::Right => (Point {col: 11, row: 3}, Facing::Up); "right from face 3")]
    #[test_case(Point{col: 4, row: 7}, Facing::Left => (Point {col: 3, row: 8}, Facing::Down); "left from face 3")]
    #[test_case(Point{col: 11, row: 3}, Facing::Down => (Point {col: 7, row: 7}, Facing::Left); "down from face 2")]
    #[test_case(Point{col: 11, row: 3}, Facing::Right => (Point {col: 7, row: 8}, Facing::Left); "right from face 2")]
    #[test_case(Point{col: 7, row: 0}, Facing::Up => (Point {col: 0, row: 15}, Facing::Right); "up from face 1")]
    #[test_case(Point{col: 4, row: 3}, Facing::Left => (Point {col: 0, row: 8}, Facing::Right); "left from face 1")]
    #[test_case(Point{col: 11, row: 0}, Facing::Up => (Point {col: 3, row: 15}, Facing::Up); "up from face 2")]
    fn other_fold_next(location: Point, facing: Facing) -> (Point, Facing) {
        let map = OTHER_FOLD.parse::<Map>().unwrap();

        map.next_spot(location, facing, true)
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 5031);
    }
}
