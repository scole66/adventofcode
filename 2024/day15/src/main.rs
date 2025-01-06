//! # Solution for Advent of Code 2024 Day 15: Warehouse Woes
//!
//! Ref: [Advent of Code 2024 Day 15](https://adventofcode.com/2024/day/15)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, Error, Result};
use core::fmt;
use std::collections::VecDeque;
use std::hash::Hash;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(anyhow!("Bad direction")),
        }
    }
}

impl Direction {
    fn delta(self) -> (i64, i64) {
        match self {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum HorizontalDirection {
    Left,
    Right,
}

impl HorizontalDirection {
    fn delta(self) -> i64 {
        match self {
            HorizontalDirection::Left => -1,
            HorizontalDirection::Right => 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum VerticalDirection {
    Up,
    Down,
}

impl VerticalDirection {
    fn delta(self) -> i64 {
        match self {
            VerticalDirection::Up => -1,
            VerticalDirection::Down => 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Object {
    Wall,
    Box,
    Robot,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum WideObject {
    Wall,
    Robot,
    BoxLeft,
    BoxRight,
}

impl TryFrom<char> for Object {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '@' => Ok(Object::Robot),
            '#' => Ok(Object::Wall),
            'O' => Ok(Object::Box),
            _ => Err(anyhow!("Bad object")),
        }
    }
}

#[derive(Clone)]
struct Map {
    map: AHashMap<(i64, i64), Object>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let map = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, ch)| {
                    if ch == '.' {
                        None
                    } else {
                        Some(Object::try_from(ch).and_then(|obj| {
                            let row = i64::try_from(row)?;
                            let col = i64::try_from(col)?;
                            Ok(((row, col), obj))
                        }))
                    }
                })
            })
            .collect::<Result<AHashMap<_, _>, _>>()?;
        Ok(Map { map })
    }
}

impl Map {
    fn gps_sum(&self) -> i64 {
        self.map
            .iter()
            .filter_map(|(position, object)| {
                if *object == Object::Box {
                    let (row, col) = *position;
                    Some(row * 100 + col)
                } else {
                    None
                }
            })
            .sum()
    }

    fn find_robot(&self) -> Option<(i64, i64)> {
        self.map
            .iter()
            .find_map(
                |(position, object)| {
                    if *object == Object::Robot {
                        Some(position)
                    } else {
                        None
                    }
                },
            )
            .copied()
    }

    fn move_robot(&mut self, robot: (i64, i64), d: Direction) -> (i64, i64) {
        let delta = d.delta();
        let new_spot = (robot.0 + delta.0, robot.1 + delta.1);
        match self.map.get(&new_spot) {
            Some(Object::Wall) => robot,
            Some(Object::Robot) => {
                unreachable!()
            }
            Some(Object::Box) => {
                let mut stage = 1;
                loop {
                    stage += 1;
                    // keep adding the delta. If we get to an open spot, then the open spot gets a box, and the robot
                    // moves. If we get to a wall, then the robot doesn't move. If we get to another box, keep going.
                    let probe_spot = (robot.0 + stage * delta.0, robot.1 + stage * delta.1);
                    match self.map.get(&probe_spot) {
                        None => {
                            self.map.insert(probe_spot, Object::Box);
                            self.map.insert(new_spot, Object::Robot);
                            self.map.remove(&robot);
                            break new_spot;
                        }
                        Some(Object::Wall) => {
                            break robot;
                        }
                        Some(Object::Robot) => unreachable!(),
                        Some(Object::Box) => {}
                    }
                }
            }
            None => {
                self.map.remove(&robot);
                self.map.insert(new_spot, Object::Robot);
                new_spot
            }
        }
    }

    fn run_robot(&mut self, directions: &[Direction]) {
        let mut robot = self.find_robot().expect("there should be a robot");
        for &d in directions {
            robot = self.move_robot(robot, d);
        }
    }
}

struct UniqueQueue<T: Hash + Eq + Clone> {
    queue: VecDeque<T>,
    marked: AHashSet<T>,
}

impl<T: Hash + Eq + Clone> UniqueQueue<T> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            marked: AHashSet::new(),
        }
    }

