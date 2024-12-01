//! # Solution for Advent of Code 2022 Day 16: Proboscidea Volcanium
//!
//! Ref: [Advent of Code 2022 Day 16](https://adventofcode.com/2022/day/16)
//!
use ahash::AHashMap;
use bimap::BiMap;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

struct ValveDescription {
    id: String,
    rate: i32,
    tunnels: Vec<String>,
}

impl FromStr for ValveDescription {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static VALVE_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^Valve (?P<id>[A-Z]{2}) has flow rate=(?P<rate>0|[1-9][0-9]*); tunnels? leads? to valves? (?P<tunnels>[A-Z]{2}(?:, [A-Z]{2})*)$").unwrap()
        });
        let caps = VALVE_PATTERN
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Bad match for string {s}"))?;
        Ok(ValveDescription {
            id: caps["id"].to_string(),
            rate: caps["rate"].parse()?,
            tunnels: caps["tunnels"].split(", ").map(String::from).collect(),
        })
    }
}

struct InputData {
    ids: BiMap<u32, String>,
    rates: Vec<i32>,
    tunnels: Vec<Vec<u32>>,
}
impl FromStr for InputData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ids: BiMap<u32, _> = BiMap::new();
        let mut next_id = 0;
        let mut rates = Vec::new();
        let mut tunnels = Vec::new();

        macro_rules! new_chamber {
            () => {
                // Added a new chamber, so bump the id, set it's flow rate to zero, and give it an empty set
                // of exits.
                next_id += 1;
                rates.push(0);
                tunnels.push(vec![]);
            };
        }

        for line in s.lines() {
            let vd = line.parse::<ValveDescription>()?;
            if !ids.contains_right(&vd.id) {
                ids.insert(next_id, vd.id.clone());
                new_chamber!();
            }
            let id = *ids.get_by_right(&vd.id).unwrap() as usize;
            rates[id] = vd.rate;
            for tunnel_id in vd.tunnels.iter() {
                if !ids.contains_right(tunnel_id) {
                    ids.insert(next_id, tunnel_id.clone());
                    new_chamber!();
                }
            }
            tunnels[id] = vd
                .tunnels
                .iter()
                .map(|tunnel_id| *ids.get_by_right(tunnel_id).unwrap())
                .collect();
        }

        Ok(InputData { ids, rates, tunnels })
    }
}

