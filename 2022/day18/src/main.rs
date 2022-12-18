//! # Solution for Advent of Code 2022 Day 18: Boiling Boulders
//!
//! Ref: [Advent of Code 2022 Day 18](https://adventofcode.com/2022/day/18)
//!
use ahash::{AHashMap, AHashSet};
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
struct Point((i32, i32, i32));
impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, remaining) = s
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Bad Format for point {s}"))?;
        let x = first.parse::<i32>()?;
        let (second, third) = remaining
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Bad format for point: {s}"))?;
        let y = second.parse::<i32>()?;
        let z = third.parse::<i32>()?;
        Ok(Point((x, y, z)))
    }
}
struct Scan {
    voxels: AHashSet<Point>,
    cache: AHashMap<Point, bool>,
}
impl FromStr for Scan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Scan {
            voxels: s
                .lines()
                .map(|line| line.parse::<Point>())
                .collect::<anyhow::Result<AHashSet<_>>>()?,
            cache: AHashMap::new(),
        })
    }
}

impl Scan {
    fn neighbor_locations(pt: Point) -> impl Iterator<Item = Point> {
        [(0, 0, 1), (0, 0, -1), (1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0)]
            .into_iter()
            .map(move |(dx, dy, dz)| Point((pt.0 .0 + dx, pt.0 .1 + dy, pt.0 .2 + dz)))
    }

