//! # A* Searching
//!
//! A* is a search method that attempts to find a "lowest cost" path through a set of nodes in a graph, from a
//! given starting node to a destination. (The destination may be a specific node, as you would use if you
//! were plotting navigation on a map, or it may be a class of nodes, as though only some parts of the node
//! define a "destination match".)
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/A*_search_algorithm) will tell you a lot more about A*.
//!
//! The general idea: Create a data structure that defines each of your graph nodes, via the trait
//! [AStarNode], whatever shared state you need into that trait's [AStarNode::AssociatedState], and let it
//! rip.
#![warn(missing_docs)]

use ahash::AHashMap;
use num::Zero;
use priority_queue::PriorityQueue;
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;

/// The data that uniquely specifies a node in the search graph
///
/// This is the data structure that the search method uses most aggressively. Only store what uniquely
/// identifies a node here; it will be hashed.
pub trait AStarNode: Clone + PartialEq + Eq + Hash {
    /// The type of the measurement for transitions between states. In addition to its other requirements,
    /// this must implement [num::Zero], as we assume the cost of moving from a node back to itself is
    /// `T::Cost::zero()` (it's only used to set the path cost to the initial node).
    type Cost: Ord + Hash + Copy + Add<Output = Self::Cost> + Zero;
    /// The iterator type to visit all the immediate neighbors of a particular node.
    type NeighborIter: Iterator<Item = (Self, Self::Cost)>;
    /// An "associated data" type that can be used to store auxilliary data. The information here is not
    /// examined by the search itself, but rather passed opaquely to the helper functions. Essentially: this
    /// should hold the stuff needed to make decisions, but which doesn't uniquely identify a node and
    /// shouldn't be hashed.
    type AssociatedState;
    /// The A* "heuristic" function. This function is what drives the search towards its destination. It is an
    /// _optimistic_ gauge of the cost to the destination. More explicitly: it must return a value which is
    /// _less or equal_ to the minimum cost path to the destination. A typical choice (in a grid pathfind) is
    /// the linear distance (or [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)) to the
    /// destination. (Note that the units of this value must match the units of cost; if you use something
    /// like distance squared (which is much cheaper to calculate), the resulting path may not be optimal.)
    fn heuristic(&self, goal: &Self, state: &Self::AssociatedState) -> Self::Cost;
    /// Generates an iterator over all the neighbors of `self`, along with the costs to get to each of them.
    fn neighbors(&self, state: &Self::AssociatedState) -> Self::NeighborIter;
    /// Decides if a given node is a goal. In many uses of the search, this is just equality, but it may also
    /// deliberately avoid some of the node data to match a class of destinations. (Like: anything on the
    /// bottom row, or something.)
    fn goal_match(&self, goal: &Self, state: &Self::AssociatedState) -> bool;
}

// Highest priority is lowest cost, so this struct is an interface to flip the ordering. (Lowest cost sorts to
// "highest".)
#[derive(PartialEq, Eq)]
struct Priority<T: AStarNode> {
    value: T::Cost,
}
impl<T: AStarNode> PartialOrd for Priority<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value).map(|ordering| match ordering {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        })
    }
}
impl<T: AStarNode> Ord for Priority<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

