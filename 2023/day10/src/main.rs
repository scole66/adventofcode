//! # Solution for Advent of Code 2023 Day 10: Pipe Maze
//!
//! Ref: [Advent of Code 2023 Day 10](https://adventofcode.com/2023/day/10)
//!
use ahash::AHashMap;
use anyhow::{anyhow, bail, Error, Result};
use std::fmt;
use std::io::{self, Read};
use std::ops::Not;
use std::str::FromStr;

#[derive(Debug, PartialEq, Hash, Copy, Clone)]
enum GridContent {
    Empty,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    StartingPosition,
}
impl fmt::Display for GridContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridContent::Empty => ' ',
                GridContent::NorthSouth => '┃',
                GridContent::EastWest => '━',
                GridContent::NorthEast => '┗',
                GridContent::NorthWest => '┛',
                GridContent::SouthEast => '┏',
                GridContent::SouthWest => '┓',
                GridContent::StartingPosition => 'S',
            }
        )
    }
}
impl TryFrom<char> for GridContent {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use GridContent::*;
        Ok(match value {
            '.' => Empty,
            '-' => EastWest,
            '|' => NorthSouth,
            'L' => NorthEast,
            '7' => SouthWest,
            'J' => NorthWest,
            'F' => SouthEast,
            'S' => StartingPosition,
            _ => bail!("Invalid character for grid"),
        })
    }
}

