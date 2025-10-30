//! # Solution for Advent of Code 2021 Day 11
//!
//! Ref: [Advent of Code 2021 Day 11](https://adventofcode.com/2021/day/11)
//!

use ahash::{AHashMap, AHashSet};
use std::io::{self, BufRead};

const NEIGHBOR_DELTAS: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

#[derive(Debug)]
struct EnergyMap {
    map: AHashMap<(i32, i32), u32>,
}

impl<S> FromIterator<S> for EnergyMap
where
    S: AsRef<str>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self
    where
        S: AsRef<str>,
    {
        let mut hm: AHashMap<(i32, i32), u32> = Default::default();

        for (row, s) in iter.into_iter().enumerate() {
            let r = row.try_into().unwrap();
            for (column, digit) in s.as_ref().chars().enumerate() {
                let c = column.try_into().unwrap();
                hm.insert((r, c), digit.to_digit(10).unwrap());
            }
        }

        Self { map: hm }
    }
}

impl EnergyMap {
    fn run_step(&mut self) -> u32 {
        let mut num_flashes = 0;
        let mut do_flash = AHashSet::<(i32, i32)>::new();
        // increase all the energy levels...
        for (&loc, val) in self.map.iter_mut() {
            let new_val = *val + 1;
            *val = new_val;
            if new_val > 9 {
                do_flash.insert(loc);
            }
        }
        // everthing >9 flashes.
        let mut flashed = AHashSet::<(i32, i32)>::new();
        while !do_flash.is_empty() {
            let loc = *do_flash.iter().next().unwrap();
            do_flash.remove(&loc);

            num_flashes += 1;
            flashed.insert(loc);
            for neighbor_loc in NEIGHBOR_DELTAS.iter().map(|l| {
                let (delta_row, delta_col) = l;
                let (orig_row, orig_col) = loc;
                (orig_row + delta_row, orig_col + delta_col)
            }) {
                if let Some(val) = self.map.get_mut(&neighbor_loc) {
                    let new_val = *val + 1;
                    *val = new_val;
                    if new_val > 9 && !flashed.contains(&neighbor_loc) {
                        do_flash.insert(neighbor_loc);
                    }
                }
            }
        }

        for loc in flashed.iter() {
            *self.map.get_mut(loc).unwrap() = 0;
        }

        num_flashes
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let lines = stdin.lock().lines().map_while(Result::ok).collect::<Vec<_>>();
    let mut energy_map = lines.iter().collect::<EnergyMap>();

    let flashes = (0..100).map(|_| energy_map.run_step() as u64).sum::<u64>();
    println!("Part 1: {flashes} flashes");

    let mut energy_map = lines.iter().collect::<EnergyMap>();
    let r = (1..)
        .map(|step| (step, energy_map.run_step()))
        .find(|&(_, flashes)| flashes >= 100)
        .unwrap()
        .0;
    println!("Part 2: All flash after step {r}");

    Ok(())
}
