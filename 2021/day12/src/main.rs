//! # Solution for Advent of Code 2021 Day 12
//!
//! Ref: [Advent of Code 2021 Day 12](https://adventofcode.com/2021/day/12)
//!

use ahash::AHashMap;
use ahash::AHashSet;
use anyhow::{self, Context};
use regex::Regex;
use std::io::{self, BufRead};
use std::sync::LazyLock;

/// Marker for cavern size
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum NodeSize {
    Big,
    Small,
}

/// Cavern Identifier
///
/// Cavern identifiers are stored in this Identifier struct (which is a zero-size wrapper); mostly this was so the
/// `size` method is available.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Identifier(String);
impl Identifier {
    /// Figure out whether a cavern is big or small based on its name
    fn size(&self) -> NodeSize {
        // unwrap safe because ids are minimum 1 char long
        if self.0.chars().next().unwrap().is_ascii_uppercase() {
            NodeSize::Big
        } else {
            NodeSize::Small
        }
    }
}
impl From<String> for Identifier {
    fn from(src: String) -> Self {
        Identifier(src)
    }
}
impl From<&str> for Identifier {
    fn from(src: &str) -> Self {
        Identifier(src.to_string())
    }
}

/// Symbolic representation matching an input line
///
/// Transform each input string into one of these, then collect them together into a `Network` via the `Network`'s
/// `from_iter` implementation.
#[derive(Debug)]
struct PartialNode {
    id: Identifier,
    connection: Identifier,
}

/// A cavern in the network
///
/// Each Network is a collection of `Node`s. A `Node` has an identifier, and a set if connections leading out from it.
/// (It also has a size, produced by `id.size()`.)
#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    id: Identifier,
    connections: AHashSet<Identifier>,
}

/// The network of caverns
///
/// Create one of these things by generating an iterator over `PartialNodes` and then collecting into a
/// `anyhow::Result<Network>`. The collection needs to wind up in a `Result` because it might fail if the input fails to
/// specify start or end nodes.
///
/// # Example
/// ```
/// let from_input: Vec<PartialNode> = vec![PartialNode{id: "start".into(), connection: "end".into()}];
/// let network = from_input.iter().collect::<anyhow::Result<Network>>()?;
/// ```
#[derive(Debug)]
struct Network {
    nodes: AHashMap<Identifier, Node>,
}

impl FromIterator<PartialNode> for anyhow::Result<Network> {
    /// Produce a Cavern Complex from a collection of `PartialNode`s (i.e.: the input data)
    ///
    /// Each input line may make up to two new `Node`s in the network; both the source and destination indicated in a
    /// particular `PartialNode`. If the nodes already exist, then connection info is updated. Note that each link
    /// between nodes is _one-way_; so to model the actual cavern complex, connections are established on both the
    /// source and destination.
    ///
    /// If the resulting network lacks a "start" or "end" node, an error is returned.
    fn from_iter<I: IntoIterator<Item = PartialNode>>(iter: I) -> Self {
        let mut hm = AHashMap::<Identifier, Node>::new();
        for node in iter.into_iter() {
            // Setup the "source" node
            if let Some(node_ref) = hm.get_mut(&node.id) {
                // Already there, just update connections.
                node_ref.connections.insert(node.connection.clone());
            } else {
                // New cavern: set up its connection and add it to the hash map.
                let key = node.id.clone();
                let mut connections = AHashSet::<Identifier>::new();
                connections.insert(node.connection.clone());
                hm.insert(
                    key,
                    Node {
                        id: node.id.clone(),
                        connections,
                    },
                );
            }
            // Setup the "destination" node
            if let Some(node_ref) = hm.get_mut(&node.connection) {
                // Already there, just update connections
                node_ref.connections.insert(node.id);
            } else {
                // New cavern: set up its connection and add it to the hash map.
                let key = node.connection.clone();
                let mut connections = AHashSet::<Identifier>::new();
                connections.insert(node.id);
                hm.insert(
                    key,
                    Node {
                        id: node.connection,
                        connections,
                    },
                );
            }
        }
        let result = Network { nodes: hm };
        // Do a last-minute validation. (Do start and end nodes exist?)
        result.validate()?;
        Ok(result)
    }
}

impl Network {
    /// Last minute validation check
    ///
    /// Some things can only be checked after the network is built, after the input has been processed. (Like: do we
    /// have start and end nodes?) Those checks happen here.
    fn validate(&self) -> anyhow::Result<()> {
        if !self.nodes.contains_key(&"start".into()) {
            return Err(anyhow::anyhow!("start node missing from caverns"));
        }
        if !self.nodes.contains_key(&"end".into()) {
            return Err(anyhow::anyhow!("end node missing from caverns"));
        }
        Ok(())
    }

