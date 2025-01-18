//! # Solution for Advent of Code 2024 Day 16: Reindeer Maze
//!
//! Ref: [Advent of Code 2024 Day 16](https://adventofcode.com/2024/day/16)
//!
#![warn(missing_docs)]

use ahash::{AHashMap, AHashSet};
use anyhow::{anyhow, bail, Error, Result};
use astar::{search_astar, AStarNode};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::io::{self, Read};
use std::str::FromStr;

/// Represents the maze puzzle input
///
/// Contains:
/// * The maze layout where '#' represents walls
/// * Start position (S) coordinates
/// * End position (E) coordinates
///
/// Example maze format:
/// ```text
/// ###############
/// #.......#....E#
/// #.#.###.#.###.#
/// #.....#.#...#.#
/// #.###.#####.#.#
/// #S..#.........#
/// ###############
/// ```
#[derive(Clone)]
struct Input {
    /// Set of wall coordinates in the maze
    map: AHashSet<(i64, i64)>,
    /// Starting position coordinates (row, column)
    start: (i64, i64),
    /// Ending position coordinates (row, column)
    end: (i64, i64),
}

impl FromStr for Input {
    type Err = Error;

    /// Parses a maze string into an Input structure
    ///
    /// The input string should contain:
    /// * Exactly one 'S' character marking the start position
    /// * Exactly one 'E' character marking the end position
    /// * '#' characters for walls
    /// * '.' characters for open spaces
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The input contains invalid characters
    /// * No start position 'S' is found
    /// * No end position 'E' is found
    /// * Row or column numbers cannot be converted to i64
    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let mut end = None;
        let mut map = AHashSet::new();
        for (row, line) in s.lines().enumerate() {
            let row = i64::try_from(row)?;
            for (col, ch) in line.chars().enumerate() {
                let col = i64::try_from(col)?;
                match ch {
                    '#' => {
                        map.insert((row, col));
                    }
                    '.' => {}
                    'S' => {
                        start = Some((row, col));
                    }
                    'E' => {
                        end = Some((row, col));
                    }
                    _ => bail!("Bad Map Item"),
                }
            }
        }
        let start = start.ok_or_else(|| anyhow!("Missing Start"))?;
        let end = end.ok_or_else(|| anyhow!("Missing End"))?;
        Ok(Input { map, start, end })
    }
}

/// Represents the cardinal directions that can be faced
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
enum Facing {
    North,
    South,
    East,
    West,
}
impl Facing {
    /// Rotates the current facing 90 degrees clockwise
    fn clockwise(self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::South => Facing::West,
            Facing::East => Facing::South,
            Facing::West => Facing::North,
        }
    }

    /// Rotates the current facing 90 degrees counter-clockwise
    fn counter_clockwise(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
            Facing::West => Facing::South,
        }
    }

    /// Calculates the cost of turning from the current facing to a new facing
    ///
    /// Returns:
    /// * 0 for no turn (same direction)
    /// * 1000 for a 90-degree turn
    /// * 2000 for a 180-degree turn
    fn turn_cost(self, new_facing: Self) -> i64 {
        if self == new_facing {
            0
        } else if self.clockwise() == new_facing || self.counter_clockwise() == new_facing {
            1000
        } else {
            2000
        }
    }
}

/// Represents a position and direction in the maze
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Node {
    row: i64,
    col: i64,
    facing: Facing,
}

impl Node {
    /// Determines which direction would need to be faced to move to a new position
    ///
    /// # Panics
    ///
    /// Panics if the new position is not adjacent to the current position
    fn needed_facing(&self, new_spot: (i64, i64)) -> Facing {
        let (row, col) = new_spot;
        match (row - self.row, col - self.col) {
            (-1, 0) => Facing::North,
            (1, 0) => Facing::South,
            (0, -1) => Facing::West,
            (0, 1) => Facing::East,
            _ => panic!("invariants violated"),
        }
    }
}

