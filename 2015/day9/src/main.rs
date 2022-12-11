//! # Solution for Advent of Code 2015 Day 9: All in a Single Night
//!
//! Ref: [Advent of Code 2015 Day 9](https://adventofcode.com/2015/day/9)
//!
use ahash::{AHashMap, AHashSet};
use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug)]
struct DataPoint {
    location_a: String,
    location_b: String,
    distance: usize,
}

impl FromStr for DataPoint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<location_a>.*) to (?P<location_b>.*) = (?P<distance>0|[1-9][0-9]+)$")
                .expect("Hand-rolled regex is valid")
        });
        let caps = PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Bad distance specification: \"{s}\""))?;
        Ok(DataPoint {
            location_a: caps["location_a"].to_string(),
            location_b: caps["location_b"].to_string(),
            distance: caps["distance"]
                .parse::<usize>()
                .context("Error while parsing distance in \"{s}\"")?,
        })
    }
}

struct DPResult(anyhow::Result<DataPoint>);

#[derive(Default, Debug)]
struct Data {
    locations: AHashSet<String>,
    distances: AHashMap<(String, String), usize>,
}

impl FromIterator<DPResult> for anyhow::Result<Data> {
    fn from_iter<T: IntoIterator<Item = DPResult>>(iter: T) -> Self {
        // Collect up all the data
        let mut data = Data::default();
        for point in iter.into_iter() {
            let point = point.0?;
            data.distances
                .insert((point.location_a.clone(), point.location_b.clone()), point.distance);
            data.locations.insert(point.location_a);
            data.locations.insert(point.location_b);
        }
        // Do some validation
        // 1. All pairs must have a distance. (either (a,b) or (b,a))
        // 2. If both orders exist, they must have the same distance ((a,b) = (b,a))
        for pair in Combination::new(data.locations.iter().cloned().collect::<Vec<_>>().as_slice(), 2)
            .map(|v| (v[0].clone(), v[1].clone()))
        {
            let alternate = (pair.1.clone(), pair.0.clone());
            let exists = data.distances.contains_key(&pair);
            let alt_exists = data.distances.contains_key(&alternate);
            if !exists && !alt_exists {
                anyhow::bail!("No distance found between {} and {}", pair.0, pair.1);
            }
            if exists && alt_exists && data.distances.get(&pair) != data.distances.get(&alternate) {
                anyhow::bail!("Inconsistent distances between {} and {}", pair.0, pair.1);
            }
        }

        Ok(data)
    }
}

impl Data {
    fn distance(&self, start: String, finish: String) -> usize {
        match self.distances.get(&(start.clone(), finish.clone())) {
            Some(&val) => val,
            None => *self.distances.get(&(finish, start)).unwrap(),
        }
    }

    fn shortest_between(&self, start: &str, finish: &str) -> anyhow::Result<Vec<String>> {
        // Find the shortest path that visits all cities, starting at `start` and ending at `finish`.
        let start = self
            .locations
            .get(start)
            .ok_or_else(|| anyhow::anyhow!("No location named {start} in the dataset"))?;
        let finish = self
            .locations
            .get(finish)
            .ok_or_else(|| anyhow::anyhow!("No location named {finish} in the dataset"))?;
        // My original code was based on geometry and the triangle inequality. The input data, however,
        // clearly has wormholes & spacetime anomolies (i.e.: the triangle inequality does not hold). So the
        // first method got scrapped. Think of these less as distances, and more like energy requirements,
        // where things like catalytic reactions can take place, and where adding a step in the right spot can
        // make the whole thing cheaper.

        // The current method is just to try every permutation and see what comes out cheapest.
        let inner_locations = self
            .locations
            .iter()
            .filter(|&loc| loc != start && loc != finish)
            .collect::<Vec<_>>();
        Ok(Permutation::new(inner_locations.as_slice())
            .map(|potential| {
                let mut path = vec![start];
                path.extend(potential);
                path.push(finish);
                (
                    self.path_distance(&path.iter().map(|&s| s.clone()).collect::<Vec<_>>()),
                    path,
                )
            })
            .min_by(|&(a, _), &(c, _)| a.cmp(&c))
            .map(|x| x.1.iter().map(|&x| x.clone()).collect::<Vec<_>>())
            .unwrap())
    }

