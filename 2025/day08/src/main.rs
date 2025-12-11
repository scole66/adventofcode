//! # Solution for Advent of Code 2025 Day 8: Playground
//!
//! Ref: [Advent of Code 2025 Day 8](https://adventofcode.com/2025/day/8)
//!
use anyhow::{Error, Result, anyhow, bail};
use std::{
    cmp::min,
    io::{self, Read},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_squared(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x_str, rest) = s.split_once(',').ok_or(anyhow!("{s} is not a point"))?;
        let (y_str, z_str) = rest.split_once(',').ok_or(anyhow!("{rest} doesn't match 'y,z'"))?;
        let x = x_str.parse::<i64>()?;
        let y = y_str.parse::<i64>()?;
        let z = z_str.parse::<i64>()?;
        Ok(Point { x, y, z })
    }
}

#[derive(Debug, Clone)]
struct Input {
    points: Vec<Point>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let points = s.lines().map(str::parse::<Point>).collect::<Result<Vec<_>>>()?;
        if points.is_empty() {
            bail!("There were no points in the input");
        }
        Ok(Input { points })
    }
}

#[derive(Debug)]
struct Circuit {
    connected_boxes: Vec<Point>,
}

#[derive(Debug)]
struct State {
    circuits: Vec<Circuit>,
}

impl From<Input> for State {
    fn from(value: Input) -> Self {
        State {
            circuits: value
                .points
                .into_iter()
                .map(|p| Circuit {
                    connected_boxes: vec![p],
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl State {
    fn circuit_containing_point(&self, p: &Point) -> Option<usize> {
        self.circuits.iter().enumerate().find_map(|info| {
            if info.1.connected_boxes.contains(p) {
                Some(info.0)
            } else {
                None
            }
        })
    }

    fn calc_distances(&self) -> Vec<(i64, (Point, Point))> {
        let points = self
            .circuits
            .iter()
            .flat_map(|circuit| circuit.connected_boxes.iter())
            .collect::<Vec<_>>();
        let mut distances = (0..points.len() - 1)
            .map(|idx| (idx, points[idx]))
            .flat_map(|(idx, pt1)| {
                points[idx + 1..points.len()]
                    .iter()
                    .map(move |pt2| (pt1.distance_squared(pt2), (*pt1, **pt2)))
            })
            .collect::<Vec<_>>();
        distances.sort_unstable_by_key(|(dist, _)| *dist);
        distances
    }

    fn make_connections(&mut self, count: usize) -> Option<(Point, Point)> {
        let distances = self.calc_distances();
        let count = min(count, distances.len());
        let mut last_connection = None;
        for (_, (pt1, pt2)) in distances.iter().take(count) {
            let cir1 = self.circuit_containing_point(pt1);
            let cir2 = self.circuit_containing_point(pt2);
            if let (Some(cir1), Some(cir2)) = (cir1, cir2)
                && cir1 != cir2
            {
                let (low, high) = if cir1 < cir2 { (cir1, cir2) } else { (cir2, cir1) };

                // `split_at_mut(high)` gives:
                // - `left`: elements 0..high, so `left[low]` is at index `low`
                // - `right`: elements high.., so `right[0]` is at index `high`
                let (left, right) = self.circuits.split_at_mut(high);

                let circuit1 = &mut left[low];
                let circuit2 = &mut right[0];
                // Now: we'd rather copy as few items as possible, so let's reassign into "big circuit" and "little circuit"
                let (big_circuit, little_circuit) = 
                if circuit1.connected_boxes.len() < circuit2.connected_boxes.len() {
                    (circuit2, circuit1)
                } else {
                    (circuit1, circuit2)
                };

                let drained = little_circuit.connected_boxes.drain(..);
                big_circuit.connected_boxes.extend(drained);

                last_connection = Some((*pt1, *pt2));
            }
        }
        last_connection
    }
}

fn part1(input: Input) -> usize {
    let mut state = State::from(input);
    state.make_connections(1000);
    state
        .circuits
        .sort_unstable_by_key(|circuit| circuit.connected_boxes.len());
    state
        .circuits
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.connected_boxes.len())
        .product()
}

fn part2(input: Input) -> i64 {
    let mut state = State::from(input);
    let last_connection = state
        .make_connections(usize::MAX)
        .unwrap_or((Point::default(), Point::default()));
    last_connection.0.x * last_connection.1.x
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<Input>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(input.clone());
    let part2 = part2(input);
    let elapsed = start_time.elapsed();

    println!("Part1: {part1}");
    println!("Part2: {part2}");
    println!("Time: {elapsed:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn parse() {
        let inp = SAMPLE.parse::<Input>().unwrap();
        let test_value = Point { x: 805, y: 96, z: 715 };
        assert!(inp.points.contains(&test_value));
    }

    #[test]
    fn part1_sample() {
        let input = SAMPLE.parse::<Input>().unwrap();
        let mut state = State::from(input);
        state.make_connections(10);
        state
            .circuits
            .sort_unstable_by_key(|circuit| circuit.connected_boxes.len());
        let val: usize = state
            .circuits
            .iter()
            .rev()
            .take(3)
            .map(|circuit| circuit.connected_boxes.len())
            .product();
        assert_eq!(val, 40);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE.parse::<Input>().unwrap()), 25272);
    }
}