    #[doc(hidden)]
    /// The workhorse of the path calculator
    ///
    /// This is really an internal-only routine; see the docs for `figure_paths` for the real api.
    fn continue_path(
        &self,
        partial: Vec<Identifier>,
        small_limit: usize,
        collect: bool,
    ) -> (Vec<Vec<Identifier>>, usize) {
        // Given a partial path:
        // If path ends with "end", terminate, returning path
        // Spawn new partial paths, one for each unvisited small cavern (mostly), and one for each big cavern.
        // recurse.
        let prior = &partial[partial.len() - 1]; // The cavern we're coming _from_.
        if prior.0 == "end" {
            // === Done ===
            // return a list with just this path alone, and a "number of paths" value of 1.
            (if collect { vec![partial] } else { vec![] }, 1)
        } else {
            // === Make new moves forward! ===
            // Get the node corresponding to the last item on the path (it has our connections)
            let node = self.nodes.get(prior).unwrap(); // unwrap is safe because network is valid
                                                       // Return values. Currently an empty list and zero.
            let mut result_vec = vec![];
            let mut result_count = 0;
            // For all the potential connections...
            for next_node in node.connections.iter() {
                if next_node.0 == "start" {
                    // Can't go back to start, so this connection doesn't continue.
                } else if next_node.size() == NodeSize::Small {
                    // Small node: see how many times we've visited this node before
                    let previous_visits = partial.iter().filter(|&id| *id == *next_node).count();
                    if previous_visits < small_limit {
                        // Under the limit, so we can add at least one more visit there.
                        // Copy the path and add the new connection to the end.
                        let mut path = partial.clone();
                        path.push(next_node.clone());
                        // Reset the limit. The AOC Part 2 question was about visiting _only one_ small node twice. So
                        // if that's what we've just done, all future paths may only involve 1 visit to a small node.
                        let limit = if previous_visits > 0 { 1 } else { small_limit };
                        // We've updated our path and our limit, so now go do the recursive descent.
                        let upstream = self.continue_path(path, limit, collect);
                        // upstream will now contain all the paths that start with our "partial" path plus this
                        // connection node. Add them to the results.
                        if collect {
                            result_vec.extend(upstream.0);
                        }
                        result_count += upstream.1;
                    }
                } else {
                    // Big node: we can visit these as many times as possible.
                    // Add this to our path, and recurse.
                    let mut path = partial.clone();
                    path.push(next_node.clone());
                    let upstream = self.continue_path(path, small_limit, collect);
                    // Then add this to our results
                    if collect {
                        result_vec.extend(upstream.0);
                    }
                    result_count += upstream.1;
                }
            }
            (result_vec, result_count)
        }
    }

    /// Figure the possible paths through the caverns.
    ///
    /// * `small_limit` is used to specify how many times a small cavern may be visited. Use `1` for AOC 12's Part 1
    ///   question; use `2` for AOC 12's Part 2 question (but see Note below)
    /// * `collect` signals whether to collect the paths, or just count them.
    ///
    /// This returns a Pair: the first item is a list of paths that go from start to end; the second is the count of
    /// those paths. If `collect` is `false`, the first item will always be the empty list (but the second will still
    /// count how many items _would have been_ there.)
    ///
    /// Note: `small_limit` really only refers to the _first_ multiply visited cavern. And the implementation of that
    /// restriction broke `small_limit` for values greater than 2.
    fn figure_paths(&self, small_limit: usize, collect: bool) -> (Vec<Vec<Identifier>>, usize) {
        // This is the top of a recursive routine, so really all we do is set up the initial path and then let it go.
        let initial_path = vec![Identifier::from("start")];
        self.continue_path(initial_path, small_limit, collect)
    }
}

/// Parse one line of input into a PartialNode
///
/// Returns an `Err` if the input line fails validation (doesn't look like identifier - dash - identifier)
fn parse(s: String) -> anyhow::Result<PartialNode> {
    static NODE_PATTERN: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("^(?P<node_name>[a-z]+|[A-Z]+)-(?P<dest_name>[a-z]+|[A-Z]+)$").unwrap());

    let captures = NODE_PATTERN
        .captures(s.as_str())
        .ok_or_else(|| anyhow::anyhow!("{} is not a valid cavern description", s))?;
    let id = captures.name("node_name").unwrap().as_str().to_string(); // unwrap safe because match would have failed if name wasn't there
    let connection = captures.name("dest_name").unwrap().as_str().to_string(); // unwrap safe because match would have failed if name wasn't there
    Ok(PartialNode {
        id: id.into(),
        connection: connection.into(),
    })
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    // Finally learned how to abort an iter at the first error and return it. Now we get:
    //     $ cargo r -q < /dev/urandom
    //     Error: Failed to parse puzzle input from stdin
    //
    //     Caused by:
    //         stream did not contain valid UTF-8
    let network = stdin
        .lock()
        .lines()
        .map(|r| r.context("Failed to parse puzzle input from stdin").and_then(parse))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect::<anyhow::Result<Network>>()?;

    //let paths = network.figure_paths(2, true).0;
    //for path in paths {
    //    let parts: Vec<String> = path.iter().map(|i| i.0.clone()).collect();
    //    println!("{}", parts.join("-"));
    //}

    // part1: how many paths?
    let path_count = network.figure_paths(1, false).1;
    println!("Part 1: There are {path_count} paths through the caverns");

    // part2: how many paths with max 2 visits to small caverns?
    let path_count = network.figure_paths(2, false).1;
    println!("Part 2: There are {path_count} paths through the caverns");

    Ok(())
}
