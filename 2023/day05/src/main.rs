//! # Solution for Advent of Code 2023 Day 5: If You Give A Seed A Fertilizer
//!
//! Ref: [Advent of Code 2023 Day 5](https://adventofcode.com/2023/day/5)
//!
use anyhow::{anyhow, bail, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::ops::Range;
use std::str::{FromStr, Lines};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct MapRange {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

impl FromStr for MapRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn grab(item: Option<Result<i64, Error>>) -> Result<i64, Error> {
            item.ok_or_else(|| anyhow!("Not enough elements for MapRange"))?
        }
        let mut items = s
            .split_whitespace()
            .map(|item| item.parse::<i64>().map_err(Error::from));
        let destination_start = grab(items.next())?;
        let source_start = grab(items.next())?;
        let range_length = grab(items.next())?;
        if items.next().is_some() {
            bail!("Too many items for MapRange");
        }
        Ok(MapRange { destination_start, source_start, range_length })
    }
}

#[derive(Debug, Default)]
struct PlantMap {
    map: Vec<MapRange>,
}

fn intersect<T>(left: &Range<T>, right: &Range<T>) -> Option<Range<T>>
where
    T: Ord + Copy,
{
    let start = left.start.max(right.start);
    let end = left.end.min(right.end);
    if start < end {
        Some(start..end)
    } else {
        None
    }
}

fn simplify<T>(mut items: Vec<Range<T>>) -> Vec<Range<T>>
where
    T: Ord + Copy,
{
    items.sort_by_key(|item| item.start);
    let mut output: Vec<Range<T>> = vec![];
    'outer: for item in items {
        for outs in output.iter_mut() {
            if item.start <= outs.end {
                if item.end > outs.end {
                    outs.end = item.end;
                }
                continue 'outer;
            }
        }
        output.push(item)
    }
    output
}

impl PlantMap {
    fn transition(&self, incoming: i64) -> i64 {
        for rng in &self.map {
            if (rng.source_start..rng.source_start + rng.range_length).contains(&incoming) {
                return incoming - rng.source_start + rng.destination_start;
            }
        }
        incoming
    }

    fn one_range(&self, incoming: Range<i64>) -> Vec<Range<i64>> {
        let mut untransitioned = vec![incoming];
        let mut transitioned = Vec::<Range<i64>>::new();

        'new_work: while let Some(work_item) = untransitioned.pop() {
            for rng in self.map.iter() {
                if let Some(intersection) =
                    intersect(&work_item, &(rng.source_start..rng.source_start + rng.range_length))
                {
                    let delta = rng.destination_start - rng.source_start;
                    transitioned.push(intersection.start + delta..intersection.end + delta);
                    if work_item.start < intersection.start {
                        untransitioned.push(work_item.start..intersection.start);
                    }
                    if work_item.end > intersection.end {
                        untransitioned.push(intersection.end..work_item.end);
                    }
                    continue 'new_work;
                }
            }
            transitioned.push(work_item);
        }
        simplify(transitioned)
    }

    fn range_transition(&self, incoming: &[Range<i64>]) -> Vec<Range<i64>> {
        let mut outgoing = vec![];
        for item in incoming {
            outgoing.extend(self.one_range(item.clone()));
        }
        simplify(outgoing)
    }
}

#[derive(Debug)]
struct Almanac {
    seed_to_soil: PlantMap,
    soil_to_fertilizer: PlantMap,
    fertilizer_to_water: PlantMap,
    water_to_light: PlantMap,
    light_to_temperature: PlantMap,
    temperature_to_humidity: PlantMap,
    humidity_to_location: PlantMap,
}

impl Almanac {
    fn seed_to_location(&self, seed: i64) -> i64 {
        self.humidity_to_location.transition(
            self.temperature_to_humidity.transition(
                self.light_to_temperature.transition(
                    self.water_to_light.transition(
                        self.fertilizer_to_water
                            .transition(self.soil_to_fertilizer.transition(self.seed_to_soil.transition(seed))),
                    ),
                ),
            ),
        )
    }

    fn seed_range_to_location_range(&self, seed_range: Range<i64>) -> Vec<Range<i64>> {
        self.humidity_to_location.range_transition(
            self.temperature_to_humidity
                .range_transition(
                    self.light_to_temperature
                        .range_transition(
                            self.water_to_light
                                .range_transition(
                                    self.fertilizer_to_water
                                        .range_transition(
                                            self.soil_to_fertilizer
                                                .range_transition(self.seed_to_soil.one_range(seed_range).as_slice())
                                                .as_slice(),
                                        )
                                        .as_slice(),
                                )
                                .as_slice(),
                        )
                        .as_slice(),
                )
                .as_slice(),
        )
    }
}