impl AStarNode for Node {
    type Cost = i64;
    type AssociatedState = Input;
    type Goal = Node;

    /// Estimates the minimum cost to reach the goal from this node
    /// Returns the Manhattan distance between current position and goal
    fn heuristic(&self, goal: &Self, _state: &Self::AssociatedState) -> Self::Cost {
        (goal.row - self.row).abs() + (goal.col - self.col).abs()
    }

    /// Returns all valid neighboring positions and their costs
    ///
    /// Cost includes:
    /// * Base movement cost of 1
    /// * Turn cost based on direction change
    ///
    /// Filters out:
    /// * Positions containing walls
    /// * Moves that would require turning back (cost >= 1500)
    fn neighbors(&self, state: &Self::AssociatedState) -> impl Iterator<Item = (Self, Self::Cost)> {
        // Check all four adjacent positions: North, South, East, West
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| (self.row + dx, self.col + dy))
            .filter(|probe| !state.map.contains(probe))
            .map(|(row, col)| {
                let new_facing = self.needed_facing((row, col));
                let cost = 1 + self.facing.turn_cost(new_facing);
                (
                    Node {
                        row,
                        col,
                        facing: new_facing,
                    },
                    cost,
                )
            })
            .filter(|(_, cost)| {
                // Filter out the "turn back on yourself" moves
                *cost < 1500
            })
    }

    /// Determines if this node matches the goal position
    /// Only checks row and column coordinates, ignoring facing direction
    fn goal_match(&self, goal: &Self, _state: &Self::AssociatedState) -> bool {
        self.row == goal.row && self.col == goal.col
    }
}

/// Calculates the total cost of a path including movement and turning costs
fn path_cost(path: &[Node]) -> i64 {
    path.windows(2)
        .map(|items| {
            let prev = &items[0];
            let next = &items[1];
            let new_facing = prev.needed_facing((next.row, next.col));
            1 + prev.facing.turn_cost(new_facing)
        })
        .sum()
}

/// Solves part 1 of the puzzle by finding the shortest path from start to end
fn part1(input: &Input) -> i64 {
    let start = Node {
        row: input.start.0,
        col: input.start.1,
        facing: Facing::East,
    };
    let goal = Node {
        row: input.end.0,
        col: input.end.1,
        facing: Facing::East,
    };
    let path = search_astar(start, &goal, input).unwrap();
    path_cost(&path)
}

/// Results from running Dijkstra's algorithm on the maze
struct DijkstraResult {
    /// Maps each node to its shortest distance from the start
    distances: AHashMap<Node, i64>,
    /// Maps each node to its possible parent nodes in shortest paths
    parents: AHashMap<Node, Vec<Node>>,
}

impl DijkstraResult {
    /// Runs Dijkstra's algorithm to find all shortest paths through the maze
    fn dijkstra(world: &Input) -> DijkstraResult {
        // Initialize data structures for Dijkstra's algorithm
        let mut distances = AHashMap::<_, _>::new();
        let mut heap = BinaryHeap::new();
        let mut parents = AHashMap::<_, _>::new();

        // The distance to the start node is zero. Any node not in the distances map has infinite distance.
        let start = Node {
            row: world.start.0,
            col: world.start.1,
            facing: Facing::East,
        };
        distances.insert(start, 0);
        heap.push(Reverse((0, start)));

        while let Some(Reverse((distance, node))) = heap.pop() {
            // Skip if we already found a better path to this node
            let previously_known_distance = *distances.get(&node).unwrap_or(&i64::MAX);
            if distance > previously_known_distance {
                continue;
            }

            // Process each neighbor, updating distances and parents for shorter paths
            for (neighbor, cost) in node.neighbors(world) {
                let new_target_distance = distance + cost;
                let previous_target_distance = distances.get(&neighbor).copied().unwrap_or(i64::MAX);
                match new_target_distance.cmp(&previous_target_distance) {
                    // Found a shorter path to neighbor
                    Ordering::Less => {
                        distances.insert(neighbor, new_target_distance);
                        parents.insert(neighbor, vec![node]);
                        heap.push(Reverse((new_target_distance, neighbor)));
                    }
                    // Found an equal-length alternative path
                    Ordering::Equal => {
                        parents
                            .get_mut(&neighbor)
                            .expect("parent vec should be there")
                            .push(node);
                    }
                    // Found a longer path, ignore it
                    Ordering::Greater => {}
                }
            }
        }

        DijkstraResult { distances, parents }
    }