    fn push_back(&mut self, item: T) {
        if !self.marked.contains(&item) {
            self.queue.push_back(item.clone());
            self.marked.insert(item);
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

struct WideMap {
    map: AHashMap<(i64, i64), WideObject>,
}

impl From<Map> for WideMap {
    fn from(value: Map) -> Self {
        Self {
            map: value
                .map
                .into_iter()
                .flat_map(|((row, col), obj)| {
                    let (left, right) = match obj {
                        Object::Wall => (
                            ((row, col * 2), WideObject::Wall),
                            Some(((row, col * 2 + 1), WideObject::Wall)),
                        ),
                        Object::Robot => (((row, col * 2), WideObject::Robot), None),
                        Object::Box => (
                            ((row, col * 2), WideObject::BoxLeft),
                            Some(((row, col * 2 + 1), WideObject::BoxRight)),
                        ),
                    };
                    [Some(left), right].into_iter().flatten()
                })
                .collect::<AHashMap<_, _>>(),
        }
    }
}

impl fmt::Display for WideMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_row, max_col) = self.map.iter().fold((i64::MIN, i64::MIN), |acc, ((row, col), _)| {
            (acc.0.max(*row), acc.1.max(*col))
        });
        for row in 0..=max_row {
            for col in 0..=max_col {
                let obj = self.map.get(&(row, col));
                write!(
                    f,
                    "{}",
                    match obj {
                        None => '.',
                        Some(WideObject::Robot) => '@',
                        Some(WideObject::Wall) => '#',
                        Some(WideObject::BoxLeft) => '[',
                        Some(WideObject::BoxRight) => ']',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl WideMap {
    fn gps_sum(&self) -> i64 {
        self.map
            .iter()
            .filter_map(|(position, object)| {
                if *object == WideObject::BoxLeft {
                    let (row, col) = *position;
                    Some(row * 100 + col)
                } else {
                    None
                }
            })
            .sum()
    }

    fn find_robot(&self) -> Option<(i64, i64)> {
        self.map
            .iter()
            .find_map(|(position, object)| {
                if *object == WideObject::Robot {
                    Some(position)
                } else {
                    None
                }
            })
            .copied()
    }

    fn horiz_move(&mut self, robot: (i64, i64), dir: HorizontalDirection) -> (i64, i64) {
        // we already know robot + step is a box.
        let delta = dir.delta();
        let far_edge_type = match dir {
            HorizontalDirection::Left => WideObject::BoxLeft,
            HorizontalDirection::Right => WideObject::BoxRight,
        };
        let mut step = 1;
        loop {
            step += 1;
            let probe = (robot.0, robot.1 + delta * step);
            match self.map.get(&probe) {
                Some(WideObject::Wall) => {
                    break robot;
                }
                Some(WideObject::BoxLeft | WideObject::BoxRight) => {}
                Some(WideObject::Robot) => unreachable!(),
                None => {
                    let mut backtrack_step = step;
                    let mut obj = far_edge_type;
                    while backtrack_step > 1 {
                        self.map.insert((robot.0, robot.1 + delta * backtrack_step), obj);
                        backtrack_step -= 1;
                        obj = match obj {
                            WideObject::BoxLeft => WideObject::BoxRight,
                            WideObject::BoxRight => WideObject::BoxLeft,
                            _ => unreachable!(),
                        };
                    }
                    self.map.insert((robot.0, robot.1 + delta), WideObject::Robot);
                    self.map.remove(&robot);
                    break (robot.0, robot.1 + delta);
                }
            }
        }
    }

    fn vert_move(&mut self, robot: (i64, i64), dir: VerticalDirection) -> (i64, i64) {
        assert!(self.map.get(&robot) == Some(&WideObject::Robot));
        let delta = dir.delta();
        let mut work_queue = UniqueQueue::new();

        // Starting from the robot, scan until we find a wall or that all of the boxes are clear
        let mut boxes_to_push = Vec::new();
        work_queue.push_back(robot);
        while let Some(spot_to_check) = work_queue.pop_front() {
            let probe = (spot_to_check.0 + delta, spot_to_check.1);
            match self.map.get(&probe) {
                None => {
                    boxes_to_push.push(spot_to_check);
                }
                Some(WideObject::Wall) => {
                    return robot;
                }
                Some(WideObject::Robot) => unreachable!(),
                Some(WideObject::BoxLeft) => {
                    boxes_to_push.push(spot_to_check);
                    work_queue.push_back(probe);
                    work_queue.push_back((probe.0, probe.1 + 1));
                }
                Some(WideObject::BoxRight) => {
                    boxes_to_push.push(spot_to_check);
                    work_queue.push_back((probe.0, probe.1 - 1));
                    work_queue.push_back(probe);
                }
            }
        }
        // Found only empty space, so we're good to shift all the boxes.
        for b in boxes_to_push.into_iter().rev() {
            let obj = self.map.remove(&b).expect("item should be present");
            self.map.insert((b.0 + delta, b.1), obj);
        }

        (robot.0 + delta, robot.1)
    }

    fn move_robot(&mut self, robot: (i64, i64), d: Direction) -> (i64, i64) {
        let delta = d.delta();
        let new_spot = (robot.0 + delta.0, robot.1 + delta.1);
        match self.map.get(&new_spot) {
            Some(WideObject::Wall) => robot,
            Some(WideObject::Robot) => unreachable!(),
            Some(WideObject::BoxRight) => {
                // The right part of a box. If direction is Left, then skip over boxes until we find a wall or an empty
                // space. If it's an empty space, then all the traversed boxes get shifted. (A new BoxLeft goes into the
                // empty space, and all the others switch Left/Right. The robot goes into the next spot over.)
                // If it's a wall, nothing moves.

                // If the direction is Right, we've entered unreachable code.

                // If the direction is Up or Down, then we get zones of influence spreading out. There can be multiple.
                match d {
                    Direction::Right => unreachable!(),
                    Direction::Left => self.horiz_move(robot, HorizontalDirection::Left),
                    Direction::Up => self.vert_move(robot, VerticalDirection::Up),
                    Direction::Down => self.vert_move(robot, VerticalDirection::Down),
                }
            }
            Some(WideObject::BoxLeft) => {
                // Similar to above
                match d {
                    Direction::Left => unreachable!(),
                    Direction::Right => self.horiz_move(robot, HorizontalDirection::Right),
                    Direction::Up => self.vert_move(robot, VerticalDirection::Up),
                    Direction::Down => self.vert_move(robot, VerticalDirection::Down),
                }
            }
            None => {
                self.map.remove(&robot);
                self.map.insert(new_spot, WideObject::Robot);
                new_spot
            }
        }
    }

    fn run_robot(&mut self, directions: &[Direction]) {
        let mut robot = self.find_robot().expect("there should be a robot");
        for &d in directions {
            robot = self.move_robot(robot, d);
        }
    }
}

struct Input {
    map: Map,
    instructions: Vec<Direction>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (map, directions) = s.split_once("\n\n").ok_or_else(|| anyhow!("Bad input"))?;
        let map = map.parse::<Map>()?;

        let instructions = directions
            .lines()
            .flat_map(|line| line.chars().map(Direction::try_from))
            .collect::<Result<Vec<_>>>()?;

        Ok(Input { map, instructions })
    }
}

fn part1(input: &Input) -> i64 {
    let mut after = input.map.clone();
    after.run_robot(&input.instructions);
    after.gps_sum()
}

fn part2(input: &Input) -> i64 {
    let mut after = WideMap::from(input.map.clone());
    after.run_robot(&input.instructions);
    after.gps_sum()
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = part2(&input);
    let elapsed = start_time.elapsed();

    println!("Part1: {part1}");
    println!("Part2: {part2}");
    println!("Time: {elapsed:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE_SMALL: &str = indoc::indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    static SAMPLE: &str = indoc::indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test_case(SAMPLE => 10092; "big sample")]
    #[test_case(SAMPLE_SMALL => 2028; "small sample")]
    fn part1_sample(input: &str) -> i64 {
        part1(&input.parse::<Input>().unwrap())
    }

    static MOVE_UP: &str = indoc::indoc! {"
        ###
        #.#
        #O#
        #@#
        ###

        ^
    "};

    #[test_case(SAMPLE => 9021; "big sample")]
    #[test_case(MOVE_UP => 102; "just push a box up")]
    fn part2_sample(input: &str) -> i64 {
        part2(&input.parse::<Input>().unwrap())
    }
}