impl InputData {
    fn valve_id(&self, letters: &str) -> u32 {
        *self.ids.get_by_right(letters).unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum ValveState {
    Closed,
    Open,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct ValveData(Vec<ValveState>);
impl From<&InputData> for ValveData {
    fn from(data: &InputData) -> Self {
        ValveData(vec![ValveState::Closed; data.ids.len()])
    }
}

struct Chambers {
    // This is the data that stays constant over all the recursive calls to score(); none of this is part of
    // the memoization key.
    tunnels: Vec<Vec<u32>>, // Tunnel graph from initial load (indices are chamber ids)
    rates: Vec<i32>,        // Valve rates from initial load (indices are chamber ids)
    time: i32,              // Time from the problem description (30 or 26, from AoC)
    location: u32,          // Starting location. (The index for chamber AA.)
}

// The core of the solution to Day 16 lives here. The core realization is that the problem statement is _not_
// asking us to find the optimal path. Rather it's asking what would happen if we _had_ taken the optimal
// path. So this devolves into a recursive definition of score. Given a number of minutes left, and a
// location: the maximum achievable score is the max of score(after I do my thing) over (all the things I can
// do). This is a permutation with many possibilities, but even so, that number is within the range of modern
// calculation. The problem is that the recursive descent revisits so many states. The solution to that issue
// is memoization. A cache is maintained to avoid re-working the same problem over and over.
//
// Against my AoC input, the cache grows to 1,006,005 entries for part 1, and 20,380,766 entries for part 2.
// Run times (for me) are about 0.76 seconds for part 1, and 22 seconds for part 2.
//
// A note about the additional player for part 2: This solution runs player 2 _after_ player 1 has completed
// his run entirely (resetting the clock, but not the valves). I don't know why this works. It seems to me
// there should be patterns where the interactions between the players would help each other, but the internet
// is completely convinced that's not the case. Clearly the AoC validator agrees with the Internet. I'd still
// like to see a real proof, though.

fn score(
    data: &Chambers,
    cache: &mut AHashMap<(u32, i32, ValveData, u32), usize>,
    location: u32,
    time_left: i32,
    valves: &ValveData,
    extra_particpants: u32,
) -> usize {
    // What is my maximum achievable score if I start at the valve at the given location, have the given time
    // remaining on the clock, and have the given valve state before making any new decisions?

    // Potential optimizations
    // * I need to clone the ValveData _many_ times. Far too many. It's just a string of booleans. It would be
    //   a lot faster to just make it a bitmask, throw it in a u32 or u64, and thus put it in a register-sized
    //   type that implements Copy.
    // * Most of the work this routine does outside of the recursion is the hash calculation for the cache. It
    //   would be much faster to analyze the true number ranges on the items that make up the cache key, and
    //   just throw it into a u32, and use a vector with O(1) indexing rather than a HashMap for the cache
    //   itself. (The cache filled up to 20 million entries for part 2. That's big, but not too big.)

    if time_left <= 0 {
        // Out of time! If this is the last player, our maximum score from here is zero. If we're not the last
        // player, reset for the next player and return their score.
        if extra_particpants > 0 {
            // Reset time & starting location, but not valve state for the next player
            return score(data, cache, data.location, data.time, valves, extra_particpants - 1);
        }
        return 0;
    }

    // Check the cache. If we have a hit, don't actually do any new work.
    let valves = valves.clone();
    let maybe_score = cache.get(&(location, time_left, valves.clone(), extra_particpants));
    if let Some(&previous_calculation) = maybe_score {
        return previous_calculation;
    }

    let location = location as usize;
    // Run through all the possibilities I have in this chamber: opening a valve (if it's not already open and
    // if it has a positive flow rate); or travelling down one of this chamber's tunnels.
    let mut best_score = 0;
    if valves.0[location] == ValveState::Closed && data.rates[location] > 0 {
        // Make a new "valves" vector with this valve marked open
        let mut new_valves = valves.clone();
        new_valves.0[location] = ValveState::Open;
        // And then try again: our score is now the sum of
        // * this value open for the remaining time
        // * the best score from here given this new valve state
        best_score = best_score.max(
            ((time_left - 1) * data.rates[location]) as usize
                + score(
                    data,
                    cache,
                    location as u32,
                    time_left - 1,
                    &new_valves,
                    extra_particpants,
                ),
        );
    }
    for next_loc in data.tunnels[location].iter() {
        // All the travelling. The best score from here is the best score from the connected location, but
        // with a bit less time
        best_score = best_score.max(score(data, cache, *next_loc, time_left - 1, &valves, extra_particpants));
    }

    // Add our new best score into the cache
    cache.insert((location as u32, time_left, valves, extra_particpants), best_score);
    // And done.
    best_score
}

fn score_part1(data: &InputData) -> usize {
    let initial_valves = ValveData::from(data);
    let mut cache = AHashMap::new();
    let scoring_run_setup = Chambers {
        time: 30,
        location: data.valve_id("AA"),
        rates: data.rates.clone(),
        tunnels: data.tunnels.clone(),
    };
    let result = score(
        &scoring_run_setup,
        &mut cache,
        scoring_run_setup.location,
        scoring_run_setup.time,
        &initial_valves,
        0,
    );

    println!("Cache had {} entries", cache.len());

    result
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let data = input.parse::<InputData>()?;
    Ok(score_part1(&data))
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let data = input.parse::<InputData>()?;
    let initial_valves = ValveData::from(&data);
    let mut cache = AHashMap::new();
    let scoring_run_setup = Chambers {
        time: 26,
        location: data.valve_id("AA"),
        rates: data.rates,
        tunnels: data.tunnels,
    };
    let result = Ok(score(
        &scoring_run_setup,
        &mut cache,
        scoring_run_setup.location,
        scoring_run_setup.time,
        &initial_valves,
        1,
    ));

    println!("Cache had {} entries", cache.len());

    result
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
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 1651);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 1707);
    }
}