    fn longest_between(&self, start: &str, finish: &str) -> anyhow::Result<Vec<String>> {
        // Find the longest path that visits all cities, starting at `start` and ending at `finish`.
        let start = self
            .locations
            .get(start)
            .ok_or_else(|| anyhow::anyhow!("No location named {start} in the dataset"))?;
        let finish = self
            .locations
            .get(finish)
            .ok_or_else(|| anyhow::anyhow!("No location named {finish} in the dataset"))?;
        // My original code was based on geometry and the triangle inequality. The input data, however,
        // clearly has wormholes & spacetime anomolies (i.e.: the triangle inequality does not hold). So the
        // first method got scrapped. Think of these less as distances, and more like energy requirements,
        // where things like catalytic reactions can take place, and where adding a step in the right spot can
        // make the whole thing cheaper.

        // The current method is just to try every permutation and see what comes out cheapest.
        let inner_locations = self
            .locations
            .iter()
            .filter(|&loc| loc != start && loc != finish)
            .collect::<Vec<_>>();
        Ok(Permutation::new(inner_locations.as_slice())
            .map(|potential| {
                let mut path = vec![start];
                path.extend(potential);
                path.push(finish);
                (
                    self.path_distance(&path.iter().map(|&s| s.clone()).collect::<Vec<_>>()),
                    path,
                )
            })
            .max_by(|&(a, _), &(c, _)| a.cmp(&c))
            .map(|x| x.1.iter().map(|&x| x.clone()).collect::<Vec<_>>())
            .unwrap())
    }

    fn path_distance(&self, path: &[String]) -> usize {
        path.windows(2)
            .map(|v| (v[0].clone(), v[1].clone()))
            .map(|pair| self.distance(pair.0, pair.1))
            .sum()
    }

    fn shortest_path(&self) -> Option<(Vec<String>, usize)> {
        Combination::new(&self.locations.iter().collect::<Vec<_>>(), 2)
            .map(|endpoints| self.shortest_between(endpoints[0], endpoints[1]).unwrap())
            .map(|city_list| (self.path_distance(&city_list), city_list))
            .min_by(|&(dist_a, _), &(dist_b, _)| dist_a.cmp(&dist_b))
            .map(|x| (x.1, x.0))
    }

    fn longest_path(&self) -> Option<(Vec<String>, usize)> {
        Combination::new(&self.locations.iter().collect::<Vec<_>>(), 2)
            .map(|endpoints| self.longest_between(endpoints[0], endpoints[1]).unwrap())
            .map(|city_list| (self.path_distance(&city_list), city_list))
            .max_by(|&(dist_a, _), &(dist_b, _)| dist_a.cmp(&dist_b))
            .map(|x| (x.1, x.0))
    }
}

struct Combination<T> {
    source: Vec<T>,
    c: Vec<usize>,
    j: usize,
    t: usize,
    done: bool,
}

impl<T> Combination<T> {
    fn new(items: &[T], size: usize) -> Combination<T>
    where
        T: Clone,
    {
        let mut c = (0..size).collect::<Vec<_>>();
        c.push(items.len());
        c.push(0);
        Combination { source: items.to_vec(), c, j: size, t: size, done: false }
    }
}

impl<T> Iterator for Combination<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    // algorithm T from Knuth 7.2.1.3 "Generating all combinations"
    fn next(&mut self) -> Option<Self::Item> {
        // This structure uses a "child vector" that contains all the indexes into the source data. We do the
        // combinatorial work on that index vector, as copying an index is likely much cheaper than cloning
        // the actual items being permuted. Source items are only cloned when the return value is constructed.
        if self.done {
            None
        } else {
            let result = self.c[0..self.t]
                .iter()
                .map(|&idx| self.source[idx].clone())
                .collect::<Vec<_>>();

            let mut x;
            if self.j > 0 {
                x = self.j;
            } else {
                if self.c[0] + 1 < self.c[1] {
                    self.c[0] += 1;
                    return Some(result);
                }
                self.j = 2;
                loop {
                    self.c[self.j - 2] = self.j - 2;
                    x = self.c[self.j - 1] + 1;
                    if x != self.c[self.j] {
                        break;
                    }
                    self.j += 1;
                }
                if self.j > self.t {
                    self.done = true;
                    return Some(result);
                }
            }
            self.c[self.j - 1] = x;
            self.j -= 1;

            Some(result)
        }
    }
}

struct Permutation<T> {
    items: Vec<T>,
    a: Vec<usize>,
    n: usize,
    done: bool,
}

impl<T> Permutation<T>
where
    T: Clone,
{
    fn new(items: &[T]) -> Self {
        let n = items.len();
        Permutation { items: items.to_vec(), n, a: [0..=n].into_iter().flatten().collect::<Vec<_>>(), done: false }
    }
}