#[derive(Debug)]
struct Grid {
    cells: AHashMap<(i64, i64), GridContent>,
    width: i64,
    height: i64,
    start: (i64, i64),
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let location = (row, col);
                let content = *self.cells.get(&location).unwrap();
                write!(f, "{content}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| Ok::<_, Self::Err>(((row as i64, col as i64), GridContent::try_from(ch)?)))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, Self::Err>>()?
            .into_iter()
            .flatten()
            .collect::<AHashMap<_, _>>();
        let width = grid.keys().max_by_key(|(_, col)| *col).unwrap().1 + 1;
        let height = grid.keys().max_by_key(|(row, _)| *row).unwrap().0 + 1;
        let start = grid
            .iter()
            .find_map(|(key, val)| {
                if *val == GridContent::StartingPosition {
                    Some(*key)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("Missing starting location"))?;
        Ok(Self {
            cells: grid,
            width,
            height,
            start,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Outside,
    Inside,
}
#[derive(Debug, PartialEq, Eq)]
enum PathHistory {
    WasNorth,
    WasSouth,
}

impl Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::Outside => State::Inside,
            State::Inside => State::Outside,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn go(from: &(i64, i64), dir: Direction) -> (i64, i64) {
    match dir {
        Direction::North => (from.0 - 1, from.1),
        Direction::South => (from.0 + 1, from.1),
        Direction::East => (from.0, from.1 + 1),
        Direction::West => (from.0, from.1 - 1),
    }
}

impl Grid {
    fn connections(&self, location: &(i64, i64)) -> Option<(Direction, Direction)> {
        let pipe = *self.cells.get(location)?;
        use Direction::*;
        match pipe {
            GridContent::Empty => None,
            GridContent::NorthSouth => Some((North, South)),
            GridContent::EastWest => Some((East, West)),
            GridContent::NorthEast => Some((North, East)),
            GridContent::NorthWest => Some((North, West)),
            GridContent::SouthEast => Some((South, East)),
            GridContent::SouthWest => Some((South, West)),
            GridContent::StartingPosition => {
                let items = [(-1, 0, North), (1, 0, South), (0, -1, West), (0, 1, East)]
                    .into_iter()
                    .filter_map(|(drow, dcol, dir)| {
                        let probe_location = (location.0 + drow, location.1 + dcol);
                        if self.cells.get(&probe_location).is_some() {
                            if let Some((d1, d2)) = self.connections(&probe_location) {
                                if d1.opposite() == dir || d2.opposite() == dir {
                                    Some(dir)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if !items.is_empty() {
                    assert_eq!(items.len(), 2);
                    Some((items[0], items[1]))
                } else {
                    None
                }
            }
        }
    }
    fn next_location(&self, prior: Option<(i64, i64)>, current: Option<(i64, i64)>) -> Option<(i64, i64)> {
        match current {
            None => Some(self.start),
            Some(current) => {
                let connections = self.connections(&current).unwrap();
                match prior {
                    Some((prior_row, prior_col)) => {
                        let probe = go(&current, connections.0);
                        let next = if probe.0 == prior_row && probe.1 == prior_col {
                            go(&current, connections.1)
                        } else {
                            probe
                        };
                        if next == self.start {
                            None
                        } else {
                            Some(next)
                        }
                    }
                    None => Some(go(&current, connections.0)),
                }
            }
        }
    }
    fn path(&self) -> Vec<(i64, i64)> {
        let mut cursor = None;
        let mut path = vec![];
        let mut prior = None;
        while let Some(new_loc) = self.next_location(prior, cursor) {
            path.push(new_loc);
            prior = cursor;
            cursor = Some(new_loc);
        }
        path
    }

    fn inclusions(&self) -> usize {
        use PathHistory::*;
        use State::*;

        let path = self.path();
        let mut inclusions = 0;
        for row in 0..self.height {
            let mut state = Outside;
            let mut path_state = WasNorth;
            for col in 0..self.width {
                let loc = (row, col);
                if !path.contains(&loc) {
                    match state {
                        Outside => {}
                        Inside => {
                            inclusions += 1;
                        }
                    }
                } else {
                    let item = self.cells.get(&loc).unwrap();
                    match item {
                        GridContent::Empty => unreachable!(),
                        GridContent::NorthSouth => {
                            state = !state;
                        }
                        GridContent::EastWest => { /* no change */ }
                        GridContent::NorthEast => {
                            path_state = WasNorth;
                        }
                        GridContent::NorthWest => {
                            if path_state != WasNorth {
                                state = !state;
                            }
                        }
                        GridContent::SouthEast => {
                            path_state = WasSouth;
                        }
                        GridContent::SouthWest => {
                            if path_state == WasNorth {
                                state = !state;
                            }
                        }
                        GridContent::StartingPosition => {
                            let connections = self.connections(&loc).unwrap();
                            match connections {
                                (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
                                    state = !state;
                                }
                                (Direction::North, Direction::East) | (Direction::East, Direction::North) => {
                                    path_state = WasNorth;
                                }
                                (Direction::North, Direction::West) | (Direction::West, Direction::North) => {
                                    if path_state != WasNorth {
                                        state = !state;
                                    }
                                }
                                (Direction::South, Direction::East) | (Direction::East, Direction::South) => {
                                    path_state = WasSouth;
                                }
                                (Direction::South, Direction::West) | (Direction::West, Direction::South) => {
                                    if path_state == WasNorth {
                                        state = !state;
                                    }
                                }
                                (Direction::East, Direction::West) | (Direction::West, Direction::East) => {}
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
        }
        inclusions
    }
}

fn part1(input: &Grid) -> usize {
    input.path().len() / 2
}

fn part2(input: &Grid) -> usize {
    input.inclusions()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    let grid = Grid::from_str(&input)?;

    println!("Part1: {}", part1(&grid));
    println!("Part2: {}", part2(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn starting_directions() {
        let input = Grid::from_str(SAMPLE).unwrap();
        let dirs = input.connections(&input.start).unwrap();
        let dvec = [dirs.0, dirs.1];
        assert!(dvec.contains(&Direction::East));
        assert!(dvec.contains(&Direction::South));
    }

    #[test]
    fn get_path() {
        let input = Grid::from_str(SAMPLE).unwrap();
        let path = input.path();
        assert_eq!(
            path,
            vec![
                (2, 0),
                (3, 0),
                (4, 0),
                (4, 1),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (2, 4),
                (2, 3),
                (1, 3),
                (0, 3),
                (0, 2),
                (1, 2),
                (1, 1),
                (2, 1)
            ]
        );
    }

    #[test]
    fn part1_sample() {
        let input = Grid::from_str(SAMPLE).unwrap();
        println!("{input}");
        assert_eq!(part1(&input), 8);
    }

    static SAMPLE2: &str = indoc::indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    static SAMPLE3: &str = indoc::indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    static SAMPLE4: &str = indoc::indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test_case(SAMPLE2 => 4)]
    #[test_case(SAMPLE3 => 8)]
    #[test_case(SAMPLE4 => 10)]
    fn part2_sample(sample: &str) -> usize {
        let input = Grid::from_str(sample).unwrap();
        part2(&input)
    }
}