/// Use a heuristic-based search from a start node to a destination class of nodes in a graph
///
/// If no path between start and the goal exists, `None` is returned.
///
/// Much more theoretical background available elsewhere, i.e.:
/// [Wikipedia](https://en.wikipedia.org/wiki/A*_search_algorithm).
///
/// # Example
///
/// ```
/// use astar::{search_astar, AStarNode};
/// # use anyhow::Error;
/// use std::str::FromStr;
/// #[derive(Clone, PartialEq, Eq, Hash)]
/// struct Node {
///     row: i64,
///     col: i64,
/// }
///
/// struct World {
///     width: i64,
///     height: i64,
///     walls: Vec<(i64, i64)>,
///     start: Node,
///     finish: Node,
/// }
/// impl FromStr for World {
///     // elided for brevity
/// #    type Err = Error;
/// #
/// #    fn from_str(s: &str) -> Result<Self, Self::Err> {
/// #        let mut height = 0;
/// #        let mut width = 0;
/// #        let mut start = Node { col: 0, row: 0 };
/// #        let mut finish = Node { col: 0, row: 0 };
/// #        let mut walls = vec![];
/// #        for (row, line) in s.lines().enumerate() {
/// #            let r = i64::try_from(row)?;
/// #            for (column, ch) in line.chars().enumerate() {
/// #                let c = i64::try_from(column)?;
/// #                match ch {
/// #                    'S' => {
/// #                        start = Node { row: r, col: c };
/// #                    }
/// #                    'G' => {
/// #                        finish = Node { row: r, col: c };
/// #                    }
/// #                    '#' => {
/// #                        walls.push((r, c));
/// #                    }
/// #                    _ => (),
/// #                }
/// #                width = width.max(c + 1);
/// #            }
/// #            height = height.max(r + 1);
/// #        }
/// #        Ok(World { width, height, walls, start, finish })
/// #    }
/// }
///
/// impl World {
///     fn path_visualization(&self, path: &Vec<Node>) -> Vec<String> {
///         // elided for brevity
/// #       let mut visualization = Vec::new();
/// #       for row in 0..self.height {
/// #           let mut line = String::new();
/// #           for col in 0..self.width {
/// #               let spot = Node { row, col };
/// #               if path.contains(&spot) {
/// #                   line.push('*');
/// #               } else if self.walls.contains(&(spot.row, spot.col)) {
/// #                   line.push('#');
/// #               } else {
/// #                   line.push('.');
/// #               }
/// #           }
/// #           visualization.push(line);
/// #       }
/// #       visualization
///     }
/// }
///
/// struct NIter {
///     info: Vec<(Node, i64)>,
///     next_index: usize,
/// }
///
/// impl Iterator for NIter {
///     type Item = (Node, i64);
///
///     fn next(&mut self) -> Option<Self::Item> {
///         if self.next_index >= self.info.len() {
///             None
///         } else {
///             self.next_index += 1;
///             Some(self.info[self.next_index - 1].clone())
///         }
///     }
/// }
///
/// impl AStarNode for Node {
///     type Cost = i64;
///     type AssociatedState = World;
///     type NeighborIter = NIter;
///
///     fn heuristic(&self, goal: &Self, _state: &Self::AssociatedState) -> Self::Cost {
///         (goal.row - self.row).abs() + (goal.col - self.col).abs()
///     }
///
///     fn goal_match(&self, goal: &Self, _state: &Self::AssociatedState) -> bool {
///         self.row == goal.row && self.col == goal.col
///     }
///
///     fn neighbors(&self, state: &Self::AssociatedState) -> Self::NeighborIter {
///         // Our neighbors are open squares (not walls) that are in-bounds.
///         NIter {
///             info: [(0, -1), (0, 1), (-1, 0), (1, 0)]
///                 .into_iter()
///                 .map(|(dy, dx)| (self.row + dy, self.col + dx))
///                 .filter(|&(row, col)| {
///                     row >= 0
///                         && col >= 0
///                         && row < state.height
///                         && col < state.width
///                         && !state.walls.contains(&(row, col))
///                 })
///                 .map(|(row, col)| (Node { row, col }, 1))
///                 .collect::<Vec<_>>(),
///             next_index: 0,
///         }
///     }
/// }
///
/// let map = &[
///     "S....#........................",
///     ".....#...............#........",
///     "###..#...............#........",
///     ".....................#........",
///     "########################......",
///     "..............................",
///     "..............................",
///     "..############################",
///     ".............................G",
///     "..............................",
/// ];
/// let world = map.join("\n").parse::<World>().unwrap();
///
/// let path = search_astar(world.start.clone(), world.finish.clone(), &world).unwrap();
/// let vis = world.path_visualization(&path);
/// let expected = &[
///     "**...#..............***.......",
///     ".***.#..............*#*.......",
///     "###*.#..............*#*.......",
///     "...******************#***.....",
///     "########################*.....",
///     "........................*.....",
///     ".************************.....",
///     ".*############################",
///     ".*****************************",
///     "..............................",
/// ];
///
/// assert_eq!(vis, expected);
/// ```
pub fn search_astar<T>(initial: T, goal: T, state: &T::AssociatedState) -> Option<Vec<T>>
where
    T: AStarNode,
{
    let mut open: PriorityQueue<T, Priority<T>> = PriorityQueue::new();
    // Like to find a way to combine these into the priority queue so we don't have to compute hashes so often
    let mut g_score = AHashMap::new();
    let mut f_score = AHashMap::new();
    let mut came_from: AHashMap<T, T> = AHashMap::new();

    g_score.insert(initial.clone(), T::Cost::zero());
    let fitness = initial.heuristic(&goal, state);
    f_score.insert(initial.clone(), fitness);

    open.push(initial, Priority { value: fitness });

    while !open.is_empty() {
        let (current, _) = open.pop().unwrap();
        if current.goal_match(&goal, state) {
            let mut result = vec![goal];
            let mut current = current;
            while let Some(previous) = came_from.get(&current) {
                result.push(previous.clone());
                current = previous.clone();
            }
            return Some(result.into_iter().rev().collect());
        }
        for (neighbor, neighbor_cost) in current.neighbors(state) {
            let tentative = g_score[&current] + neighbor_cost;
            if g_score.get(&neighbor).map_or(true, |&previous| tentative < previous) {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative);
                let new_fscore = tentative + neighbor.heuristic(&goal, state);
                f_score.insert(neighbor.clone(), new_fscore);
                open.push(neighbor, Priority { value: new_fscore });
            }
        }
    }
    None
}