    fn free_neighbors(&self, pt: Point) -> impl Iterator<Item = Point> + '_ {
        Self::neighbor_locations(pt).filter(|p| !self.voxels.contains(p))
    }

    fn free_neighbor_count(&self, pt: Point) -> usize {
        self.free_neighbors(pt).count()
    }

    fn boundaries(&self) -> (Point, Point) {
        let (smallest_x, smallest_y, smallest_z, largest_x, largest_y, largest_z) = self.voxels.iter().fold(
            (i32::MAX, i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN),
            |acc, &p| {
                (
                    acc.0.min(p.0 .0),
                    acc.1.min(p.0 .1),
                    acc.2.min(p.0 .2),
                    acc.3.max(p.0 .0),
                    acc.4.max(p.0 .1),
                    acc.5.max(p.0 .2),
                )
            },
        );
        (
            Point((smallest_x - 1, smallest_y - 1, smallest_z - 1)),
            Point((largest_x + 1, largest_y + 1, largest_z + 1)),
        )
    }

    fn path_to_exterior_exists(
        &mut self,
        pt: Point,
        targets: &(Point, Point),
        previously_examined: &mut AHashSet<Point>,
    ) -> bool {
        if let Some(&result) = self.cache.get(&pt) {
            return result;
        }
        previously_examined.insert(pt);
        for to_check in [(0, 0, 1), (0, 0, -1), (1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0)]
            .into_iter()
            .map(|(dx, dy, dz)| Point((dx + pt.0 .0, dy + pt.0 .1, dz + pt.0 .2)))
        {
            if self.voxels.contains(&to_check) || previously_examined.contains(&to_check) {
                continue;
            }

            if to_check.0 .0 <= (targets.0).0 .0
                || to_check.0 .0 >= (targets.1).0 .0
                || to_check.0 .1 <= (targets.0).0 .1
                || to_check.0 .1 >= (targets.1).0 .1
                || to_check.0 .2 <= (targets.0).0 .2
                || to_check.0 .2 >= (targets.1).0 .2
                || self.path_to_exterior_exists(to_check, targets, previously_examined)
            {
                self.cache.insert(pt, true);
                return true;
            }
        }

        self.cache.insert(pt, false);
        false
    }

    fn exterior_count(&mut self, pt: Point) -> usize {
        // A point is an exterior point if at least one of its neighbors is empty and that neighbor can follow
        // a path to infinity without needing to cross through any other pixels. We want the number of faces
        // of our point where that's true. (So an isolated voxel has a count of 6.) Though we don't strictly
        // need to find the optimal path, we just need to show that a path exists.
        //
        // So, depth-first search then. (Can't to breadth-first, as we have an infinitely wide set of points.)
        // Our target is the limits of the scanned voxels, so we should prefer heading in the same direction
        // we were already travelling when iterating within the search.
        let target = self.boundaries();

        #[allow(clippy::needless_collect)] // it's not actually needless
        let neighbors = self.free_neighbors(pt).collect::<Vec<_>>();
        neighbors
            .into_iter()
            .filter(|&p| {
                let mut already_scanned = AHashSet::new();
                self.path_to_exterior_exists(p, &target, &mut already_scanned)
            })
            .count()
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let voxels = input.parse::<Scan>()?;
    let exposed_face_count = voxels.voxels.iter().map(|&vox| voxels.free_neighbor_count(vox)).sum();
    Ok(exposed_face_count)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut voxels = input.parse::<Scan>()?;
    #[allow(clippy::needless_collect)] // it's not actually needless
    let points = voxels.voxels.iter().copied().collect::<Vec<_>>();
    let free_count = points.into_iter().map(|vox| voxels.exterior_count(vox)).sum();

    Ok(free_count)
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
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 64);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 58);
    }

    #[test_case("1,1,1" => 6; "one pixel")]
    #[test_case("1,1,1\n2,1,1" => 5; "two neighboring pixels")]
    #[test_case("1,1,1\n2,2,2" => 6; "two disconnected pixels")]
    #[test_case(indoc::indoc!{"
        0,0,1
        1,0,1
        2,0,1
        0,1,1
        1,1,1
        2,1,1
        0,2,1
        2,2,1
        0,3,1
        2,3,1
        0,4,1
        2,4,1
        0,5,1
        2,5,1
        0,0,0
        1,0,0
        2,0,0
        0,1,0
        1,1,0
        2,1,0
        0,2,0
        1,2,0
        2,2,0
        0,3,0
        1,3,0
        2,3,0
        0,4,0
        1,4,0
        2,4,0
        0,5,0
        1,5,0
        2,5,0
        0,0,2
        1,0,2
        2,0,2
        0,1,2
        1,1,2
        2,1,2
        0,2,2
        1,2,2
        2,2,2
        0,3,2
        1,3,2
        2,3,2
        0,4,2
        1,4,2
        2,4,2
        0,5,2
        1,5,2
        2,5,2
    "} => 1; "pit")]
    #[test_case(indoc::indoc!{"
        0,0,1
        1,0,1
        2,0,1
        0,1,1
        1,1,1
        2,1,1
        0,2,1
        2,2,1
        0,3,1
        2,3,1
        0,4,1
        2,4,1
        0,5,1
        2,5,1
        0,0,0
        1,0,0
        2,0,0
        0,1,0
        1,1,0
        2,1,0
        0,2,0
        1,2,0
        2,2,0
        0,3,0
        1,3,0
        2,3,0
        0,4,0
        1,4,0
        2,4,0
        0,5,0
        1,5,0
        2,5,0
        0,0,2
        1,0,2
        2,0,2
        0,1,2
        1,1,2
        2,1,2
        0,2,2
        1,2,2
        2,2,2
        0,3,2
        1,3,2
        2,3,2
        0,4,2
        1,4,2
        2,4,2
        0,5,2
        1,5,2
        2,5,2
        1,5,1
    "} => 0; "closed pit")]
    #[test_case(indoc::indoc!{"
        0,0,1
        1,0,1
        2,0,1
        3,0,1
        4,0,1
        0,1,1
        1,1,1
        4,1,1
        0,2,1
        2,2,1
        4,2,1
        0,3,1
        4,3,1
        0,4,1
        2,4,1
        3,4,1
        4,4,1
        0,0,0
        1,0,0
        2,0,0
        3,0,0
        4,0,0
        0,1,0
        1,1,0
        2,1,0
        3,1,0
        4,1,0
        0,2,0
        1,2,0
        2,2,0
        3,2,0
        4,2,0
        0,3,0
        1,3,0
        2,3,0
        3,3,0
        4,3,0
        0,4,0
        1,4,0
        2,4,0
        3,4,0
        4,4,0
        0,0,2
        1,0,2
        2,0,2
        3,0,2
        4,0,2
        0,1,2
        1,1,2
        2,1,2
        3,1,2
        4,1,2
        0,2,2
        1,2,2
        2,2,2
        3,2,2
        4,2,2
        0,3,2
        1,3,2
        2,3,2
        3,3,2
        4,3,2
        0,4,2
        1,4,2
        2,4,2
        3,4,2
        4,4,2
    "} => 2; "same-tunnel reused")]
    fn exterior_count(input: &str) -> usize {
        let mut scan = input.parse::<Scan>().unwrap();
        scan.exterior_count(Point((1, 1, 1)))
    }
}