#[derive(Debug)]
struct Input {
    initial_seeds: Vec<i64>,
    almanac: Almanac,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn grab(item: Option<&str>) -> Result<&str, Error> {
            item.ok_or_else(|| anyhow!("Not enough lines in input"))
        }
        fn blank_line(item: Option<&str>) -> Result<(), Error> {
            let line = item.ok_or_else(|| anyhow!("Not enough lines in input"))?;
            if line.is_empty() {
                Ok(())
            } else {
                Err(anyhow!("Line should be blank: {line}"))
            }
        }
        let mut lines = s.lines();
        let seed_line = grab(lines.next())?;
        static SEED_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^seeds:(?<seeds>(?: +[1-9][0-9]*)+)$").unwrap());
        let caps = SEED_PATTERN
            .captures(seed_line)
            .ok_or_else(|| anyhow!("Bad seed description: {seed_line}"))?;
        let initial_seeds = caps
            .name("seeds")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|item| item.parse::<i64>().map_err(Error::from))
            .collect::<Result<Vec<i64>, Error>>()?;

        blank_line(lines.next())?;

        fn grab_map(lines: &mut Lines, name: &str) -> Result<PlantMap> {
            let header = grab(lines.next())?;
            if header != format!("{name} map:") {
                bail!("Expected {name} map header: {header}");
            }
            let map = PlantMap {
                map: lines
                    .map_while(|line| line.parse::<MapRange>().ok())
                    .collect::<Vec<_>>(),
            };
            Ok(map)
        }

        let almanac = Almanac {
            seed_to_soil: grab_map(&mut lines, "seed-to-soil")?,
            soil_to_fertilizer: grab_map(&mut lines, "soil-to-fertilizer")?,
            fertilizer_to_water: grab_map(&mut lines, "fertilizer-to-water")?,
            water_to_light: grab_map(&mut lines, "water-to-light")?,
            light_to_temperature: grab_map(&mut lines, "light-to-temperature")?,
            temperature_to_humidity: grab_map(&mut lines, "temperature-to-humidity")?,
            humidity_to_location: grab_map(&mut lines, "humidity-to-location")?,
        };

        Ok(Input { initial_seeds, almanac })
    }
}

impl Input {
    fn seeds_as_ranges(&self) -> Vec<Range<i64>> {
        self.initial_seeds
            .as_slice()
            .chunks_exact(2)
            .map(|vals| vals[0]..vals[0] + vals[1])
            .collect::<Vec<_>>()
    }
}

fn part1(input: &str) -> Result<i64> {
    let my_input = input.parse::<Input>()?;

    Ok(my_input
        .initial_seeds
        .iter()
        .map(|seed| my_input.almanac.seed_to_location(*seed))
        .min()
        .unwrap())
}

fn part2(input: &str) -> Result<i64> {
    let my_input = input.parse::<Input>()?;

    let locations = my_input
        .seeds_as_ranges()
        .into_iter()
        .flat_map(|seeds| my_input.almanac.seed_range_to_location_range(seeds))
        .collect::<Vec<_>>();
    let locations_simplified = simplify(locations);
    Ok(locations_simplified[0].start)
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
#[allow(clippy::single_range_in_vec_init)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 35);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 46);
    }

    #[test_case(vec![] => Vec::<Range<i64>>::new(); "empty input")]
    #[test_case(vec![0..22, 56..102] => vec![0..22, 56..102]; "does nothing")]
    #[test_case(vec![0..10, 10..20, 20..30] => vec![0..30]; "collapse on edges")]
    #[test_case(vec![0..10, 5..15, 30..40] => vec![0..15, 30..40]; "merge")]
    #[test_case(vec![0..5, 10..15, 20..25, 2..21] => vec![0..25]; "overlaps")]
    fn simplify(incoming: Vec<Range<i64>>) -> Vec<Range<i64>> {
        super::simplify(incoming)
    }

    fn map_1() -> PlantMap {
        PlantMap {
            map: vec![
                MapRange { destination_start: 50, source_start: 10, range_length: 10 },
                MapRange { destination_start: 40, source_start: 20, range_length: 10 },
                MapRange { destination_start: 10, source_start: 40, range_length: 20 },
            ],
        }
    }

    #[test_case(
        PlantMap { map: vec![
                MapRange { destination_start: 99, source_start: 10, range_length: 30 },
                MapRange { destination_start: 10, source_start: 99, range_length: 30 },
            ] },
        &[0..10]
        => vec![0..10];
        "unaltered range"
    )]
    #[test_case(map_1(), &[0..1] => vec![0..1]; "map1: unaltered")]
    #[test_case(map_1(), &[10..11] => vec![50..51]; "map1: mapped location")]
    #[test_case(map_1(), &[9..11] => vec![9..10, 50..51]; "map1: two spots, one mapped")]
    #[test_case(map_1(), &[9..21] => vec![9..10, 40..41, 50..60]; "map1: three mappings")]
    fn range_transition(map: PlantMap, incoming: &[Range<i64>]) -> Vec<Range<i64>> {
        map.range_transition(incoming)
    }
}
