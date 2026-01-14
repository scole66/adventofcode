//! # Solution for Advent of Code 2025 Day 9: Movie Theater
//!
//! Ref: [Advent of Code 2025 Day 9](https://adventofcode.com/2025/day/9)
//!
use anyhow::{Error, Result, anyhow, bail};
use std::fmt::Display;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x_str, y_str) = s.split_once(',').ok_or(anyhow!("Bad pair: {s}"))?;
        let x = x_str.parse::<i64>()?;
        let y = y_str.parse::<i64>()?;
        Ok(Point { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Point {
    fn area_with(&self, other: &Self) -> i64 {
        (1 + (self.x - other.x).abs()) * (1 + (self.y - other.y).abs())
    }

    fn orientation(&self, q: &Self, r: &Self) -> i64 {
        ((q.x - self.x) * (r.y - self.y) - (q.y - self.y) * (r.x - self.x)).signum()
    }
}

struct Input {
    points: Vec<Point>,
}
impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let points = s.lines().map(str::parse::<Point>).collect::<Result<Vec<_>>>()?;
        if points.is_empty() {
            bail!("Invalid empty input");
        }
        Ok(Input { points })
    }
}

fn part1(input: &Input) -> i64 {
    input.points[0..input.points.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(idx, pt1)| input.points[idx + 1..].iter().map(|pt2| pt1.area_with(pt2)))
        .max()
        .unwrap_or(0)
}

fn segments_intersect(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    if a == c || a == d || b == c || b == d {
        return false;
    }
    let o1 = a.orientation(b, c);
    let o2 = a.orientation(b, d);
    let o3 = c.orientation(d, a);
    let o4 = c.orientation(d, b);

    o1 != o2 && o3 != o4
}

impl Input {
    pub fn entirely_within_parimeter(&self, pt1: usize, pt2: usize) -> bool {
        let r1 = Point {
            x: self.points[pt1].x,
            y: self.points[pt1].y,
        };
        let r2 = Point {
            x: self.points[pt1].x,
            y: self.points[pt2].y,
        };
        let r3 = Point {
            x: self.points[pt2].x,
            y: self.points[pt2].y,
        };
        let r4 = Point {
            x: self.points[pt2].x,
            y: self.points[pt1].y,
        };

        let box_edges = [(&r1, &r2), (&r2, &r3), (&r3, &r4), (&r4, &r1)];

        for perim_index in 0..self.points.len() {
            let next_index = (perim_index + 1) % self.points.len();
            let seg_a = &self.points[perim_index];
            let seg_b = &self.points[next_index];

            let intersection_spotted = box_edges
                .iter()
                .any(|box_seg| segments_intersect(seg_a, seg_b, box_seg.0, box_seg.1));
            if intersection_spotted {
                let (box_left, box_right) = if r1.x < r3.x { (r1.x, r3.x) } else { (r3.x, r1.x) };
                let (box_top, box_bottom) = if r1.y < r3.y { (r1.y, r3.y) } else { (r3.y, r1.y) };
                let (seg_left, seg_right) = if seg_a.x < seg_b.x {
                    (seg_a.x, seg_b.x)
                } else {
                    (seg_b.x, seg_a.x)
                };
                let (seg_upper, seg_lower) = if seg_a.y < seg_b.y {
                    (seg_a.y, seg_b.y)
                } else {
                    (seg_b.y, seg_a.y)
                };
                if !((box_left == seg_right)
                    || (box_right == seg_left)
                    || (box_top == seg_lower)
                    || (box_bottom == seg_upper))
                {
                    return false;
                }
            }
        }
        true
    }
}

fn part2(input: &Input) -> i64 {
    let mut biggest = 0;
    for pt1_idx in 0..input.points.len() - 1 {
        for pt2_idx in pt1_idx + 1..input.points.len() {
            let area = input.points[pt1_idx].area_with(&input.points[pt2_idx]);
            if area > biggest && input.entirely_within_parimeter(pt1_idx, pt2_idx) {
                biggest = area;
            }
        }
    }
    biggest
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

    static SAMPLE: &str = indoc::indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<Input>().unwrap()), 50);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<Input>().unwrap()), 24);
    }
}