    /// Reconstructs all possible shortest paths from source to target
    fn reconstruct_paths(&self, source: Node, target: Node) -> Vec<Vec<Node>> {
        let mut paths = Vec::new();
        let mut current_path = Vec::new();
        self.dfs_reconstruct(source, target, &mut current_path, &mut paths);
        paths
    }

    /// Helper function for path reconstruction using depth-first search
    fn dfs_reconstruct(&self, source: Node, current: Node, current_path: &mut Vec<Node>, paths: &mut Vec<Vec<Node>>) {
        current_path.push(current);

        if current == source {
            let mut path = current_path.clone();
            path.reverse();
            paths.push(path);
        } else if let Some(parents) = self.parents.get(&current) {
            for &parent in parents {
                self.dfs_reconstruct(source, parent, current_path, paths);
            }
        }

        current_path.pop();
    }
}

/// Solves part 2 of the puzzle by finding all possible shortest paths and counting unique positions
fn part2(world: &Input) -> Result<usize> {
    let dj_res = DijkstraResult::dijkstra(world);

    // We'll have up to four "goals" in that result (one for each facing), so pick the ones with the smallest distance.
    let best_distance = [Facing::West, Facing::East, Facing::North, Facing::South]
        .iter()
        .filter_map(|f| {
            let goal = Node {
                row: world.end.0,
                col: world.end.1,
                facing: *f,
            };
            dj_res.distances.get(&goal).copied()
        })
        .min()
        .ok_or_else(|| anyhow!("No paths to target"))?;

    let targets = [Facing::West, Facing::East, Facing::North, Facing::South]
        .iter()
        .filter_map(|f| {
            let goal = Node {
                row: world.end.0,
                col: world.end.1,
                facing: *f,
            };
            if let Some(distance) = dj_res.distances.get(&goal).copied() {
                if distance == best_distance {
                    return Some(goal);
                }
            }
            None
        })
        .collect::<Vec<_>>();

    let source = Node {
        row: world.start.0,
        col: world.start.1,
        facing: Facing::East,
    };
    let paths = targets
        .iter()
        .flat_map(|tgt| dj_res.reconstruct_paths(source, *tgt))
        .collect::<Vec<_>>();

    let mut good_seats = AHashSet::new();
    for path in paths {
        for seat in path {
            good_seats.insert((seat.row, seat.col));
        }
    }

    Ok(good_seats.len())
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = part2(&input)?;
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

    static SAMPLE: &str = indoc::indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    static SAMPLE2: &str = indoc::indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test_case(SAMPLE => 7036; "first sample")]
    #[test_case(SAMPLE2 => 11048; "second sample")]
    fn part1_sample(inp: &str) -> i64 {
        part1(&inp.parse::<Input>().unwrap())
    }

    #[test_case(SAMPLE => 45; "first sample")]
    #[test_case(SAMPLE2 => 64; "second sample")]
    #[test_case(indoc::indoc!("
        ####
        #SE#
        ####
    ") => 2; "one move - two good seats")]
    #[test_case(indoc::indoc!("
        ###
        #E#
        #S#
        ###
    ") => 2; "one move with turn - two good seats")]
    #[test_case(indoc::indoc!("
        #####
        #S.E#
        #####
    ") => 3; "two moves - three good seats")]
    fn part2_sample(inp: &str) -> usize {
        part2(&inp.parse::<Input>().unwrap()).unwrap()
    }
}