impl<T> Iterator for Permutation<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            // Algorithm L from Knuth 7.2.1.2. Generating all permutations.
            let result = Some(
                self.a[1..=self.n]
                    .iter()
                    .map(|&idx| self.items[idx - 1].clone())
                    .collect::<Vec<_>>(),
            );

            let mut j = self.n - 1;
            while j > 0 && self.a[j + 1] <= self.a[j] {
                j -= 1;
            }
            if j == 0 {
                self.done = true;
                return result;
            }
            let mut l = self.n;
            while self.a[j] >= self.a[l] {
                l -= 1;
            }
            (self.a[j], self.a[l]) = (self.a[l], self.a[j]);
            let mut k = j + 1;
            let mut l = self.n;
            while k < l {
                (self.a[k], self.a[l]) = (self.a[l], self.a[k]);
                k += 1;
                l -= 1;
            }
            return result;
        }
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let data = input
        .lines()
        .map(|line| DPResult(line.parse::<DataPoint>()))
        .collect::<Result<Data, anyhow::Error>>()?;

    let (short_path, distance) = data.shortest_path().unwrap();
    println!("{short_path:?}: {distance}");
    Ok(distance)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let data = input
        .lines()
        .map(|line| DPResult(line.parse::<DataPoint>()))
        .collect::<Result<Data, anyhow::Error>>()?;

    let (long_path, distance) = data.longest_path().unwrap();
    println!("{long_path:?}: {distance}");
    Ok(distance)
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

    static SAMPLE: &str = indoc::indoc! {"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 605);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 982);
    }

    #[test]

    fn comborator() {
        let input = vec!["Chicago", "Boston", "Miami", "Sacramento", "Phoenix"];
        let result = Combination::new(&input, 2).collect::<Vec<_>>();

        assert_eq!(result.len(), 10);
        assert!(result.contains(&["Chicago", "Boston"].to_vec()) || result.contains(&["Boston", "Chicago"].to_vec()));
        assert!(result.contains(&["Chicago", "Miami"].to_vec()) || result.contains(&["Miami", "Chicago"].to_vec()));
        assert!(
            result.contains(&["Chicago", "Sacramento"].to_vec())
                || result.contains(&["Sacramento", "Chicago"].to_vec())
        );
        assert!(result.contains(&["Chicago", "Phoenix"].to_vec()) || result.contains(&["Phoenix", "Chicago"].to_vec()));
        assert!(result.contains(&["Boston", "Miami"].to_vec()) || result.contains(&["Miami", "Boston"].to_vec()));
        assert!(
            result.contains(&["Boston", "Sacramento"].to_vec()) || result.contains(&["Sacramento", "Boston"].to_vec())
        );
        assert!(result.contains(&["Boston", "Phoenix"].to_vec()) || result.contains(&["Phoenix", "Boston"].to_vec()));
        assert!(
            result.contains(&["Miami", "Sacramento"].to_vec()) || result.contains(&["Sacramento", "Miami"].to_vec())
        );
        assert!(result.contains(&["Miami", "Phoenix"].to_vec()) || result.contains(&["Phoenix", "Miami"].to_vec()));
        assert!(
            result.contains(&["Sacramento", "Phoenix"].to_vec())
                || result.contains(&["Phoenix", "Sacramento"].to_vec())
        );
    }

    #[test]
    fn permutator() {
        let input = &[4, 12, 33, -1];
        let result = Permutation::new(input).collect::<Vec<_>>();
        assert_eq!(result.len(), 4 * 3 * 2);
        assert!(result.contains(&[4, 12, 33, -1].to_vec()));
        assert!(result.contains(&[4, 12, -1, 33].to_vec()));
        assert!(result.contains(&[4, 33, 12, -1].to_vec()));
        assert!(result.contains(&[4, -1, 12, 33].to_vec()));
        assert!(result.contains(&[4, 33, -1, 12].to_vec()));
        assert!(result.contains(&[4, -1, 33, 12].to_vec()));
        assert!(result.contains(&[12, 4, 33, -1].to_vec()));
        assert!(result.contains(&[12, 4, -1, 33].to_vec()));
        assert!(result.contains(&[33, 4, 12, -1].to_vec()));
        assert!(result.contains(&[-1, 4, 12, 33].to_vec()));
        assert!(result.contains(&[33, 4, -1, 12].to_vec()));
        assert!(result.contains(&[-1, 4, 33, 12].to_vec()));
        assert!(result.contains(&[12, 33, 4, -1].to_vec()));
        assert!(result.contains(&[12, -1, 4, 33].to_vec()));
        assert!(result.contains(&[33, 12, 4, -1].to_vec()));
        assert!(result.contains(&[-1, 12, 4, 33].to_vec()));
        assert!(result.contains(&[33, -1, 4, 12].to_vec()));
        assert!(result.contains(&[-1, 33, 4, 12].to_vec()));
        assert!(result.contains(&[12, 33, -1, 4].to_vec()));
        assert!(result.contains(&[12, -1, 33, 4].to_vec()));
        assert!(result.contains(&[33, 12, -1, 4].to_vec()));
        assert!(result.contains(&[-1, 12, 33, 4].to_vec()));
        assert!(result.contains(&[33, -1, 12, 4].to_vec()));
        assert!(result.contains(&[-1, 33, 12, 4].to_vec()));
    }
}
