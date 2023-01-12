//! # Solution for Advent of Code 2022 Day 12: Hill Climbing Algorithm
//!
//! Ref: [Advent of Code 2022 Day 12](https://adventofcode.com/2022/day/12)
//!
use ahash::{AHashMap, AHashSet};
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    start: (usize, usize),
    finish: (usize, usize),
    elevation_data: Vec<u8>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = vec![];
        let mut line_width = None;
        let mut map_height = 0;
        let mut start_location = None;
        for (line_number, line) in s.lines().enumerate() {
            match line_width {
                None => {
                    line_width = Some(line.len());
                }
                Some(prior) => {
                    if prior != line.len() {
                        anyhow::bail!("Map is not rectangular");
                    }
                }
            }
            map_height = line_number + 1;
            data.extend(
                line.chars()
                    .map(|ch| {
                        Ok(match ch {
                            'S' => 1,
                            'E' => 27,
                            e if e.is_ascii_lowercase() => e as u8 - b'a' + 1,
                            _ => {
                                anyhow::bail!("Improper character in map");
                            }
                        })
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?,
            );
            if start_location.is_none() {
                start_location = line.chars().position(|ch| ch == 'S').map(|idx| (idx, line_number));
            }
        }
        let width = line_width.ok_or_else(|| anyhow::anyhow!("Empty input"))?;
        assert_eq!(data.len(), width * map_height);
        let start = start_location.ok_or_else(|| anyhow::anyhow!("Unspecified start location"))?;
        let finish_index = data
            .iter()
            .position(|&x| x == 27)
            .ok_or_else(|| anyhow::anyhow!("Unspecified endpoint"))?;
        let finish = (finish_index % width, finish_index / width);
        Ok(Map {
            width: line_width.ok_or_else(|| anyhow::anyhow!("Empty input"))?,
            height: map_height,
            start,
            finish,
            elevation_data: data,
        })
    }
}

impl Map {
    fn find_path(&self) -> Option<Vec<(usize, usize)>> {
        self.path_from(self.start)
    }
    fn path_from(&self, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        // Straightforward A* pathfind
        let mut open = AHashMap::new();
        let mut closed = AHashMap::new();
        struct Node {
            parent: (usize, usize),
            f: usize,
            g: usize,
        }
        open.insert(start, Node { parent: (usize::MAX, usize::MAX), f: 0, g: 0 });
        while !open.is_empty() {
            // Find the lowest f-value in the open list
            let current_pos = *open.iter().min_by(|a, b| a.1.f.cmp(&b.1.f)).unwrap().0;
            // Remove it from the open list
            let node = open.remove(&current_pos).unwrap();
            let current_g = node.g;
            // Add the removed node to the closed list
            closed.insert(current_pos, node);
            if current_pos == self.finish {
                // do some backtracking to return the path
                let mut result = vec![];
                let mut pos = current_pos;
                loop {
                    result.push(pos);
                    if pos == start {
                        return Some(result.into_iter().rev().collect::<Vec<_>>());
                    }
                    pos = closed.get(&pos).unwrap().parent;
                }
            }
            // Calculate child nodes
            let current_elevation = self.elevation_data[current_pos.0 + self.width * current_pos.1];
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if dx < 0 && current_pos.0 == 0
                    || dx > 0 && current_pos.0 == self.width - 1
                    || dy < 0 && current_pos.1 == 0
                    || dy > 0 && current_pos.1 == self.height - 1
                {
                    continue;
                }
                let child_pos = (
                    (current_pos.0 as isize + dx) as usize,
                    (current_pos.1 as isize + dy) as usize,
                );

                if closed.contains_key(&child_pos)
                    || self.elevation_data[child_pos.0 + self.width * child_pos.1] > current_elevation + 1
                {
                    continue;
                }
                let child_g = current_g + 1;
                let cdx = self.finish.0 as isize - child_pos.0 as isize;
                let cdy = self.finish.1 as isize - child_pos.1 as isize;
                let child_h = (cdx.abs() + cdy.abs()) as usize;
                let child_f = child_g + child_h;

                match open.get(&child_pos) {
                    None => {
                        open.insert(child_pos, Node { parent: current_pos, f: child_f, g: child_g });
                    }
                    Some(node) if node.g > child_g => {
                        open.insert(child_pos, Node { parent: current_pos, f: child_f, g: child_g });
                    }
                    Some(_) => {}
                }
            }
        }
        None
    }

    fn neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if point.0 > 0 {
            result.push((point.0 - 1, point.1));
        }
        if point.0 < self.width - 1 {
            result.push((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            result.push((point.0, point.1 - 1));
        }
        if point.1 < self.height - 1 {
            result.push((point.0, point.1 + 1));
        }
        result
    }

    fn dijkstra_with_early_exit(&self, start: (usize, usize)) -> Option<((usize, usize), usize)> {
        let mut dist = AHashMap::new();
        let mut prev: AHashMap<_, Option<(usize, usize)>> = AHashMap::new();
        let mut q = AHashSet::new();
        itertools::iproduct!(0..self.width, 0..self.height).for_each(|point| {
            dist.insert(point, usize::MAX);
            prev.insert(point, None);
            q.insert(point);
        });
        dist.insert(start, 0);

        while !q.is_empty() {
            let u = q
                .iter()
                .map(|&point| (point, dist[&point]))
                .min_by_key(|&info| info.1)
                .map(|info| info.0)
                .unwrap();
            q.remove(&u);

            let current_elevation = self.elevation_data[u.0 + self.width * u.1];
            let current_dist = dist[&u];

            for v in self.neighbors(u).into_iter().filter(|x| q.contains(x)) {
                let v_elevation = self.elevation_data[v.0 + self.width * v.1];
                if v_elevation + 1 < current_elevation {
                    continue;
                }
                if v_elevation == 1 {
                    return Some((v, current_dist + 1));
                }
                let alt = if current_dist == usize::MAX {
                    usize::MAX
                } else {
                    current_dist + 1
                };
                if alt < dist[&v] {
                    dist.insert(v, alt);
                    prev.insert(v, Some(u));
                }
            }
        }
        None
    }
}

fn part1(input: &str) -> anyhow::Result<Option<usize>> {
    let map = input.parse::<Map>()?;
    let path = map.find_path();
    Ok(path.map(|path| path.len() - 1))
}

fn part2(input: &str) -> anyhow::Result<Option<usize>> {
    let map = input.parse::<Map>()?;
    let short_path = map.dijkstra_with_early_exit(map.finish);
    Ok(Some(short_path.unwrap().1))
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?.unwrap());
    println!("Part2: {}", part2(&input)?.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap().unwrap(), 31);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap().unwrap(), 29);
    }

    #[test]
    fn fromstr() {
        let map = SAMPLE.parse::<Map>().unwrap();
        assert_eq!(map.height, 5);
        assert_eq!(map.width, 8);
        assert_eq!(map.start, (0, 0));
        assert_eq!(map.finish, (5, 2));
        assert_eq!(
            map.elevation_data,
            vec![
                1, 1, 2, 17, 16, 15, 14, 13, 1, 2, 3, 18, 25, 24, 24, 12, 1, 3, 3, 19, 26, 27, 24, 11, 1, 3, 3, 20, 21,
                22, 23, 10, 1, 2, 4, 5, 6, 7, 8, 9
            ]
        );
    }
}
